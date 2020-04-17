//! this module contains The game input/output event defeniton

#![allow(clippy::unreadable_literal)]
#[repr(u32)]
pub enum Key {
    A = 2335202,
    Enter = 67114680,
}

pub enum Event {
    KeyDown,
    KeyUp,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct KeyModifier(u8);
impl KeyModifier {
    pub fn shift(self) -> bool {
        self.0 & 1 != 0
    }
    pub fn control(self) -> bool {
        self.0 & (1 << 1) != 0
    }
    pub fn alt(self) -> bool {
        self.0 & (1 << 2) != 0
    }
    pub fn meta(self) -> bool {
        self.0 & (1 << 3) != 0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MouseEvent {
    buttons: u8,
    pub modifier: KeyModifier,
    pub x: i32,
    pub y: i32,
}

impl MouseEvent {
    pub fn left_mb(&self) -> bool {
        self.buttons & 1 != 0
    }
    pub fn right_mb(&self) -> bool {
        self.buttons & (1 << 1) != 0
    }
    pub fn middle_mb(&self) -> bool {
        self.buttons & (1 << 2) != 0
    }
}
