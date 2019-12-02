use super::LocalModule;
use crate::core::{data::Checksum, spawn};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    future::LocalBoxFuture,
    lock,
    task::{Context, Poll},
    Future, Sink, SinkExt, Stream, TryFutureExt,
};
use js_sys::{
    Function, Number, Uint8Array,
    WebAssembly::{compile, instantiate_module, Instance as WasmInstance, Memory, Module},
};
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap, pin::Pin, rc::Rc};
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
    _panic: Closure<dyn FnMut(u32, u32)>,
    _enqueue: Closure<dyn FnMut()>,
    receiver: Pin<Box<UnboundedReceiver<Vec<u8>>>>,
}

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

#[derive(Clone)]
pub struct WebContainers;

impl WebContainers {
    pub fn new() -> Self {
        WebContainers
    }
}

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
        Handle(Box::pin(async move {
            handle.call0(&handle).unwrap();
        }))
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
        self.input.call1(&self.input, &ptr.into()).unwrap();
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

pub(crate) struct Compile(LocalBoxFuture<'static, LocalModule>);

impl Future for Compile {
    type Output = LocalModule;

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

lazy_static! {
    static ref TEMP_CACHE: lock::Mutex<HashMap<Checksum, WebModule>> =
        lock::Mutex::new(HashMap::new());
}

impl WebContainers {
    pub(crate) fn compile(
        &self,
        data: Vec<u8>,
    ) -> impl Future<Output = LocalModule> + Sync + Send + 'static {
        Compile(Box::pin(async move {
            let mut cache = TEMP_CACHE.lock().await;
            let sum = Checksum::new(&data).await.unwrap();
            let data: Uint8Array = data.as_slice().into();
            cache.insert(
                sum.clone(),
                WebModule(
                    JsFuture::from(compile(&data.into()))
                        .await
                        .unwrap()
                        .dyn_into()
                        .unwrap(),
                ),
            );
            LocalModule(sum)
        }))
    }
    pub(crate) fn instantiate(
        &self,
        module: &LocalModule,
    ) -> impl Future<Output = WebInstance> + Sync + Send + 'static {
        let module = module.0.clone();
        Instantiate(Box::pin(async move {
            let module = TEMP_CACHE.lock().await.get(&module).unwrap().0.clone();
            let (sender, receiver) = unbounded();
            let handle: Rc<RefCell<Option<InstanceStateRead>>> = Rc::new(RefCell::new(None));
            let imports = js_sys::Object::new();
            let h = handle.clone();
            let output = Closure::wrap(Box::new(move |ptr: u32, len: u32| {
                let mut sender = sender.clone();
                let data = h.read(ptr, len);
                spawn(async move { sender.send(data).unwrap_or_else(|_| panic!()).await });
            }) as Box<dyn FnMut(_, _)>);
            let h_3 = handle.clone();
            let panic = Closure::wrap(Box::new(move |ptr: u32, len: u32| {
                let data = h_3.read(ptr, len);
                if let Some(item) = String::from_utf8(data).ok() {
                    panic!(item);
                } else {
                    panic!();
                }
            }) as Box<dyn FnMut(_, _)>);
            let h_2 = handle.clone();
            let enqueue = Closure::wrap(Box::new(move || {
                spawn(h_2.handle());
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
                    js_sys::Reflect::set(
                        &env,
                        &"_EXPORT_panic".into(),
                        panic.as_ref().unchecked_ref(),
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
                _panic: panic,
                _enqueue: enqueue,
                receiver: Box::pin(receiver),
            }
        }))
    }
}
