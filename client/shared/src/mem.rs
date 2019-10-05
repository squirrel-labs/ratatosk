use crate::state::State;
type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::{sprite::*, texture::*};

const fn KiB(n: usize) -> usize {
    n * 1024
}
const fn MiB(n: usize) -> usize {
    n * KiB(1024)
}

const STACK_ALIGNMENT: usize = 1024 * 63;

pub const GRAPHIC_STACK_SIZE: usize = MiB(4) + STACK_ALIGNMENT;
pub const ALLOCATOR_AREA_START: usize = GRAPHIC_STACK_SIZE;
pub const SHARED_BUFFER_AREA_START: usize = ALLOCATOR_AREA_START + MiB(1);
pub const LOGIC_ALLOCATION_AREA_START: usize =
    SHARED_BUFFER_AREA_START + std::mem::size_of::<Buffer>();
pub const GRAPHICS_ALLOCATION_AREA_START: usize = LOGIC_ALLOCATION_AREA_START + MiB(32);
pub const SHARED_ALLOCATION_AREA_START: usize = GRAPHICS_ALLOCATION_AREA_START + MiB(32);

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(SHARED_BUFFER_AREA_START as *mut Buffer) }
}

pub struct SharedHeap {
    last_addr: u32,
    animations: Vec<Animation>,
    texture_notify: bool,
    textures: Option<Vec<Texture>>,
}

impl SharedHeap {
    pub fn animations_mut(&mut self) -> &mut Vec<Animation> {
        &mut self.animations
    }

    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }

    pub fn unset_texture_notify(&mut self) {
        self.texture_notify = false
    }

    pub fn set_texture_notify(&mut self) {
        self.texture_notify = true
    }

    pub fn get_texture_notify(&mut self) -> bool {
        self.texture_notify
    }

    pub fn textures_mut(&mut self) -> &mut Option<Vec<Texture>> {
        &mut self.textures
    }

    pub fn textures(&self) -> &Option<Vec<Texture>> {
        &self.textures
    }
}

pub fn shared_heap() -> &'static mut SharedHeap {
    unsafe { &mut *(SHARED_ALLOCATION_AREA_START as *mut SharedHeap) }
}
