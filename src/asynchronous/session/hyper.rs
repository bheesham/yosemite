//! Implementations of traits in `hyper::rt`.
use futures::Future;
use hyper::rt;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{ReadHalf, Stream, WriteHalf};

#[cfg(feature = "tokio")]
use tokio::spawn;

#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

#[cfg(feature = "smol")]
use smol::spawn;

#[cfg(feature = "smol")]
use smol::io::{AsyncRead, AsyncWrite};

impl<F> rt::Executor<F> for Stream
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        let _ = spawn(future);
    }
}

#[cfg(feature = "tokio")]
impl rt::Read for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: rt::ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // SAFETY: `ReadBuf::uninit` accepts `&'a mut [MaybeUninit<u8>]`, and tokio
        // initializes the filled bytes before we advance.
        unsafe {
            let mut hbuf = ReadBuf::uninit(buf.as_mut());
            let result = AsyncRead::poll_read(self, cx, &mut hbuf);
            let filled = hbuf.filled().len();
            buf.advance(filled);
            result
        }
    }
}

// YAGNI?
#[cfg(feature = "tokio")]
impl rt::Read for ReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: rt::ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // SAFETY: `ReadBuf::uninit` accepts `&'a mut [MaybeUninit<u8>]`, and tokio
        // initializes the filled bytes before we advance.
        unsafe {
            let mut hbuf = ReadBuf::uninit(buf.as_mut());
            let result = AsyncRead::poll_read(self, cx, &mut hbuf);
            let filled = hbuf.filled().len();
            buf.advance(filled);
            result
        }
    }
}

#[cfg(feature = "smol")]
impl rt::Read for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: rt::ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // SAFETY: `ReadBuf::uninit` accepts `&'a mut [MaybeUninit<u8>]`.
        unsafe {
            let _ = AsyncRead::poll_read(self, cx, buf.as_mut().assume_init_mut())?;
            Poll::Ready(Ok(()))
        }
    }
}

// YAGNI?
#[cfg(feature = "smol")]
impl rt::Read for ReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: rt::ReadBufCursor<'_>,
    ) -> Poll<io::Result<()>> {
        // SAFETY: `ReadBuf::uninit` accepts `&'a mut [MaybeUninit<u8>]`.
        unsafe {
            let _ = AsyncRead::poll_read(self, cx, buf.as_mut().assume_init_mut())?;
            Poll::Ready(Ok(()))
        }
    }
}

#[cfg(feature = "tokio")]
impl rt::Write for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(self, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(self, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(self, cx)
    }
}

// YAGNI?
#[cfg(feature = "tokio")]
impl rt::Write for WriteHalf {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        AsyncWrite::poll_write(self, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_flush(self, cx)
    }
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        AsyncWrite::poll_shutdown(self, cx)
    }
}

#[cfg(feature = "smol")]
impl rt::Write for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        AsyncWrite::poll_write(self, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_flush(self, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_close(self, cx)
    }
}

// YAGNI?
#[cfg(feature = "smol")]
impl rt::Write for WriteHalf {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        AsyncWrite::poll_write(self, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_flush(self, cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        AsyncWrite::poll_close(self, cx)
    }
}
