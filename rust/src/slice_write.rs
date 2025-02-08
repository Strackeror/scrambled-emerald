use core::cmp::min;
use core::fmt::{self, Arguments, Write as _};
use core::{mem, usize};

pub use arrayvec::ArrayString;
use arrayvec::ArrayVec;

pub(crate) trait Write {
    fn write(&mut self, buf: &[u8]) -> core::fmt::Result;
    fn write_fmt(&mut self, args: Arguments) -> core::fmt::Result
    where
        Self: Sized,
    {
        struct Adapter<'a>(&'a mut dyn Write);
        impl fmt::Write for Adapter<'_> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.write(s.as_bytes())
            }
        }
        Adapter(self).write_fmt(args)
    }
}

impl Write for &mut [u8] {
    fn write(&mut self, buf: &[u8]) -> core::fmt::Result {
        let count = min(self.len(), buf.len());
        let (a, b) = mem::take(self).split_at_mut(count);
        a.copy_from_slice(&buf[..count]);
        *self = b;
        Ok(())
    }
}

impl<const C: usize> Write for ArrayVec<u8, C> {
    fn write(&mut self, buf: &[u8]) -> core::fmt::Result {
        let remaining = C - self.len();
        let end = min(buf.len(), remaining);
        self.try_extend_from_slice(&buf[0..end])
            .map_err(|_| fmt::Error)
    }
}

#[macro_export]
macro_rules! aformat {
    ($size:literal, $($t:tt)*) => {
        {
            let mut buf = $crate::slice_write::ArrayString::<$size>::new();
            write!(buf, $($t)*).expect("should have space");
            buf
        }
    };
}
