use std::marker::PhantomData;
use serde::{Serialize, Deserialize};

pub trait Resource: 'static + Serialize + for<'a> Deserialize<'a> + Send + Sync + std::fmt::Debug {
}

pub trait ErasedResource: 'static + std::fmt::Debug + Send + Sync {
}


#[derive(Debug)]
pub struct ResourceImpl<T>
  where T: std::fmt::Debug,
{
  name: &'static str,
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for ResourceImpl<T> where T: std::fmt::Debug {}
unsafe impl<T> Sync for ResourceImpl<T> where T: std::fmt::Debug {}

impl<T> ResourceImpl<T> where T: std::fmt::Debug {
  pub fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedResource for ResourceImpl<T>
  where
    T: std::fmt::Debug + Serialize + for<'a> Deserialize<'a> + 'static
{
}
