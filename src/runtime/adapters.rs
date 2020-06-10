use super::{
    FramedTransportCoalesce, FramedTransportUnravel, RawTransportCoalesce, RawTransportUnravel,
};
use bitbuf::{BitBuf, BitSlice};
use bitbuf_vlq::{AsyncReadVlq, Vlq};
use core::{
    cmp::min,
    marker::PhantomData,
    mem::replace,
    pin::Pin,
    task::{Context, Poll},
};
use core_error::Error;
use core_futures_io::{AsyncRead, AsyncWrite, FuturesCompat};
use futures::{io::Cursor, ready, Sink, Stream, TryStream};
use std::io::{Cursor as SyncCursor, Write};
use thiserror::Error;

pub struct FrameAdapterStream<T: TryStream<Ok = Vec<u8>>> {
    stream: T,
    vlq: Cursor<VlqWrapper>,
    buffer: Cursor<Vec<u8>>,
}

impl<T: TryStream<Ok = Vec<u8>>> FrameAdapterStream<T> {
    pub fn new(stream: T) -> Self {
        FrameAdapterStream {
            stream,
            vlq: Cursor::new(VlqWrapper(Vlq::from(0u64))),
            buffer: Cursor::new(vec![]),
        }
    }
}

pub struct VlqWrapper(Vlq);

impl AsRef<[u8]> for VlqWrapper {
    fn as_ref(&self) -> &[u8] {
        &*(self.0)
    }
}

impl<T: Unpin + TryStream<Ok = Vec<u8>>> AsyncRead for FrameAdapterStream<T> {
    type Error = T::Error;

    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buffer: &mut [u8],
    ) -> Poll<Result<usize, Self::Error>> {
        let this = &mut *self;

        loop {
            if this.buffer.get_ref().is_empty() {
                let buffer = ready!(Pin::new(&mut this.stream).try_poll_next(cx));
                if let Some(buffer) = buffer {
                    let buffer = buffer?;
                    let len = buffer.len() as u64;
                    this.buffer = Cursor::new(buffer);
                    this.vlq = Cursor::new(VlqWrapper(Vlq::from(len)));
                } else {
                    return Poll::Ready(Ok(0));
                }
            } else {
                let vlq_read =
                    ready!(Pin::new(&mut FuturesCompat::new(&mut this.vlq)).poll_read(cx, buffer))
                        .unwrap();

                if vlq_read == 0 {
                    match ready!(
                        Pin::new(&mut FuturesCompat::new(&mut this.buffer)).poll_read(cx, buffer)
                    )
                    .unwrap()
                    {
                        0 => {
                            this.buffer = Cursor::new(vec![]);
                        }
                        n => {
                            return Poll::Ready(Ok(n));
                        }
                    }
                } else {
                    return Poll::Ready(Ok(vlq_read));
                }
            }
        }
    }
}

enum FrameAdapterSinkState {
    ReadingVlq(AsyncReadVlq),
    ReadingData(SyncCursor<Vec<u8>>),
    WritingData(Vec<u8>),
}

pub struct FrameAdapterSink<T: Sink<Vec<u8>>> {
    sink: T,
    flushed: bool,
    state: FrameAdapterSinkState,
}

impl<T: Sink<Vec<u8>>> FrameAdapterSink<T> {
    pub fn new(sink: T) -> Self {
        FrameAdapterSink {
            sink,
            flushed: true,
            state: FrameAdapterSinkState::ReadingVlq(Vlq::async_read()),
        }
    }
}

impl<T: Unpin + Sink<Vec<u8>>> AsyncWrite for FrameAdapterSink<T> {
    type WriteError = T::Error;
    type FlushError = T::Error;
    type CloseError = T::Error;

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::WriteError>> {
        use FrameAdapterSinkState::{ReadingData, ReadingVlq, WritingData};

        let this = &mut *self;

        loop {
            match &mut this.state {
                ReadingVlq(vlq) => {
                    let mut bitbuf = BitSlice::new(&*buf);
                    let read = vlq.poll_read(&mut bitbuf);
                    if let Ok(len) = read {
                        this.state = FrameAdapterSinkState::ReadingData(SyncCursor::new(
                            Some(0).into_iter().cycle().take(len as usize).collect(),
                        ));
                    }
                    return Poll::Ready(Ok(bitbuf.len() / 8));
                }
                ReadingData(data) => {
                    let target = data.get_ref().len();
                    let pos = data.position() as usize;

                    if pos < target {
                        let read_len = min(target - pos, buf.len());
                        data.write(&buf[..read_len]).unwrap();
                        return Poll::Ready(Ok(read_len));
                    } else {
                        let state = replace(&mut this.state, ReadingVlq(Vlq::async_read()));
                        if let ReadingData(data) = state {
                            this.state = WritingData(data.into_inner());
                        } else {
                            panic!("invalid state")
                        }
                    }
                }
                WritingData(_) => {
                    let state = replace(&mut this.state, ReadingVlq(Vlq::async_read()));
                    let data = if let WritingData(data) = state {
                        data
                    } else {
                        panic!("invalid state")
                    };
                    let mut sink = Pin::new(&mut this.sink);
                    match sink.as_mut().poll_ready(cx) {
                        Poll::Ready(it) => {
                            it?;
                        }
                        Poll::Pending => {
                            this.state = WritingData(data);
                            return Poll::Pending;
                        }
                    }
                    sink.start_send(data)?;
                    this.flushed = false;
                }
            }
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Result<(), Self::FlushError>> {
        use FrameAdapterSinkState::{ReadingData, ReadingVlq, WritingData};

        let this = &mut *self;

        loop {
            if !this.flushed {
                ready!(Pin::new(&mut this.sink).poll_flush(cx))?;
                this.flushed = true;
            }

            let state = replace(&mut this.state, ReadingVlq(Vlq::async_read()));

            match state {
                WritingData(data) => {
                    let mut sink = Pin::new(&mut this.sink);
                    match sink.as_mut().poll_ready(cx) {
                        Poll::Ready(it) => {
                            it?;
                        }
                        Poll::Pending => {
                            this.state = WritingData(data);
                            return Poll::Pending;
                        }
                    }
                    sink.start_send(data)?;
                    this.flushed = false;
                }
                ReadingData(data) => {
                    if data.get_ref().len() == data.position() as usize {
                        this.state = WritingData(data.into_inner());
                    } else {
                        this.state = ReadingVlq(Vlq::async_read());
                        return Poll::Ready(Ok(()));
                    }
                }
                ReadingVlq(data) => {
                    this.state = ReadingVlq(data);
                    return Poll::Ready(Ok(()));
                }
            }
        }
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Result<(), Self::CloseError>> {
        ready!(self.as_mut().poll_flush(cx))?;

        return Pin::new(&mut self.sink).poll_close(cx);
    }
}

pub struct FrameAdapter<T, U: Unpin + TryStream<Ok = Vec<u8>>, V: Unpin + Sink<Vec<u8>>, S, W> {
    marker: PhantomData<(W, T, U, V, S)>,
}

impl<
        T,
        U: Unpin + TryStream<Ok = Vec<u8>>,
        V: Unpin + Sink<Vec<u8>>,
        S,
        W: RawTransportCoalesce<T, FrameAdapterStream<U>, FrameAdapterSink<V>, S>,
    > FramedTransportCoalesce<T, U, V, S> for FrameAdapter<T, U, V, S, W>
{
    type Coalesce = W::Coalesce;

    fn coalesce(stream: U, sink: V, spawner: S) -> Self::Coalesce {
        W::coalesce(
            FrameAdapterStream::new(stream),
            FrameAdapterSink::new(sink),
            spawner,
        )
    }
}

impl<
        T,
        U: Unpin + TryStream<Ok = Vec<u8>>,
        V: Unpin + Sink<Vec<u8>>,
        S,
        W: RawTransportUnravel<T, FrameAdapterStream<U>, FrameAdapterSink<V>, S>,
    > FramedTransportUnravel<T, U, V, S> for FrameAdapter<T, U, V, S, W>
{
    type Unravel = W::Unravel;

    fn unravel(item: T, stream: U, sink: V, spawner: S) -> Self::Unravel {
        W::unravel(
            item,
            FrameAdapterStream::new(stream),
            FrameAdapterSink::new(sink),
            spawner,
        )
    }
}

enum RawAdapterReaderState {
    ReadingVlq(AsyncReadVlq),
    ReadingData(SyncCursor<Vec<u8>>),
}

pub struct RawAdapterReader<T: AsyncRead> {
    reader: T,
    state: RawAdapterReaderState,
}

impl<T: AsyncRead> RawAdapterReader<T> {
    pub fn new(reader: T) -> Self {
        RawAdapterReader {
            reader,
            state: RawAdapterReaderState::ReadingVlq(Vlq::async_read()),
        }
    }
}

impl<T: Unpin + AsyncRead> Stream for RawAdapterReader<T> {
    type Item = Result<Vec<u8>, T::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        use RawAdapterReaderState::{ReadingData, ReadingVlq};

        let this = &mut *self;

        loop {
            match &mut this.state {
                ReadingVlq(vlq) => {
                    let buf = &mut [0u8];
                    if ready!(Pin::new(&mut this.reader).poll_read(cx, &mut *buf))? < 1 {
                        return Poll::Ready(None);
                    }
                    if let Ok(len) = vlq.poll_read(BitSlice::new(buf)) {
                        this.state = ReadingData(SyncCursor::new(
                            Some(0u8).into_iter().cycle().take(len as usize).collect(),
                        ));
                    }
                }
                ReadingData(cursor) => {
                    let pos = cursor.position() as usize;
                    let target = cursor.get_ref().len();

                    if pos == target {
                        let state = replace(&mut this.state, ReadingVlq(Vlq::async_read()));
                        if let ReadingData(cursor) = state {
                            return Poll::Ready(Some(Ok(cursor.into_inner())));
                        } else {
                            panic!("invalid state")
                        }
                    }

                    let buf = &mut cursor.get_mut()[pos..target - pos];
                    let read = ready!(Pin::new(&mut this.reader).poll_read(cx, &mut *buf))?;
                    cursor.set_position((pos + read) as u64);
                }
            }
        }
    }
}

#[derive(Debug, Error)]
#[bounds(where
        T: Error + 'static,
        U: Error + 'static,
        V: Error + 'static
    )]
pub enum RawAdapterWriterError<T, U, V> {
    #[error("{0}")]
    Write(#[source] T),
    #[error("{0}")]
    Flush(#[source] U),
    #[error("{0}")]
    Close(#[source] V),
    #[error("attempted to send an item before the sink is ready")]
    NotReady,
}

pub struct RawAdapterWriter<T: AsyncWrite> {
    writer: T,
    seek: usize,
    vlq: Option<(Vlq, usize)>,
    buffer: Option<Vec<u8>>,
}

impl<T: AsyncWrite> RawAdapterWriter<T> {
    pub fn new(writer: T) -> Self {
        Self {
            writer,
            seek: 0,
            vlq: None,
            buffer: None,
        }
    }
}

impl<T: AsyncWrite + Unpin> Sink<Vec<u8>> for RawAdapterWriter<T> {
    type Error = RawAdapterWriterError<T::WriteError, T::FlushError, T::CloseError>;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        use RawAdapterWriterError::Write;
        let this = &mut *self;

        loop {
            if let Some(buffer) = &this.buffer {
                if let Some((vlq, idx)) = &mut this.vlq {
                    if *idx == (&*vlq).len() {
                        this.vlq.take();
                    } else {
                        *idx += ready!(Pin::new(&mut this.writer).poll_write(cx, &vlq[*idx..]))
                            .map_err(Write)?
                    }
                }

                if this.seek == buffer.len() {
                    this.buffer.take();
                    return Poll::Ready(Ok(()));
                }

                this.seek +=
                    ready!(Pin::new(&mut this.writer).poll_write(cx, &buffer[this.seek..]))
                        .map_err(Write)?;
            } else {
                return Poll::Ready(Ok(()));
            }
        }
    }

    fn start_send(mut self: Pin<&mut Self>, buffer: Vec<u8>) -> Result<(), Self::Error> {
        if self.buffer.is_some() {
            return Err(RawAdapterWriterError::NotReady);
        }
        self.vlq = Some((Vlq::from(buffer.len() as u64), 0));
        self.buffer = Some(buffer);
        self.seek = 0;
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        use RawAdapterWriterError::Flush;

        ready!(self.as_mut().poll_ready(cx))?;

        Pin::new(&mut self.writer).poll_flush(cx).map_err(Flush)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        use RawAdapterWriterError::Close;

        ready!(self.as_mut().poll_flush(cx))?;

        Pin::new(&mut self.writer).poll_close(cx).map_err(Close)
    }
}

pub struct RawAdapter<T, U: Unpin + AsyncRead, V: Unpin + AsyncWrite, S, W> {
    marker: PhantomData<(W, T, U, V, S)>,
}

impl<
        T,
        U: Unpin + AsyncRead,
        V: Unpin + AsyncWrite,
        S,
        W: FramedTransportCoalesce<T, RawAdapterReader<U>, RawAdapterWriter<V>, S>,
    > RawTransportCoalesce<T, U, V, S> for RawAdapter<T, U, V, S, W>
{
    type Coalesce = W::Coalesce;

    fn coalesce(stream: U, sink: V, spawner: S) -> Self::Coalesce {
        W::coalesce(
            RawAdapterReader::new(stream),
            RawAdapterWriter::new(sink),
            spawner,
        )
    }
}

impl<
        T,
        U: Unpin + AsyncRead,
        V: Unpin + AsyncWrite,
        S,
        W: FramedTransportUnravel<T, RawAdapterReader<U>, RawAdapterWriter<V>, S>,
    > RawTransportUnravel<T, U, V, S> for RawAdapter<T, U, V, S, W>
{
    type Unravel = W::Unravel;

    fn unravel(item: T, stream: U, sink: V, spawner: S) -> Self::Unravel {
        W::unravel(
            item,
            RawAdapterReader::new(stream),
            RawAdapterWriter::new(sink),
            spawner,
        )
    }
}
