use crate::io::{AsyncRead, ReadBuf};

use std::future::Future;
use std::io;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A future which can be used to easily read exactly enough bytes to fill
/// a buffer.
///
/// Created by the [`AsyncReadExt::read_exact`][read_exact].
/// [read_exact]: [crate::io::AsyncReadExt::read_exact]
pub(crate) fn read_exact<'a, A>(reader: &'a mut A, buf: &'a mut [u8]) -> ReadExact<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    ReadExact {
        reader,
        buf: ReadBuf::new(buf),
    }
}

cfg_io_util! {
    /// Creates a future which will read exactly enough bytes to fill `buf`,
    /// returning an error if EOF is hit sooner.
    ///
    /// On success the number of bytes is returned
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadExact<'a, A: ?Sized> {
        reader: &'a mut A,
        buf: ReadBuf<'a>,
    }
}

fn eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "early eof")
}

impl<A> Future for ReadExact<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        loop {
            // if our buffer is empty, then we need to read some data to continue.
            let rem = self.buf.remaining();
            if rem != 0 {
                let me = &mut *self;
                ready!(Pin::new(&mut *me.reader).poll_read(cx, &mut me.buf))?;
                if me.buf.remaining() == rem {
                    return Err(eof()).into();
                }
            } else {
                return Poll::Ready(Ok(self.buf.capacity()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_unpin() {
        use std::marker::PhantomPinned;
        crate::is_unpin::<ReadExact<'_, PhantomPinned>>();
    }
}
