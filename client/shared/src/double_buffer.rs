use crate::mem;
use std::fmt::Debug;

pub trait Element: Clone + Sized + Default + Debug {}
type Flag = u8;

impl<T: Clone + Sized + Default + Debug> Element for T {}

#[derive(Debug, Default)]
pub struct DoubleBuffer<T: Element> {
    pub(self) reading_at: Flag,
    pub(self) provided: Flag,
    buffer: [T; 2],
}

#[derive(Debug)]
pub struct ReaderBufferView<'a, T: Element> {
    ptr: &'a mut DoubleBuffer<T>,
    read_pos: u8,
}

#[derive(Debug)]
pub struct WriterBufferView<'a, T: Element> {
    ptr: &'a mut DoubleBuffer<T>,
    write_pos: u8,
}

impl<T: Element> DoubleBuffer<T> {
    pub fn new() -> Self {
        DoubleBuffer {
            reading_at: 0,
            provided: 0,
            buffer: [T::default(), T::default()],
        }
    }

    pub fn borrow_reader(&mut self) -> Option<ReaderBufferView<T>> {
        match (self.get_reading_at(), self.get_provided()) {
            (0, 0) => None,
            (0, p) => {
                let mut x = p;
                self.set_reading_at(x);
                while x != p {
                    x = p;
                    self.set_reading_at(x);
                }
                Some(ReaderBufferView {
                    ptr: self,
                    read_pos: x - 1,
                })
            }
            (c, p) => panic!("invalid state ({},{}) for consumer reached", c, p),
        }
    }

    pub fn borrow_writer(&mut self) -> WriterBufferView<T> {
        let write_pos = match (self.get_reading_at(), self.get_provided()) {
            (0, 0) => 0,
            (0, y) => 2 - y,
            (y, x) => {
                if x == y {
                    2 - y
                } else {
                    self.set_provided(y);
                    y - 1
                }
            }
        };
        WriterBufferView {
            ptr: self,
            write_pos,
        }
    }

    pub extern "C" fn set_reading_at(&mut self, reading_at: Flag) {
        unsafe { mem::atomic_write_u8(&mut self.reading_at, reading_at) }
    }

    pub extern "C" fn get_reading_at(&self) -> Flag {
        unsafe { mem::atomic_read_u8(&self.reading_at) }
    }

    pub extern "C" fn set_provided(&mut self, provided: Flag) {
        unsafe { mem::atomic_write_u8(&mut self.provided, provided) }
    }

    pub extern "C" fn get_provided(&self) -> Flag {
        unsafe { mem::atomic_read_u8(&self.provided) }
    }
}

impl<'a, T: Element> ReaderBufferView<'a, T> {
    pub fn get(&self) -> &T {
        &self.ptr.buffer[self.read_pos as usize]
    }
}

impl<'a, T: Element> WriterBufferView<'a, T> {
    pub fn get(&self) -> &T {
        &self.ptr.buffer[self.write_pos as usize]
    }

    pub fn set(&mut self, data: T) {
        self.ptr.buffer[self.write_pos as usize] = data;
    }
}

impl<'a, T: Element> std::ops::Drop for ReaderBufferView<'a, T> {
    fn drop(&mut self) {
        self.ptr.set_reading_at(0);
    }
}

impl<'a, T: Element> std::ops::Drop for WriterBufferView<'a, T> {
    fn drop(&mut self) {
        self.ptr.set_provided(self.write_pos + 1);
    }
}
