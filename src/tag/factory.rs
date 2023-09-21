use std::any::Any;
use std::marker::PhantomData;
use erased_serde::{serialize_trait_object, Error};

pub trait Resource: 'static + erased_serde::Serialize + Send + Sync {
}

serialize_trait_object!(Resource);
/*
impl<'erased> Serialize for dyn Resource + 'erased {
  fn serialize<S>(&self, serializer: S) -> ::erased_serde::__private::Result<S::Ok, S::Error>
  where
      S: erased_serde::__private::serde::Serializer,
  {
    ::erased_serde::serialize(self, serializer)
  }
}
*/

pub trait ErasedResource: 'static + Send + Sync {
}

impl dyn ErasedResource {
  pub fn deserialize<T>(&self, deserializer: &mut dyn erased_serde::Deserializer) -> Result<Box<T>, Error>
  where
    T: serde::de::DeserializeOwned + 'static
  {
    let ty = erased_serde::deserialize::<T>(deserializer)?;
    Ok(Box::new(ty))
  }
}
//serialize_trait_object!(ErasedResource);


pub struct ResourceImpl<T> {
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for ResourceImpl<T> {}
unsafe impl<T> Sync for ResourceImpl<T> {}

impl<T> ResourceImpl<T> where T: serde::de::DeserializeOwned {
  pub fn new(_name: &'static str) -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedResource for ResourceImpl<T>
  where
    //T: std::fmt::Debug + Serialize + for<'a> Deserialize<'a> + 'static
    T: erased_serde::Serialize + serde::de::DeserializeOwned + 'static
{
}
