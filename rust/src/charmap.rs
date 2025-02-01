use core::ffi::CStr;

const fn map(char: u8) -> u8 {
    match char {
        c @ b'a'..=b'z' => c - b'a' + 0xd5,
        c @ b'A'..=b'Z' => c - b'A' + 0xbb,
        c @ b'0'..=b'9' => c - b'0' + 0xa1,
        b' ' => 0x00,
        b'!' => 0xAB,
        b'?' => 0xAC,
        b'.' => 0xAD,
        b'-' => 0xAE,
        b'_' => 0xAE,
        b':' => 0xF0,
        b'>' => 0x86,
        b'<' => 0x85,
        b')' => 0x5D,
        b'(' => 0x5C,
        b',' => 0x35,
        b'+' => 0x2E,
        b'&' => 0x2D,
        0 => 0xFF,
        _ => 0xAE,
    }
}
#[macro_export]
macro_rules! pokestr {
    ($str:literal) => {
        $crate::charmap::map_bytes::<{$str.len() + 1}>($str)
    }
}

pub const fn map_bytes<const T: usize>(bytes: &[u8]) -> [u8; T] {
    let mut i = 0;
    let mut ret = [0; T];
    while i < T - 1 {
        ret[i] = map(bytes[i]);
        i += 1;
    }
    ret[i] = 0xff;
    ret
}
