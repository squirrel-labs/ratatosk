use super::Resource;
use crate::EngineError;

/// The library is used to store and retrieve resources.
pub struct ResourceTable(&'static mut [Resource]);

macro_rules! get_store {
    ($type: ty, $enum_type: ident) => {
        impl GetStore<$type> for ResourceTable {
            unsafe fn get(&'static self, id: usize) -> Result<&'static $type, EngineError> {
                self.index_check(id)?;
                match &self.0[id] {
                    Resource::$enum_type(value) => Ok(&value),
                    _ => Err("Wrong resource type".into()),
                }
            }
            unsafe fn store(&mut self, data: $type, id: usize) -> Result<(), EngineError> {
                self.index_check(id)?;
                Ok(self.0[id] = Resource::$enum_type(data))
            }
        }
    };
}

pub trait GetStore<T> {
    /// Retrieve a resource from the library.
    ///
    /// # Safety
    ///
    /// The function is not thread safe.
    unsafe fn get(&'static self, id: usize) -> Result<&'static T, EngineError>;
    /// Store a resource to the library
    ///
    /// # Safety
    ///
    /// The function is not thread safe.
    unsafe fn store(&mut self, data: T, id: usize) -> Result<(), EngineError>;
}

impl ResourceTable {
    /// Create a new library at a specific position in memory.
    ///
    /// # Safety
    ///
    /// The function is safe as long as the memory from memory_offset to memory_offset + CATALOG_SIZE * sizeof(Resource)
    pub unsafe fn new(memory_offset: usize, catalog_size: usize) -> Self {
        ResourceTable(core::slice::from_raw_parts_mut(
            memory_offset as *mut Resource,
            catalog_size,
        ))
    }
    pub unsafe fn init(&mut self) {
        for i in 0..self.0.len() {
            self.0[i] = Resource::None;
        }
    }

    fn index_check(&self, id: usize) -> Result<(), EngineError> {
        if id >= self.0.len() {
            return Err(EngineError::ResourceError(format!(
                "The requested resource index: {} is out ouf range, the max id is {}",
                id,
                self.0.len() - 1
            )));
        }
        Ok(())
    }
}

get_store!(super::Texture, Texture);
get_store!(spine::skeleton::Skeleton, Skeleton);
get_store!(super::Sound, Sound);
