use core::cmp::min;
use core::fmt::{self, Arguments, Write as _};
use core::mem;

pub(crate) trait Write {
    fn write(&mut self, buf: &[u8]) -> core::fmt::Result;
    fn write_fmt(&mut self, args: Arguments) -> core::fmt::Result;
}

impl Write for &mut [u8] {
    fn write(&mut self, buf: &[u8]) -> core::fmt::Result {
        let count = min(self.len(), buf.len());
        let (a, b) = mem::take(self).split_at_mut(count);
        a.copy_from_slice(&buf[..count]);
        *self = b;
        Ok(())
    }
    fn write_fmt(&mut self, args: Arguments) -> fmt::Result {
        struct Adapter<'a>(&'a mut [u8]);
        impl fmt::Write for Adapter<'_> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.write(s.as_bytes())
            }
        }
        Adapter(self).write_fmt(args)
    }
}
