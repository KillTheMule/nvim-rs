//! This module contains compatibility wrappers and other adapter thingies to
//! bridge between various async libraries. The code was taken from
//! <https://github.com/tokio-rs/tokio-compat/pull/2> and a bit adjusted. Thanks
//! to Eliza Weisman for providing it. The goal would be to remove it as soon at
//! tokio-compat it released with this functionality.
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures;
use pin_project::pin_project;

/// A compatibility layer that allows conversion between the
/// `tokio::io` and `futures-io` `AsyncRead` and `AsyncWrite` traits.
#[pin_project]
#[derive(Copy, Clone, Debug)]
pub struct Compat<T> {
    #[pin]
    inner: T,
}

/// Extension trait that allows converting a type implementing
/// `tokio::io::AsyncRead` to implement `futures_io::AsyncRead`.
pub trait TokioAsyncReadCompatExt: tokio::io::AsyncRead {
    /// Wraps `self` with a compatibility layer that implements
    /// `futures_io::AsyncRead`.
    fn compat_read(self) -> Compat<Self>
    where
        Self: Sized,
    {
        Compat::new(self)
    }
}

impl<T: tokio::io::AsyncRead> TokioAsyncReadCompatExt for T {}

/// Extension trait that allows converting a type implementing
/// `tokio::io::AsyncWrite` to implement `futures_io::AsyncWrite`.
pub trait TokioAsyncWriteCompatExt: tokio::io::AsyncWrite {
    /// Wraps `self` with a compatibility layer that implements
    /// `futures_io::AsyncWrite`.
    fn compat_write(self) -> Compat<Self>
    where
        Self: Sized,
    {
        Compat::new(self)
    }
}

impl<T: tokio::io::AsyncWrite> TokioAsyncWriteCompatExt for T {}

// === impl Compat ===

impl<T> Compat<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> futures::io::AsyncRead for Compat<T>
where
    T: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        tokio::io::AsyncRead::poll_read(self.project().inner, cx, buf)
    }
}

impl<T> futures::io::AsyncBufRead for Compat<T>
where
    T: tokio::io::AsyncBufRead,
{
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&'a [u8]>> {
        tokio::io::AsyncBufRead::poll_fill_buf(self.project().inner, cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        tokio::io::AsyncBufRead::consume(self.project().inner, amt)
    }
}

impl<T> futures::io::AsyncWrite for Compat<T>
where
    T: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        tokio::io::AsyncWrite::poll_write(self.project().inner, cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(self.project().inner, cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(self.project().inner, cx)
    }
}
