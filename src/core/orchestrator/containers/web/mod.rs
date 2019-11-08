use super::{Containers, Instance};
use crate::{
    core,
    core::{executor::Spawn, Executor},
};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    future::LocalBoxFuture,
    task::Context,
    Poll, Sink, SinkExt, Stream, TryFutureExt,
};
use js_sys::{
    Function, Number, Uint8Array,
    WebAssembly::{compile, instantiate_module, Instance as WasmInstance, Memory, Module},
};
use std::{cell::RefCell, pin::Pin, rc::Rc};
use void::Void;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;

pub struct WebInstance {
    instance: WasmInstance,
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

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn start_send(self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct WebContainers;

struct InstanceState {
    handle: Function,
    make_buffer: Function,
    memory: Memory,
}

trait InstanceHelper {
    fn handle(&self);
    fn make_buffer(&self, size: u32) -> u32;
    fn read(&self, ptr: u32, len: u32) -> Vec<u8>;
}

impl InstanceHelper for Rc<RefCell<Option<InstanceState>>> {
    fn handle(&self) {
        let cell = self.borrow();
        let cell = cell.as_ref().unwrap();
        cell.handle.call0(&cell.handle).unwrap();
    }
    fn make_buffer(&self, size: u32) -> u32 {
        let cell = self.borrow();
        let cell = cell.as_ref().unwrap();
        f64::from(
            cell.handle
                .call1(&cell.handle, &size.into())
                .unwrap()
                .dyn_into::<Number>()
                .unwrap(),
        ) as u32
    }
    fn read(&self, ptr: u32, len: u32) -> Vec<u8> {
        let cell = self.borrow();
        let cell = cell.as_ref().unwrap();
        Uint8Array::new(&cell.memory.buffer())
            .slice(ptr, ptr + len)
            .to_vec()
    }
}

impl Containers for WebContainers {
    type Module = Module;
    type Instance = WebInstance;

    fn compile<T: AsRef<[u8]>>(&mut self, data: T) -> LocalBoxFuture<'static, Self::Module> {
        let data = data.as_ref().to_vec();
        Box::pin(async move {
            let data: Uint8Array = data.as_slice().into();
            JsFuture::from(compile(&data.into()))
                .await
                .unwrap()
                .dyn_into()
                .unwrap()
        })
    }
    fn instantiate(&mut self, module: &Self::Module) -> LocalBoxFuture<'static, Self::Instance> {
        let module = module.clone();
        Box::pin(async move {
            let (sender, receiver) = unbounded();
            let handle: Rc<RefCell<Option<InstanceState>>> = Rc::new(RefCell::new(None));
            let imports = js_sys::Object::new();
            let h = handle.clone();
            let mut executor = core::<dyn Executor>().unwrap();
            let output = Closure::wrap(Box::new(move |ptr: u32, len: u32| {
                let mut sender = sender.clone();
                let data = h.read(ptr, len);
                executor.spawn(async move { sender.send(data).unwrap_or_else(|_| panic!()).await });
            }) as Box<dyn FnMut(_, _)>);
            let h_2 = handle.clone();
            let enqueue = Closure::wrap(Box::new(move || {
                h_2.handle();
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
            handle.replace(Some(InstanceState {
                handle: handle_func,
                make_buffer,
                memory: js_sys::Reflect::get(&instance.exports(), &"memory".into())
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            }));
            initializer.call0(&initializer).unwrap();
            WebInstance {
                instance,
                _output: output,
                _enqueue: enqueue,
                receiver: Box::pin(receiver),
            }
        })
    }
}
