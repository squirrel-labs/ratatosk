#![allow(clippy::unreadable_literal)]
//! this module contains The game input/output event definition

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Key(u32);

impl From<u32> for Key {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

impl Key {
    pub const UNKNOWN: Key = Key(0);
    pub const CONTROL_LEFT: Key = Key(4247333604);
    pub const META_LEFT: Key = Key(3908895692);
    pub const ALT_LEFT: Key = Key(759638320);
    pub const SPACE: Key = Key(80085222);
    pub const ALT_RIGHT: Key = Key(2079612435);
    pub const CONTEXT_MENU: Key = Key(1564800910);
    pub const CONTROL_RIGHT: Key = Key(2823983839);
    pub const ARROW_LEFT: Key = Key(977763216);
    pub const ARROW_DOWN: Key = Key(977535019);
    pub const ARROW_RIGHT: Key = Key(251549619);
    pub const ARROW_UP: Key = Key(930625636);
    pub const END: Key = Key(69819);
    pub const SHIFT_RIGHT: Key = Key(3915039450);
    pub const SLASH: Key = Key(79966557);
    pub const PERIOD: Key = Key(2387108321);
    pub const COMMA: Key = Key(65290933);
    pub const KEY_M: Key = Key(2335214);
    pub const KEY_N: Key = Key(2335215);
    pub const KEY_B: Key = Key(2335203);
    pub const KEY_V: Key = Key(2335223);
    pub const KEY_C: Key = Key(2335204);
    pub const KEY_X: Key = Key(2335225);
    pub const KEY_Z: Key = Key(2335227);
    pub const INTL_BACKSLASH: Key = Key(2691421945);
    pub const SHIFT_LEFT: Key = Key(2897055625);
    pub const CAPSLOCK: Key = Key(12763084);
    pub const KEY_A: Key = Key(2335202);
    pub const KEY_S: Key = Key(2335220);
    pub const KEY_D: Key = Key(2335205);
    pub const KEY_F: Key = Key(2335207);
    pub const KEY_G: Key = Key(2335208);
    pub const KEY_H: Key = Key(2335209);
    pub const KEY_J: Key = Key(2335211);
    pub const KEY_K: Key = Key(2335212);
    pub const KEY_L: Key = Key(2335213);
    pub const SEMICOLON: Key = Key(1289876625);
    pub const QUOTE: Key = Key(78401116);
    pub const BACKSLASH: Key = Key(3357357270);
    pub const ENTER: Key = Key(67114680);
    pub const PAGE_DOWN: Key = Key(923631601);
    pub const PAGE_UP: Key = Key(2383081898);
    pub const BRACKET_RIGHT: Key = Key(1149837940);
    pub const BRACKET_LEFT: Key = Key(1422382255);
    pub const KEY_P: Key = Key(2335217);
    pub const KEY_O: Key = Key(2335216);
    pub const KEY_I: Key = Key(2335210);
    pub const KEY_U: Key = Key(2335222);
    pub const KEY_Y: Key = Key(2335226);
    pub const KEY_T: Key = Key(2335221);
    pub const KEY_R: Key = Key(2335219);
    pub const KEY_E: Key = Key(2335206);
    pub const KEY_W: Key = Key(2335224);
    pub const KEY_Q: Key = Key(2335218);
    pub const TAB: Key = Key(83829);
    pub const DIGIT1: Key = Key(2046924996);
    pub const DIGIT2: Key = Key(2046924997);
    pub const DIGIT3: Key = Key(2046924998);
    pub const DIGIT4: Key = Key(2046924999);
    pub const DIGIT5: Key = Key(2046925000);
    pub const DIGIT6: Key = Key(2046925001);
    pub const DIGIT7: Key = Key(2046925002);
    pub const DIGIT8: Key = Key(2046925003);
    pub const DIGIT9: Key = Key(2046925004);
    pub const DIGIT0: Key = Key(2046924995);
    pub const MINUS: Key = Key(74348624);
    pub const EQUAL: Key = Key(67204884);
    pub const BACKSPACE: Key = Key(3357475935);
    pub const HOME: Key = Key(2255103);
    pub const DELETE: Key = Key(2043376075);
    pub const PRINT_SCREEN: Key = Key(1318883673);
    pub const INSERT: Key = Key(2195042009);
    pub const F1: Key = Key(2219);
    pub const F2: Key = Key(2220);
    pub const F3: Key = Key(2221);
    pub const F4: Key = Key(2222);
    pub const F5: Key = Key(2223);
    pub const F6: Key = Key(2224);
    pub const F7: Key = Key(2225);
    pub const F8: Key = Key(2226);
    pub const F9: Key = Key(2227);
    pub const F10: Key = Key(68837);
    pub const F11: Key = Key(68838);
    pub const F12: Key = Key(68839);
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
