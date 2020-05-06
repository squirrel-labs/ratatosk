#![feature(allocator_api)]
#![feature(link_llvm_intrinsics)]
#![feature(llvm_asm)]
#![feature(arbitrary_enum_discriminant)]

pub mod alloc;
pub mod double_buffer;
pub mod error;
pub mod mem;
pub mod message_queue;
pub mod sprite;
pub mod state;
pub mod wasm_log;

pub use error::*;
pub use mem::*;

pub use wee_alloc;
