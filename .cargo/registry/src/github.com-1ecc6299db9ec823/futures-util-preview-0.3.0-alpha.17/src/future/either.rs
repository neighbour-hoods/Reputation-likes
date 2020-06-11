use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::future::{FusedFuture, Future};
use futures_core::stream::{FusedStream, Stream};
#[cfg(feature = "sink")]
use futures_sink::Sink;

/// Combines two different futures, streams, or sinks having the same associated types into a single
/// type.
#[derive(Debug, Clone)]
pub enum Either<A, B> {
    /// First branch of the type
    Left(A),
    /// Second branch of the type
    Right(B),
}

impl<A, B, T> Either<(T, A), (T, B)> {
    /// Factor out a homogeneous type from an either of pairs.
    ///
    /// Here, the homogeneous type is the first element of the pairs.
    pub fn factor_first(self) -> (T, Either<A, B>) {
        match self {
            Either::Left((x, a)) => (x, Either::Left(a)),
            Either::Right((x, b)) => (x, Either::Right(b)),
        }
    }
}

impl<A, B, T> Either<(A, T), (B, T)> {
    /// Factor out a homogeneous type from an either of pairs.
    ///
    /// Here, the homogeneous type is the second element of the pairs.
    pub fn factor_second(self) -> (Either<A, B>, T) {
        match self {
            Either::Left((a, x)) => (Either::Left(a), x),
            Either::Right((b, x)) => (Either::Right(b), x),
        }
    }
}

impl<T> Either<T, T> {
    /// Extract the value of an either over two equivalent types.
    pub fn into_inner(self) -> T {
        match self {
            Either::Left(x) => x,
            Either::Right(x) => x,
        }
    }
}

impl<A, B> Future for Either<A, B>
where
    A: Future,
    B: Future<Output = A::Output>,
{
    type Output = A::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<A::Output> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll(cx),
            }
        }
    }
}

impl<A, B> FusedFuture for Either<A, B>
where
    A: FusedFuture,
    B: FusedFuture,
{
    fn is_terminated(&self) -> bool {
        match self {
            Either::Left(x) => x.is_terminated(),
            Either::Right(x) => x.is_terminated(),
        }
    }
}

impl<A, B> Stream for Either<A, B>
where
    A: Stream,
    B: Stream<Item = A::Item>,
{
    type Item = A::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<A::Item>> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll_next(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll_next(cx),
            }
        }
    }
}

impl<A, B> FusedStream for Either<A, B>
where
    A: FusedStream,
    B: FusedStream,
{
    fn is_terminated(&self) -> bool {
        match self {
            Either::Left(x) => x.is_terminated(),
            Either::Right(x) => x.is_terminated(),
        }
    }
}

#[cfg(feature = "sink")]
impl<A, B, Item> Sink<Item> for Either<A, B>
where
    A: Sink<Item>,
    B: Sink<Item, Error = A::Error>,
{
    type Error = A::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll_ready(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll_ready(cx),
            }
        }
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).start_send(item),
                Either::Right(x) => Pin::new_unchecked(x).start_send(item),
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll_flush(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll_flush(cx),
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(x) => Pin::new_unchecked(x).poll_close(cx),
                Either::Right(x) => Pin::new_unchecked(x).poll_close(cx),
            }
        }
    }
}

#[cfg(feature = "io")]
#[cfg(feature = "std")]
mod if_std {
    use super::Either;
    use core::pin::Pin;
    use core::task::{Context, Poll};
    use futures_io::{
        AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, Initializer, IoSlice, IoSliceMut, Result,
        SeekFrom,
    };

    impl<A, B> AsyncRead for Either<A, B>
    where
        A: AsyncRead,
        B: AsyncRead,
    {
        unsafe fn initializer(&self) -> Initializer {
            match self {
                Either::Left(x) => x.initializer(),
                Either::Right(x) => x.initializer(),
            }
        }

        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_read(cx, buf),
                    Either::Right(x) => Pin::new_unchecked(x).poll_read(cx, buf),
                }
            }
        }

        fn poll_read_vectored(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &mut [IoSliceMut<'_>],
        ) -> Poll<Result<usize>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_read_vectored(cx, bufs),
                    Either::Right(x) => Pin::new_unchecked(x).poll_read_vectored(cx, bufs),
                }
            }
        }
    }

    impl<A, B> AsyncWrite for Either<A, B>
    where
        A: AsyncWrite,
        B: AsyncWrite,
    {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_write(cx, buf),
                    Either::Right(x) => Pin::new_unchecked(x).poll_write(cx, buf),
                }
            }
        }

        fn poll_write_vectored(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[IoSlice<'_>],
        ) -> Poll<Result<usize>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_write_vectored(cx, bufs),
                    Either::Right(x) => Pin::new_unchecked(x).poll_write_vectored(cx, bufs),
                }
            }
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_flush(cx),
                    Either::Right(x) => Pin::new_unchecked(x).poll_flush(cx),
                }
            }
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_close(cx),
                    Either::Right(x) => Pin::new_unchecked(x).poll_close(cx),
                }
            }
        }
    }

    impl<A, B> AsyncSeek for Either<A, B>
    where
        A: AsyncSeek,
        B: AsyncSeek,
    {
        fn poll_seek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            pos: SeekFrom,
        ) -> Poll<Result<u64>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_seek(cx, pos),
                    Either::Right(x) => Pin::new_unchecked(x).poll_seek(cx, pos),
                }
            }
        }
    }

    impl<A, B> AsyncBufRead for Either<A, B>
    where
        A: AsyncBufRead,
        B: AsyncBufRead,
    {
        fn poll_fill_buf<'a>(
            self: Pin<&'a mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<&'a [u8]>> {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).poll_fill_buf(cx),
                    Either::Right(x) => Pin::new_unchecked(x).poll_fill_buf(cx),
                }
            }
        }

        fn consume(self: Pin<&mut Self>, amt: usize) {
            unsafe {
                match self.get_unchecked_mut() {
                    Either::Left(x) => Pin::new_unchecked(x).consume(amt),
                    Either::Right(x) => Pin::new_unchecked(x).consume(amt),
                }
            }
        }
    }
}