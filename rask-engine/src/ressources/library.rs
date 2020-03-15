use super::Resource;
use crate::EngineError;

const CATALOUGE_SIZE: usize = 512;

pub struct Library {
    catalog: &'static mut [Resource],
}

macro_rules! get_store {
    ($type: ty, $enum_type: ident) => {
        impl GetStore<$type> for Library {
            unsafe fn get(&'static self, id: usize) -> Result<&'static $type, EngineError> {
                match &self.catalog[id] {
                    Resource::$enum_type(value) => Ok(&value),
                    _ => Err("Wrong ressource type".into()),
                }
            }
            unsafe fn store(&'static mut self, data: $type, id: usize) {
                self.catalog[id] = Resource::$enum_type(data);
            }
        }
    };
}

pub trait GetStore<T> {
    unsafe fn get(&'static self, id: usize) -> Result<&'static T, EngineError>;
    unsafe fn store(&'static mut self, data: T, id: usize);
}

impl Library {
    pub unsafe fn new(memory_offset: usize) -> Self {
        Library {
            catalog: core::slice::from_raw_parts_mut(
                memory_offset as *mut Resource,
                CATALOUGE_SIZE,
            ),
        }
    }
}
get_store!(super::Texture, Texture);
get_store!(spine::skeleton::Skeleton, Skeleton);
get_store!(super::Sound, Sound);
