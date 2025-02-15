#![allow(unused)]

use crate::pokeemerald::gMain;

#[derive(Clone, Copy)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
    R,
    L,
}

impl Button {
    pub fn code(self) -> u16 {
        match self {
            Button::A => 1 << 0,
            Button::B => 1 << 1,
            Button::Select => 1 << 2,
            Button::Start => 1 << 3,
            Button::Right => 1 << 4,
            Button::Left => 1 << 5,
            Button::Up => 1 << 6,
            Button::Down => 1 << 7,
            Button::R => 1 << 8,
            Button::L => 1 << 9,
        }
    }
    pub fn pressed(self) -> bool {
        unsafe { gMain.newKeys & self.code() != 0 }
    }
    pub fn held(self) -> bool {
        unsafe { gMain.heldKeys & self.code() != 0 }
    }
    pub fn repeat(self) -> bool {
        unsafe { gMain.newAndRepeatedKeys & self.code() != 0 }
    }
}
