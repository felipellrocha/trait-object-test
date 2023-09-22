use std::marker::PhantomData;
use erased_serde::{serialize_trait_object, Error};
use super::tagged::Deserialize;

pub trait Resource: 'static + erased_serde::Serialize + Deserialize + Send + Sync {
}

serialize_trait_object!(Resource);

pub trait ErasedResource: 'static + Send + Sync {
}

/*
impl dyn ErasedResource {
  pub fn deserialize<T>(&self, deserializer: &mut dyn erased_serde::Deserializer<'_>) -> Result<Box<T>, Error>
  where
    T: Deserialize + 'static
  {
    Ok(Box::new(erased_serde::deserialize::<T>(deserializer)?))
  }
}
*/

pub struct ResourceImpl<T> {
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for ResourceImpl<T> {}
unsafe impl<T> Sync for ResourceImpl<T> {}

impl<T> ResourceImpl<T> where T: Deserialize {
  pub fn new(_name: &'static str) -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedResource for ResourceImpl<T>
  where
    //T: std::fmt::Debug + Serialize + for<'a> Deserialize<'a> + 'static
    T: erased_serde::Serialize + Deserialize + 'static
{
}
