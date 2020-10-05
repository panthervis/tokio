use crate::io::seek::{seek, Seek};
use crate::io::AsyncSeek;
use std::io::SeekFrom;

cfg_io_util! {
    /// An extension trait which adds utility methods to [`AsyncSeek`] types.
    ///
    /// As a convenience, this trait may be imported using the [`prelude`]:
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Cursor, SeekFrom};
    /// use tokio::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let mut cursor = Cursor::new(b"abcdefg");
    ///
    ///     // the `seek` method is defined by this trait
    ///     cursor.seek(SeekFrom::Start(3)).await?;
    ///
    ///     let mut buf = [0; 1];
    ///     let n = cursor.read(&mut buf).await?;
    ///     assert_eq!(n, 1);
    ///     assert_eq!(buf, [b'd']);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// See [module][crate::io] documentation for more details.
    ///
    /// [`AsyncSeek`]: AsyncSeek
    /// [`prelude`]: crate::prelude
    pub trait AsyncSeekExt: AsyncSeek {
        /// Creates a future which will seek an IO object, and then yield the
        /// new position in the object and the object itself.
        ///
        /// Equivalent to:
        ///
        /// ```ignore
        /// async fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
        /// ```
        ///
        /// In the case of an error the buffer and the object will be discarded, with
        /// the error yielded.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use tokio::fs::File;
        /// use tokio::prelude::*;
        ///
        /// use std::io::SeekFrom;
        ///
        /// # async fn dox() -> std::io::Result<()> {
        /// let mut file = File::open("foo.txt").await?;
        /// file.seek(SeekFrom::Start(6)).await?;
        ///
        /// let mut contents = vec![0u8; 10];
        /// file.read_exact(&mut contents).await?;
        /// # Ok(())
        /// # }
        /// ```
        fn seek(&mut self, pos: SeekFrom) -> Seek<'_, Self>
        where
            Self: Unpin,
        {
            seek(self, pos)
        }
    }
}

impl<S: AsyncSeek + ?Sized> AsyncSeekExt for S {}
