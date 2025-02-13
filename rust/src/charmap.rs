use core::ops::Deref;
use core::ptr::slice_from_raw_parts;

use arrayvec::ArrayVec;

#[repr(transparent)]
pub struct Pkstr([u8]);

impl Pkstr {
    pub unsafe fn from_ptr<'a>(ptr: *const u8, len: usize) -> &'a Pkstr {
        unsafe { pkstr_raw(&*slice_from_raw_parts(ptr, len)) }
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub struct ArrayPkstr<const CAP: usize>(ArrayVec<u8, CAP>);

impl<const CAP: usize> ArrayPkstr<CAP> {
    pub unsafe fn from_slice(slice: &[u8]) -> Self {
        let mut a = Self(ArrayVec::<u8, CAP>::new());
        a.0.try_extend_from_slice(slice).unwrap();
        a
    }

    pub fn from_str(str: &str) -> Self {
        let mut a = ArrayVec::<u8, CAP>::new();
        for char in str.chars() {
            a.push(map(char as u8));
        }
        a.push(0xFF);
        Self(a)
    }
}

impl<const CAP: usize> Deref for ArrayPkstr<CAP> {
    type Target = Pkstr;
    fn deref(&self) -> &Self::Target {
        unsafe { Pkstr::from_ptr(self.0.as_ptr(), self.0.len()) }
    }
}

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
        b'/' => 0xBA,
        0 => 0xFF,
        _ => 0xAE,
    }
}

const fn map_special(bytes: &[u8]) -> &'static [u8] {
    match bytes {
        b"PAUSE" => &[0xFC, 0x09],
        b"PARAGRAPH" | b"P" => &[0xFB],
        _ => panic!("Invalid special char"),
    }
}

#[macro_export]
macro_rules! pkstr {
    ($str:literal) => {{
        use $crate::charmap::*;
        const LEN: usize = pkstr_bytes_len($str);
        const ARR: [u8; LEN] = pkstr_build::<LEN>($str);
        unsafe { pkstr_raw(&ARR) }
    }};
}

const fn index_of(input: &[u8], check: u8) -> usize {
    let mut index = 0;
    while index < input.len() {
        if input[index] == check {
            return index;
        }
        index += 1;
    }
    panic!("Couldn't find char")
}

pub const fn pkstr_bytes_len(input: &[u8]) -> usize {
    let mut index = 0;
    let mut size = 0;
    while index < input.len() {
        if input[index] == b'{' {
            let (_, remaining) = input.split_at(index + 1);
            let content_len = index_of(remaining, b'}');
            let (content, _) = remaining.split_at(content_len);
            size += map_special(content).len();
            index += content_len + 2;
        } else {
            size += 1;
            index += 1;
        }
    }
    size + 1
}

pub const fn pkstr_write(buf: &mut [u8], input: &[u8]) {
    let mut index = 0;
    let mut offset = 0;
    while index < input.len() {
        if input[index] == b'{' {
            let (_, remaining) = input.split_at(index + 1);
            let content_len = index_of(remaining, b'}');
            let (content, _) = remaining.split_at(content_len);
            index += content_len + 2;

            let to_write = map_special(content);
            let (_, write) = buf.split_at_mut(offset);
            let mut write_index = 0;
            while write_index < to_write.len() {
                write[write_index] = to_write[write_index];
                write_index += 1;
                offset += 1;
            }
        } else {
            buf[offset] = map(input[index]);
            index += 1;
            offset += 1;
        }
    }
    buf[offset] = 0xFF;
}

pub const fn pkstr_build<const S: usize>(input: &[u8]) -> [u8; S] {
    let mut ret = [0u8; S];
    pkstr_write(&mut ret, input);
    ret
}

pub const unsafe fn pkstr_raw(src: &[u8]) -> &Pkstr {
    &*(&raw const *src as *const Pkstr)
}
