// Inspired by https://github.com/alecmocatta/serde_traitobject

use std::marker::PhantomData;
use crate::tag::registry::Registry;

pub trait Serialize: serialize::Sealed {}
impl<T: serde::ser::Serialize + ?Sized> Serialize for T {}

/*
pub trait Deserialize {}
impl<T: serde::de::DeserializeOwned> Deserialize for str {}
impl<T: serde::de::DeserializeOwned> Deserialize for [T] {}
impl Deserialize for str {}
*/

pub(crate) mod serialize {
  use std::any::TypeId;

  pub trait Sealed: erased_serde::Serialize {
    fn serialize_sized<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
        Self: Sized;

    #[inline]
    fn type_id(&self) -> TypeId
      where Self: 'static
    {
      TypeId::of::<Self>()
    }
  }

  impl<T: serde::ser::Serialize + ?Sized> Sealed for T {
    #[inline]
    default fn serialize_sized<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
        Self: Sized,
    {
      let _ = serializer;
      unreachable!()
    }
  }

  impl<T: serde::ser::Serialize> Sealed for T {
    #[inline]
    fn serialize_sized<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: serde::Serializer,
        Self: Sized,
    {
      serde::ser::Serialize::serialize(self, serializer)
    }
  }
}

struct Serializer<T: Serialize + ?Sized + 'static>(PhantomData<fn(T)>);
trait SerializerTrait<T: Serialize + ?Sized> {
  fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer;
}

impl<T: Serialize> SerializerTrait<T> for Serializer<T> {
  #[inline]
  fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
    t.serialize_sized(serializer)
  }
}

impl SerializerTrait<str> for Serializer<str> {
  #[inline]
  fn serialize<S>(t: &str, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
    serde::ser::Serialize::serialize(t, serializer)
  }
}

impl<T: serde::ser::Serialize> SerializerTrait<[T]> for Serializer<[T]> {
  #[inline]
  fn serialize<S>(t: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
  {
    serde::ser::Serialize::serialize(t, serializer)
  }
}

impl<T: Serialize + ?Sized + 'static> SerializerTrait<T> for Serializer<T> {
  #[inline]
  default fn serialize<S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      let binding = Registry::registry();
      let registry = binding.lock().unwrap();
      let type_id = &<T as serialize::Sealed>::type_id(t);
      let trait_name = std::any::type_name::<T>();
      if let Some((name, _)) = registry.get(*type_id) {
        serializer.serialize_newtype_variant(trait_name, 0, name, &SerializeErased(t))
      } else {
        // TODO: Find a way to let the user know which type is not registered
        panic!("Serialization attempted in unregistered type that implements the following trait: {:}", trait_name);
      }
    }
}

struct SerializeErased<'a, T: Serialize + ?Sized + 'a>(&'a T);
impl<'a, T: Serialize + ?Sized> serde::ser::Serialize for SerializeErased<'a, T> {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    erased_serde::serialize(self.0, serializer)
  }
}

pub fn serialize<T, B, S>(
	t: &B,
  serializer: S,
) -> Result<S::Ok, S::Error>
  where
    T: Serialize + ?Sized + 'static,
    B: AsRef<T> + ?Sized,
    S: serde::Serializer,
{
	Serializer::<T>::serialize(t.as_ref(), serializer)
}
