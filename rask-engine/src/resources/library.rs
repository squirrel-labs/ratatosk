use super::Resource;
use crate::EngineError;

/// Size of the internal catalog
/// This determines the highest available id.
const CATALOG_SIZE: usize = 512;

/// The library is used to store and retrive Resources
pub struct Library {
    catalog: &'static mut [Resource],
}

macro_rules! get_store {
    ($type: ty, $enum_type: ident) => {
        impl GetStore<$type> for Library {
            unsafe fn get(&'static self, id: usize) -> Result<&'static $type, EngineError> {
                match &self.catalog[id] {
                    Resource::$enum_type(value) => Ok(&value),
                    _ => Err("Wrong resource type".into()),
                }
            }
            unsafe fn store(&'static mut self, data: $type, id: usize) {
                self.catalog[id] = Resource::$enum_type(data);
            }
        }
    };
}

pub trait GetStore<T> {
    /// retrive a resource from the library
    /// # Safety
    /// the function is not concurrency safe
    unsafe fn get(&'static self, id: usize) -> Result<&'static T, EngineError>;
    /// store a resource to the library
    /// # Safety
    /// the function is not concurrency safe
    unsafe fn store(&'static mut self, data: T, id: usize);
}

impl Library {
    /// Create a new library at a specific position in memory
    /// # Safety
    /// the function is save as long as the memory from memory_offset to memory_offset + CATALOG_SIZE * sizeof(Resource)
    pub unsafe fn new(memory_offset: usize) -> Self {
        Library {
            catalog: core::slice::from_raw_parts_mut(memory_offset as *mut Resource, CATALOG_SIZE),
        }
    }

    pub unsafe fn init(&mut self) {
        for i in 0..CATALOG_SIZE {
            self.catalog[i] = Resource::None;
        }
    }
}

get_store!(super::Texture, Texture);
get_store!(spine::skeleton::Skeleton, Skeleton);
get_store!(super::Sound, Sound);
