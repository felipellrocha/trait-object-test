use std::marker::PhantomData;
use erased_serde::serialize_trait_object;

pub trait Resource: 'static + erased_serde::Serialize + Send + Sync + std::fmt::Debug {
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

pub trait ErasedResource: 'static + std::fmt::Debug + Send + Sync {
}
//serialize_trait_object!(ErasedResource);


#[derive(Debug)]
pub struct ResourceImpl<T>
  where T: std::fmt::Debug,
{
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for ResourceImpl<T> where T: std::fmt::Debug {}
unsafe impl<T> Sync for ResourceImpl<T> where T: std::fmt::Debug {}

impl<T> ResourceImpl<T> where T: std::fmt::Debug + serde::de::DeserializeOwned {
  pub fn new(_name: &'static str) -> Self {
    Self {
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedResource for ResourceImpl<T>
  where
    //T: std::fmt::Debug + Serialize + for<'a> Deserialize<'a> + 'static
    T: std::fmt::Debug + erased_serde::Serialize + serde::de::DeserializeOwned + 'static
{
}
