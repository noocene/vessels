use crate::{
    acquire,
    resource::{ErasedResourceManager, ResourceError, ResourceManagerExt},
    Convert, CoreError, Resource, Sha256,
};
use core::{
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use core_error::Error as StdError;
use core_futures_io::{AsyncRead, AsyncWrite};
use futures::{
    task::{waker as make_waker, ArcWake, AtomicWaker},
    Future,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use thiserror::Error;
use wasmer_runtime::{
    error::Error as WasmError, func, imports, instantiate, Func, Instance, Memory,
};

#[derive(Debug, Error)]
#[bounds(where T: StdError + 'static)]
pub enum RuntimeError<T> {
    #[error("failed to acquire binary")]
    NoBinary,
    #[error("no active resource manager")]
    NoResourceManager,
    #[error("core error: {0}")]
    Core(#[source] CoreError),
    #[error("runtime error: {0}")]
    Runtime(#[source] T),
    #[error("resource error: {0}")]
    Resource(#[source] ResourceError<Infallible>),
}

impl<T> From<CoreError> for RuntimeError<T> {
    fn from(input: CoreError) -> Self {
        RuntimeError::Core(input)
    }
}

impl<T> From<ResourceError<Infallible>> for RuntimeError<T> {
    fn from(input: ResourceError<Infallible>) -> Self {
        RuntimeError::Resource(input)
    }
}

pub trait Runtime<T: AsyncWrite, U: AsyncRead> {
    type Instance: Future<Output = Result<(), RuntimeError<Self::Error>>>;
    type Error;

    fn instantiate(&mut self, module: WasmResource, writer: T, reader: U) -> Self::Instance;
}

pub type WasmResource = Resource<Wasm, Convert, Sha256>;

pub struct Wasm(pub Vec<u8>);

impl From<Vec<u8>> for Wasm {
    fn from(binary: Vec<u8>) -> Self {
        Wasm(binary)
    }
}

impl From<Wasm> for Vec<u8> {
    fn from(module: Wasm) -> Vec<u8> {
        module.0
    }
}

pub struct Module<T> {
    pub binary: Vec<u8>,
    ty: PhantomData<T>,
}

impl<T> From<Vec<u8>> for Module<T> {
    fn from(binary: Vec<u8>) -> Self {
        Module {
            binary,
            ty: PhantomData,
        }
    }
}

impl<T> From<Module<T>> for Vec<u8> {
    fn from(module: Module<T>) -> Vec<u8> {
        module.binary
    }
}

pub struct WasmerRuntime;

pub struct WasmerFuture {
    instance: Instance,
    waker: Arc<AtomicWaker>,
    reader_wakeup: Arc<AtomicBool>,
    writer_write_wakeup: Arc<AtomicBool>,
    writer_flush_wakeup: Arc<AtomicBool>,
    writer_close_wakeup: Arc<AtomicBool>,
}

impl Future for WasmerFuture {
    type Output = Result<(), RuntimeError<WasmError>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self
            .reader_wakeup
            .compare_and_swap(true, false, Ordering::SeqCst)
        {
            self.instance
                .exports
                .get::<Func<(), ()>>("_vessel_wake_reader")
                .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
                .call()
                .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?;
        }

        if self
            .writer_write_wakeup
            .compare_and_swap(true, false, Ordering::SeqCst)
        {
            self.instance
                .exports
                .get::<Func<(), ()>>("_vessel_wake_writer_write")
                .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
                .call()
                .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?;
        }

        if self
            .writer_flush_wakeup
            .compare_and_swap(true, false, Ordering::SeqCst)
        {
            self.instance
                .exports
                .get::<Func<(), ()>>("_vessel_wake_writer_flush")
                .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
                .call()
                .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?;
        }

        if self
            .writer_close_wakeup
            .compare_and_swap(true, false, Ordering::SeqCst)
        {
            self.instance
                .exports
                .get::<Func<(), ()>>("_vessel_wake_writer_close")
                .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
                .call()
                .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?;
        }

        if self
            .instance
            .exports
            .get::<Func<(), i32>>("_vessel_poll")
            .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
            .call()
            .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?
            != 0
        {
            self.waker.register(cx.waker());

            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

pub struct VesselWaker {
    wakeup: Arc<AtomicBool>,
    waker: Arc<AtomicWaker>,
}

impl ArcWake for VesselWaker {
    fn wake_by_ref(arc: &Arc<Self>) {
        arc.wakeup.store(true, Ordering::SeqCst);
        arc.waker.wake();
    }
}

struct VesselContext<T, U> {
    reader: U,
    writer: T,
    memory: Option<Memory>,
}

impl<T: 'static + Send + Unpin + AsyncWrite, U: 'static + Send + Unpin + AsyncRead> Runtime<T, U>
    for WasmerRuntime
{
    type Instance = Pin<Box<dyn Future<Output = Result<(), RuntimeError<Self::Error>>> + Send>>;
    type Error = WasmError;

    fn instantiate(&mut self, module: WasmResource, writer: T, reader: U) -> Self::Instance {
        Box::pin(async move {
            let reader_wakeup = Arc::new(AtomicBool::new(false));
            let writer_write_wakeup = Arc::new(AtomicBool::new(false));
            let writer_flush_wakeup = Arc::new(AtomicBool::new(false));
            let writer_close_wakeup = Arc::new(AtomicBool::new(false));

            let manager: ErasedResourceManager =
                acquire().await?.ok_or(RuntimeError::NoResourceManager)?;

            let fetch = manager.fetch(module);
            let data = fetch.await?.ok_or(RuntimeError::NoBinary)?.0;

            let waker = Arc::new(AtomicWaker::new());

            let waker_handle = waker.clone();

            let reader_waker = make_waker(Arc::new(VesselWaker {
                wakeup: reader_wakeup.clone(),
                waker: waker.clone(),
            }));

            let writer_write_waker = make_waker(Arc::new(VesselWaker {
                wakeup: writer_write_wakeup.clone(),
                waker: waker.clone(),
            }));

            let writer_flush_waker = make_waker(Arc::new(VesselWaker {
                wakeup: writer_write_wakeup.clone(),
                waker: waker.clone(),
            }));

            let writer_close_waker = make_waker(Arc::new(VesselWaker {
                wakeup: writer_write_wakeup.clone(),
                waker: waker.clone(),
            }));

            let context = Arc::new(Mutex::new(VesselContext {
                reader,
                writer,
                memory: None,
            }));

            let reader_context_handle = context.clone();
            let writer_write_context_handle = context.clone();
            let writer_flush_context_handle = context.clone();
            let writer_close_context_handle = context.clone();

            let instance = instantiate(
                &data,
                &imports! {
                    "env" => {
                        "_vessel_wake" => func!(move || {
                            waker_handle.wake();
                        }),
                        "_vessel_poll_read" => func!(move |ptr: i32, len: i32| -> i32 {
                            let context = &mut *(reader_context_handle.lock().unwrap());

                            let mut buffer = vec![0u8; len as usize];

                            match Pin::new(&mut context.reader).poll_read(&mut Context::from_waker(&reader_waker), &mut buffer) {
                                Poll::Pending => 0,
                                Poll::Ready(Ok(len)) => {
                                    let view = context.memory.as_ref().unwrap().view::<u8>();

                                    buffer[..len].iter().zip(&view[ptr as usize..]).for_each(|(byte, cell)| {
                                        cell.set(*byte)
                                    });

                                    (len + 1) as i32
                                }
                                _ => panic!()
                            }
                        }),
                        "_vessel_poll_write" => func!(move |ptr: i32, len: i32| -> i32 {
                            let (len, ptr) = (len as usize, ptr as usize);

                            let context = &mut *(writer_write_context_handle.lock().unwrap());

                            let mut buffer = Vec::with_capacity(len);

                            let view = context.memory.as_ref().unwrap().view::<u8>();

                            for byte in &view[ptr..ptr + len] {
                                buffer.push(byte.get())
                            }

                            match Pin::new(&mut context.writer).poll_write(&mut Context::from_waker(&writer_write_waker), &buffer) {
                                Poll::Pending => 0,
                                Poll::Ready(Ok(len)) => {
                                    (len + 1) as i32
                                }
                                _ => panic!()
                            }
                        }),
                        "_vessel_poll_flush" => func!(move || -> i32 {
                            let context = &mut *(writer_flush_context_handle.lock().unwrap());

                            match Pin::new(&mut context.writer).poll_flush(&mut Context::from_waker(&writer_flush_waker)) {
                                Poll::Pending => 0,
                                Poll::Ready(Ok(())) => 1,
                                _ => panic!()
                            }
                        }),
                        "_vessel_poll_close" => func!(move || -> i32 {
                            let context = &mut *(writer_close_context_handle.lock().unwrap());

                            match Pin::new(&mut context.writer).poll_close(&mut Context::from_waker(&writer_close_waker)) {
                                Poll::Pending => 0,
                                Poll::Ready(Ok(())) => 1,
                                _ => panic!()
                            }
                        }),
                    }
                },
            )
            .map_err(RuntimeError::Runtime)?;

            context.lock().unwrap().memory = Some(
                instance
                    .exports
                    .get::<Memory>("memory")
                    .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?,
            );

            instance
                .exports
                .get::<Func<(), ()>>("_vessel_entry")
                .map_err(|e| RuntimeError::Runtime(WasmError::ResolveError(e)))?
                .call()
                .map_err(|e| RuntimeError::Runtime(WasmError::RuntimeError(e)))?;

            WasmerFuture {
                instance,
                waker,
                reader_wakeup,
                writer_write_wakeup,
                writer_flush_wakeup,
                writer_close_wakeup,
            }
            .await
        })
    }
}
