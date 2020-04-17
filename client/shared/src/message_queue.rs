use crate::mem::{atomic_read_u8, atomic_write_u8, MESSAGE_QUEUE, MESSAGE_QUEUE_ELEMENT_COUNT};

#[repr(C, u32)]
#[derive(Debug, Clone)]
pub enum Message {
    None,
    KeyDown(KeyModifier, i32),
    KeyUp(KeyModifier, i32),
    KeyPress(u16),
    TextInput(bool),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    ResizeWindow { width: i32, height: i32 },
    ResquestAlloc(i32),
}

impl Default for Message {
    fn default() -> Self {
        Message::None
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct KeyModifier(u8);
impl KeyModifier {
    pub fn shift(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn control(&self) -> bool {
        self.0 & (1 << 1) == 1
    }
    pub fn alt(&self) -> bool {
        self.0 & (1 << 2) == 1
    }
    pub fn meta(&self) -> bool {
        self.0 & (1 << 3) == 1
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
        self.buttons & 1 == 1
    }
    pub fn right_mb(&self) -> bool {
        self.buttons & (1 << 1) == 1
    }
    pub fn middle_mb(&self) -> bool {
        self.buttons & (1 << 2) == 1
    }
}

#[repr(C, align(32))]
#[derive(Debug)]
pub struct MessageQueueElement<T: Sized + Clone> {
    writing: u8,
    payload: T,
}

impl<T: Sized + Clone + Default> MessageQueueElement<T> {
    fn get_writing(&self) -> u8 {
        unsafe { atomic_read_u8(&self.writing) }
    }
    fn read(&mut self) -> Option<T> {
        let e = std::mem::replace(&mut self.payload, T::default());
        if self.get_writing() == 0 {
            Some(e)
        } else {
            None
        }
    }
}

pub struct MessageQueueReader {
    /// the index of the next element to be read
    reader_index: u32,
}

impl MessageQueueReader {
    pub fn length() -> usize {
        MESSAGE_QUEUE_ELEMENT_COUNT
    }

    pub fn new() -> Self {
        MessageQueueReader { reader_index: 0 }
    }

    unsafe fn get_mut<T: Sized + Clone>(
        &mut self,
        n: usize,
    ) -> Option<&mut MessageQueueElement<T>> {
        core::slice::from_raw_parts_mut(
            MESSAGE_QUEUE as *mut MessageQueueElement<T>,
            Self::length(),
        )
        .get_mut(n)
    }

    pub fn pop<T: Sized + Clone + Default + std::fmt::Debug>(&mut self) -> T {
        loop {
            let i = self.reader_index;
            let e = unsafe { self.get_mut(self.reader_index as usize).unwrap() };
            /*log::info!(
                "bytes are  {:?}\nreading at: {}",
                unsafe {
                    std::slice::from_raw_parts_mut(e as *mut MessageQueueElement<_> as *mut u8, 16)
                },
                i
            );*/
            let e = e.read();
            if let Some(n) = e.clone() {
                if std::mem::discriminant(&T::default()) == std::mem::discriminant(&n) {
                    return n;
                }
            }
            self.reader_index += 1;
            if self.reader_index as usize >= Self::length() {
                self.reader_index = 0;
            }
            match e {
                None => continue,
                Some(msg) => return msg,
            }
        }
    }
}
