use super::{Containers, Instance};
use crate::{core::spawn, kind::Future};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    lock,
    task::{Context, Poll},
    Sink, SinkExt, Stream,
};
use serde::{
    de::{Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{
    ffi::c_void,
    fmt,
    pin::Pin,
    sync::{Arc, Mutex},
};
use void::Void;
use wasmer_runtime::{
    cache::Artifact, compile, compiler_for_backend, func, imports, memory::MemoryView, wasm::Value,
    Backend, Ctx, Export, Instance as WasmInstance, Memory, Module,
};
use wasmer_runtime_core::load_cache_with;

pub struct NativeContainers;

impl NativeContainers {
    pub fn new() -> Self {
        NativeContainers
    }
}

pub struct NativeInstance {
    instance: Arc<Mutex<WasmInstance>>,
    memory: Memory,
    receiver: Pin<Box<UnboundedReceiver<Vec<u8>>>>,
}

impl NativeInstance {
    fn write(&mut self, data: Vec<u8>) {
        let instance = self.instance.lock().unwrap();
        use Value::I32;
        let len = data.len() as i32;
        if let I32(ptr) = instance.call("_EXPORT_make_buffer", &[I32(len)]).unwrap()[0] {
            let view: MemoryView<u8> = self.memory.view();
            for (idx, byte) in data.into_iter().enumerate() {
                view[ptr as usize + idx].set(byte)
            }
            instance.call("_EXPORT_input", &[I32(ptr)]).unwrap();
        } else {
            panic!("bad write")
        }
    }
}

impl Instance for NativeInstance {}

impl Stream for NativeInstance {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.receiver.as_mut().poll_next(cx)
    }
}

impl Sink<Vec<u8>> for NativeInstance {
    type Error = Void;

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn start_send(mut self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        self.write(item);
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

struct State {
    handle: Box<dyn FnMut() + Sync + Send>,
    output: UnboundedSender<Vec<u8>>,
}

fn enqueue(cx: &mut Ctx) {
    let state = unsafe { Box::from_raw(cx.data as *mut lock::Mutex<State>) };
    spawn(async move {
        {
            let mut st = state.lock().await;
            (&mut *st.handle)();
        }
        Box::leak(state);
    });
}

fn output(cx: &mut Ctx, ptr: i32, len: i32) {
    let mem = cx.memory(0);
    let mut buffer = vec![0u8; len as usize];
    let view: MemoryView<u8> = mem.view();
    let ptr = ptr as usize;
    for (idx, byte) in buffer.iter_mut().enumerate() {
        *byte = view[ptr + idx].get();
    }
    let state = unsafe { Box::from_raw(cx.data as *mut lock::Mutex<State>) };
    spawn(async move {
        state.lock().await.output.send(buffer).await.unwrap();
        Box::leak(state);
    });
}

fn panic(cx: &mut Ctx, ptr: i32, len: i32) {
    let mem = cx.memory(0);
    let mut buffer = vec![0u8; len as usize];
    let view: MemoryView<u8> = mem.view();
    let ptr = ptr as usize;
    for (idx, byte) in buffer.iter_mut().enumerate() {
        *byte = view[ptr + idx].get();
    }
    if let Ok(item) = String::from_utf8(buffer) {
        panic!(item);
    } else {
        panic!();
    }
}

#[derive(Clone)]
pub struct NativeModule(Module);

impl Serialize for NativeModule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0.cache().unwrap().serialize().unwrap().as_slice())
    }
}

pub struct ModuleVisitor;

impl<'de> Visitor<'de> for ModuleVisitor {
    type Value = NativeModule;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a vessels module")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(NativeModule(unsafe {
            load_cache_with(
                Artifact::deserialize(v).unwrap(),
                compiler_for_backend(Backend::default()).unwrap().as_ref(),
            )
            .unwrap()
        }))
    }
}

impl<'de> Deserialize<'de> for NativeModule {
    fn deserialize<D>(deserializer: D) -> Result<NativeModule, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ModuleVisitor)
    }
}

impl Containers for NativeContainers {
    type Module = NativeModule;
    type Compile = Future<NativeModule>;
    type Instance = NativeInstance;
    type Instantiate = Future<NativeInstance>;

    fn compile<T: AsRef<[u8]>>(&self, data: T) -> Self::Compile {
        let data = data.as_ref().to_vec();
        Box::pin(async move { NativeModule(compile(data.as_ref()).unwrap()) })
    }

    fn instantiate(&self, module: &Self::Module) -> Self::Instantiate {
        let module = module.clone();
        Box::pin(async move {
            let import_object = imports! {
                "env" => {
                    "_EXPORT_enqueue" => func!(enqueue),
                    "_EXPORT_output" => func!(output),
                    "_EXPORT_panic" => func!(panic),
                },
            };
            let instance = module.0.instantiate(&import_object).unwrap();
            let instance = Arc::new(Mutex::new(instance));
            let inst = instance.clone();
            let inst_2 = inst.clone();
            let mut instance = instance.lock().unwrap();
            let ctx = instance.context_mut();
            let (sender, receiver) = unbounded();
            let state = lock::Mutex::new(State {
                handle: Box::new(move || {
                    let inst = inst.clone();
                    spawn(async move {
                        inst.lock().unwrap().call("_EXPORT_handle", &[]).unwrap();
                    });
                }),
                output: sender,
            });
            ctx.data = Box::into_raw(Box::new(state)) as *mut c_void;
            ctx.data_finalizer = Some(|ptr| {
                drop(unsafe { Box::from_raw(ptr as *mut lock::Mutex<State>) });
            });
            let ret = if let Export::Memory(memory) = instance
                .exports()
                .find(|(name, _)| name == "memory")
                .unwrap()
                .1
            {
                NativeInstance {
                    instance: inst_2,
                    memory,
                    receiver: Box::pin(receiver),
                }
            } else {
                panic!("no memory in module")
            };
            instance.call("_EXPORT_initialize", &[]).unwrap();
            ret
        })
    }
}
