#![allow(clippy::unreadable_literal)]
//! this module contains The game input/output event definition
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

#[rustfmt::skip]
lazy_static::lazy_static! {
    /// This is an example for using doc comment attributes
    static ref MAPPING: HashMap<u32, u32> = [
        (977763216, 0), (977535019, 1), (251549619, 2), (930625636, 3), (4247333604, 4),
        (2823983839, 5), (2897055625, 6), (3915039450, 7), (12763084, 8), (3908895692, 9),
        (759638320, 10), (2079612435, 11), (1564800910, 12), (83829, 13), (80085222, 14),
        (67114680, 15), (3357475935, 16), (2691421945, 17), (3357357270, 18), (79966557, 19),
        (2387108321, 20), (65290933, 21), (1289876625, 22), (78401116, 23), (1149837940, 24),
        (1422382255, 25), (74348624, 26), (67204884, 27), (923631601, 28), (2383081898, 29),
        (1318883673, 30), (2255103, 31), (69819, 32), (2195042009, 33), (2043376075, 34),
        (2046924995, 35), (2046924996, 36), (2046924997, 37), (2046924998, 38), (2046924999, 39),
        (2046925000, 40), (2046925001, 41), (2046925002, 42), (2046925003, 43), (2046925004, 44),
        (2335202, 45), (2335203, 46), (2335204, 47), (2335205, 48), (2335206, 49),
        (2335207, 50), (2335208, 51), (2335209, 52), (2335210, 53), (2335211, 54),
        (2335212, 55), (2335213, 56), (2335214, 57), (2335215, 58), (2335216, 59),
        (2335217, 60), (2335218, 61), (2335219, 62), (2335220, 63), (2335221, 64),
        (2335222, 65), (2335223, 66), (2335224, 67), (2335225, 68), (2335226, 69),
        (2335227, 70), (2219, 71), (2220, 72), (2221, 73), (2222, 74),
        (2223, 75), (2224, 76), (2225, 77), (2226, 78), (2227, 79),
        (68837, 80), (68838, 81), (68839, 82)].iter().copied().collect();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Key(u32);

impl From<u32> for Key {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Key {
    pub const ARROW_LEFT: Key = Key(0);
    pub const ARROW_DOWN: Key = Key(1);
    pub const ARROW_RIGHT: Key = Key(2);
    pub const ARROW_UP: Key = Key(3);
    pub const CONTROL_LEFT: Key = Key(4);
    pub const CONTROL_RIGHT: Key = Key(5);
    pub const SHIFT_LEFT: Key = Key(6);
    pub const SHIFT_RIGHT: Key = Key(7);
    pub const CAPSLOCK: Key = Key(8);
    pub const META_LEFT: Key = Key(9);
    pub const ALT_LEFT: Key = Key(10);
    pub const ALT_RIGHT: Key = Key(11);
    pub const CONTEXT_MENU: Key = Key(12);
    pub const TAB: Key = Key(13);
    pub const SPACE: Key = Key(14);
    pub const ENTER: Key = Key(15);
    pub const BACKSPACE: Key = Key(16);
    pub const INTL_BACKSLASH: Key = Key(17);
    pub const BACKSLASH: Key = Key(18);
    pub const SLASH: Key = Key(19);
    pub const PERIOD: Key = Key(20);
    pub const COMMA: Key = Key(21);
    pub const SEMICOLON: Key = Key(22);
    pub const QUOTE: Key = Key(23);
    pub const BRACKET_RIGHT: Key = Key(24);
    pub const BRACKET_LEFT: Key = Key(25);
    pub const MINUS: Key = Key(26);
    pub const EQUAL: Key = Key(27);
    pub const PAGE_DOWN: Key = Key(28);
    pub const PAGE_UP: Key = Key(29);
    pub const PRINT_SCREEN: Key = Key(30);
    pub const HOME: Key = Key(31);
    pub const END: Key = Key(32);
    pub const INSERT: Key = Key(33);
    pub const DELETE: Key = Key(34);
    pub const DIGIT0: Key = Key(35);
    pub const DIGIT1: Key = Key(36);
    pub const DIGIT2: Key = Key(37);
    pub const DIGIT3: Key = Key(38);
    pub const DIGIT4: Key = Key(39);
    pub const DIGIT5: Key = Key(40);
    pub const DIGIT6: Key = Key(41);
    pub const DIGIT7: Key = Key(42);
    pub const DIGIT8: Key = Key(43);
    pub const DIGIT9: Key = Key(44);
    pub const KEY_A: Key = Key(45);
    pub const KEY_B: Key = Key(46);
    pub const KEY_C: Key = Key(47);
    pub const KEY_D: Key = Key(48);
    pub const KEY_E: Key = Key(49);
    pub const KEY_F: Key = Key(50);
    pub const KEY_G: Key = Key(51);
    pub const KEY_H: Key = Key(52);
    pub const KEY_I: Key = Key(53);
    pub const KEY_J: Key = Key(54);
    pub const KEY_K: Key = Key(55);
    pub const KEY_L: Key = Key(56);
    pub const KEY_M: Key = Key(57);
    pub const KEY_N: Key = Key(58);
    pub const KEY_O: Key = Key(59);
    pub const KEY_P: Key = Key(60);
    pub const KEY_Q: Key = Key(61);
    pub const KEY_R: Key = Key(62);
    pub const KEY_S: Key = Key(63);
    pub const KEY_T: Key = Key(64);
    pub const KEY_U: Key = Key(65);
    pub const KEY_V: Key = Key(66);
    pub const KEY_W: Key = Key(67);
    pub const KEY_X: Key = Key(68);
    pub const KEY_Y: Key = Key(69);
    pub const KEY_Z: Key = Key(70);
    pub const F1: Key = Key(71);
    pub const F2: Key = Key(72);
    pub const F3: Key = Key(73);
    pub const F4: Key = Key(74);
    pub const F5: Key = Key(75);
    pub const F6: Key = Key(76);
    pub const F7: Key = Key(77);
    pub const F8: Key = Key(78);
    pub const F9: Key = Key(79);
    pub const F10: Key = Key(80);
    pub const F11: Key = Key(81);
    pub const F12: Key = Key(82);
    pub fn from_hash(hash: u32) -> Option<Self> {
        MAPPING.get(&hash).map(|key| Self(*key))
    }
}

#[test]
fn check_keycodes() {
    assert_eq!(Key::KEY_R, Key::from_hash(2335219).unwrap());
    assert!(Key::from_hash(2378235219).is_none());
}

pub struct Keyboard([AtomicBool; 83]);

impl Keyboard {
    pub fn new() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }

    pub fn set(&self, key: Key, value: bool) {
        assert!((key.0 as usize) < self.0.len());
        self.0[key.0 as usize].store(value, Ordering::Relaxed);
    }

    pub fn get(&self, key: Key) -> bool {
        assert!((key.0 as usize) < self.0.len());
        self.0[key.0 as usize].load(Ordering::Relaxed)
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
    pub const fn shift(self) -> bool {
        self.0 & 1 != 0
    }

    pub const fn control(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn alt(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn meta(self) -> bool {
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
