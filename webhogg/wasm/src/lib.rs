//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod talloc;
#[global_allocator]
static A: talloc::TAllocator = talloc::TAllocator;

mod logger;
pub(crate) mod memory;
pub mod error;
pub mod context;

pub mod logic;
pub mod graphics;

pub use logic::*;
pub use graphics::*;
