#![allow(clippy::unreadable_literal)]
//! this module contains The game input/output event defeniton

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Key {
    Unknown = 0,
    A = 2335202,
    Enter = 67114680,
}

impl Key {
    pub fn from_u8(n: u32) -> Key {
        unsafe { std::mem::transmute(n) }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    KeyDown(KeyModifier, Key),
    KeyUp(KeyModifier, Key),
    KeyPress(u16, u16),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
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
