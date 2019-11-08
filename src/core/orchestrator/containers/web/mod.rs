use super::{Containers, Instance};
use crate::{
    core,
    core::{executor::Spawn, Executor},
};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    future::LocalBoxFuture,
    task::Context,
    Future, Poll, Sink, SinkExt, Stream, TryFutureExt,
};
use js_sys::{
    Function, Number, Uint8Array,
    WebAssembly::{compile, instantiate_module, Instance as WasmInstance, Memory, Module},
};
use std::{cell::RefCell, pin::Pin, rc::Rc};
use void::Void;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;

#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for WebInstance {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for WebInstance {}

pub struct WebInstance {
    state: InstanceStateWrite,
    _output: Closure<dyn FnMut(u32, u32)>,
    _enqueue: Closure<dyn FnMut()>,
    receiver: Pin<Box<UnboundedReceiver<Vec<u8>>>>,
}

impl Instance for WebInstance {}

impl Stream for WebInstance {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.receiver.as_mut().poll_next(cx)
    }
}

impl Sink<Vec<u8>> for WebInstance {
    type Error = Void;

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn start_send(self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        (*self.as_ref()).state.write(item);
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct WebContainers;

struct InstanceStateRead {
    handle: Function,
    memory: Memory,
}

struct InstanceStateWrite {
    make_buffer: Function,
    memory: Memory,
    input: Function,
}

trait InstanceHelper {
    fn handle(&self) -> Handle;
    fn read(&self, ptr: u32, len: u32) -> Vec<u8>;
}

#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for Handle {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for Handle {}

pub struct Handle(LocalBoxFuture<'static, ()>);

impl Future for Handle {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

impl InstanceStateRead {
    fn handle(&self) -> Handle {
        let handle = self.handle.clone();
        Handle(Box::pin(async move { handle.call0(&handle).unwrap(); }))
    }
    fn read(&self, ptr: u32, len: u32) -> Vec<u8> {
        Uint8Array::new(&self.memory.buffer())
            .slice(ptr, ptr + len)
            .to_vec()
    }
}

impl InstanceStateWrite {
    fn make_buffer(&self, size: u32) -> u32 {
        f64::from(
            self.make_buffer
                .call1(&self.make_buffer, &size.into())
                .unwrap()
                .dyn_into::<Number>()
                .unwrap(),
        ) as u32
    }
    fn write(&self, data: Vec<u8>) {
        let ptr = self.make_buffer(data.len() as u32);
        Uint8Array::new(&self.memory.buffer()).set(&Uint8Array::from(data.as_slice()), ptr);
        self.input.call0(&self.input).unwrap();
    }
}

impl InstanceHelper for Rc<RefCell<Option<InstanceStateRead>>> {
    fn handle(&self) -> Handle {
        let cell = self.borrow();
        let cell = cell.as_ref().unwrap();
        cell.handle()
    }
    fn read(&self, ptr: u32, len: u32) -> Vec<u8> {
        let cell = self.borrow();
        let cell = cell.as_ref().unwrap();
        cell.read(ptr, len)
    }
}

#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for WebModule {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for WebModule {}

pub struct WebModule(Module);

#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for Compile {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for Compile {}

pub struct Compile(LocalBoxFuture<'static, WebModule>);

impl Future for Compile {
    type Output = WebModule;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

#[cfg(not(target_feature = "atomics"))]
unsafe impl Send for Instantiate {}
#[cfg(not(target_feature = "atomics"))]
unsafe impl Sync for Instantiate {}

pub struct Instantiate(LocalBoxFuture<'static, WebInstance>);

impl Future for Instantiate {
    type Output = WebInstance;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

impl Containers for WebContainers {
    type Module = WebModule;
    type Compile = Compile;
    type Instance = WebInstance;
    type Instantiate = Instantiate;

    fn compile<T: AsRef<[u8]>>(&mut self, data: T) -> Compile {
        let data = data.as_ref().to_vec();
        Compile(Box::pin(async move {
            let data: Uint8Array = data.as_slice().into();
            WebModule(
                JsFuture::from(compile(&data.into()))
                    .await
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            )
        }))
    }
    fn instantiate(&mut self, module: &Self::Module) -> Instantiate {
        let module = module.0.clone();
        Instantiate(Box::pin(async move {
            let (sender, receiver) = unbounded();
            let handle: Rc<RefCell<Option<InstanceStateRead>>> = Rc::new(RefCell::new(None));
            let imports = js_sys::Object::new();
            let h = handle.clone();
            let mut executor = core::<dyn Executor>().unwrap();
            let output = Closure::wrap(Box::new(move |ptr: u32, len: u32| {
                let mut sender = sender.clone();
                let data = h.read(ptr, len);
                executor.spawn(async move { sender.send(data).unwrap_or_else(|_| panic!()).await });
            }) as Box<dyn FnMut(_, _)>);
            let h_2 = handle.clone();
            let mut handle_executor = core::<dyn Executor>().unwrap();
            let enqueue = Closure::wrap(Box::new(move || {
                handle_executor.spawn(h_2.handle());
            }) as Box<dyn FnMut()>);
            js_sys::Reflect::set(
                &imports,
                &"env".into(),
                &{
                    let env = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &env,
                        &"_EXPORT_output".into(),
                        output.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &env,
                        &"_EXPORT_enqueue".into(),
                        enqueue.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                    env
                }
                .into(),
            )
            .unwrap();
            let instance: WasmInstance = JsFuture::from(instantiate_module(&module, &imports))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();
            let initializer =
                js_sys::Reflect::get(&instance.exports(), &"_EXPORT_initialize".into())
                    .unwrap()
                    .dyn_into::<Function>()
                    .unwrap();
            let handle_func = js_sys::Reflect::get(&instance.exports(), &"_EXPORT_handle".into())
                .unwrap()
                .dyn_into::<Function>()
                .unwrap();
            let make_buffer =
                js_sys::Reflect::get(&instance.exports(), &"_EXPORT_make_buffer".into())
                    .unwrap()
                    .dyn_into::<Function>()
                    .unwrap();
            let input = js_sys::Reflect::get(&instance.exports(), &"_EXPORT_input".into())
                .unwrap()
                .dyn_into::<Function>()
                .unwrap();
            let memory: Memory = js_sys::Reflect::get(&instance.exports(), &"memory".into())
                .unwrap()
                .dyn_into()
                .unwrap();
            let read = InstanceStateRead {
                handle: handle_func,
                memory: memory.clone(),
            };
            let write = InstanceStateWrite {
                input,
                memory,
                make_buffer,
            };
            handle.replace(Some(read));
            initializer.call0(&initializer).unwrap();
            WebInstance {
                state: write,
                _output: output,
                _enqueue: enqueue,
                receiver: Box::pin(receiver),
            }
        }))
    }
}
