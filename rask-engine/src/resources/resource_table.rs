use super::Resource;
use super::RESOURCE_COUNT;
use crate::EngineError;
use std::fmt::Debug;

#[cfg_attr(not(feature = "nightly"), repr(transparent))]
/// The library is used to store and retrieve resources.
pub struct ResourceTable([Resource; RESOURCE_COUNT as usize]);

macro_rules! character_check_helper {
    (Texture, $value: ident) => {
        return Ok($value.texture());
    };
    ($enum_type: ident, $value: ident) => {};
}

macro_rules! get_store {
    ($type: ty, $enum_type: ident) => {
        impl GetStore<$type> for ResourceTable {
            fn get<U: Into<usize> + Debug + Copy>(&self, id: U) -> Result<&$type, EngineError> {
                self.index_check(id.into())?;
                match &self.0[id.into()] {
                    Resource::$enum_type(value) => Ok(&value),
                    Resource::None => Err(EngineError::ResourceMissing(format!(
                        "Could not find requested recource #{}",
                        id.into(),
                    ))),
                    res => {
                        #[allow(unused_variables)]
                        {
                            if let Resource::Character(value) = res {
                                character_check_helper!($enum_type, value);
                            }
                        }
                        Err(EngineError::ResourceType(format!(
                            "Wrong resource type, required \"{}\"",
                            stringify!($type),
                        )))
                    }
                }
            }

            fn store(&mut self, data: $type, id: usize) -> Result<(), EngineError> {
                self.index_check(id)?;
                Ok(self.0[id] = Resource::$enum_type(data))
            }
        }
    };
}

pub trait GetStore<T> {
    /// Retrieve a resource from the library.
    fn get<U: Into<usize> + Debug + Copy>(&self, id: U) -> Result<&T, EngineError>;

    /// Store a resource to the library
    fn store(&mut self, data: T, id: usize) -> Result<(), EngineError>;
}

impl Default for ResourceTable {
    fn default() -> Self {
        Self::new()
    }
}
impl ResourceTable {
    /// Create a new library initialzed with None resources.
    #[cfg(feature = "nightly")]
    pub const fn new() -> Self {
        Self([Resource::None; RESOURCE_COUNT as usize])
    }
    #[cfg(not(feature = "nightly"))]
    pub fn new() -> Self {
        let bytes = [0u8; std::mem::size_of::<Self>()];
        unsafe { std::mem::transmute(bytes) }
    }

    fn index_check(&self, id: usize) -> Result<(), EngineError> {
        if id >= self.0.len() {
            return Err(EngineError::ResourceIndex(format!(
                "The requested resource index: {} is out ouf range, the max id is {}",
                id,
                self.0.len() - 1
            )));
        }
        Ok(())
    }
}

get_store!(super::Texture, Texture);
get_store!(super::Sound, Sound);
get_store!(Box<super::Character>, Character);
