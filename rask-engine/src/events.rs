#![allow(clippy::unreadable_literal)]
//! this module contains The game input/output event definition

#[derive(Debug, Clone, Copy)]
pub struct Key(u32);

impl From<u32> for Key {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Key {
    pub const UNKNOWN: Key = Key(0);
    pub const A: Key = Key(2335202);
    pub const ENTER: Key = Key(67114680);
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
