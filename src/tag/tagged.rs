// Inspired by https://github.com/alecmocatta/serde_traitobject

use std::any::Any;
use std::marker::PhantomData;
use crate::tag::{
  factory::ErasedResource,
  registry::{DeserializeFn, Registry},
};


// ============
// Serialize
// ============

pub trait Serialize: serialize::Sealed {}
impl<T: serde::ser::Serialize + ?Sized> Serialize for T {}

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
    if let Some(name) = registry.get_name(*type_id) {
      serializer.serialize_newtype_variant(trait_name, 0, name, &SerializeErased(t))
    } else {
      // TODO: Find a way to let the user know which type is not registered
      //panic!("Serialization attempted in unregistered type that implements the following trait: {:}", trait_name);
      Err(serde::ser::Error::custom(format!("Serialization attempted in unregistered type that implements the following trait: {:}", trait_name)))
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
// ============
// Serialize
// ============


// ============
// Deserialize
// ============
pub trait Deserialize: deserialize::Sealed {}
impl<T: serde::de::DeserializeOwned> Deserialize for T {}
impl<T: serde::de::DeserializeOwned> Deserialize for [T] {}
impl Deserialize for str {}

pub(crate) mod deserialize {
  use std::ptr::NonNull;
  use std::any::TypeId;

  pub trait Sealed {
    fn deserialize_erased(
      self: *const Self,
      deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<NonNull<()>, erased_serde::Error> {
      let _ = deserializer;
      unreachable!()
    }

    fn deserialize_box<'de, D>(deserializer: D) -> Result<Box<Self>, D::Error>
      where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
      let _ = deserializer;
      unreachable!()
    }

    #[inline]
    fn type_id(self: *const Self) -> TypeId where Self: 'static {
      TypeId::of::<Self>()
    }
  }

  impl<T: serde::de::DeserializeOwned> Sealed for T {
    #[inline]
    fn deserialize_erased(
      self: *const Self,
      deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<NonNull<()>, erased_serde::Error> {
      erased_serde::deserialize::<Self>(deserializer)
        .map(|item| NonNull::new(Box::into_raw(Box::new(item)).cast()).unwrap())
    }

    fn deserialize_box<'de, D>(deserializer: D) -> Result<Box<Self>, D::Error>
      where
        D: serde::Deserializer<'de>,
        Self: Sized,
    {
      serde::de::Deserialize::deserialize(deserializer).map(Box::new)
    }
  }

  impl Sealed for str {}
  impl<T: serde::de::DeserializeOwned> Sealed for [T] {}

  /// Rust currently doesn't support returning Self traitobjects from
	/// traitobject methods. Work around that by returning a thin pointer and
	/// fattening it.
  /// https://github.com/alecmocatta/serde_traitobject/blob/master/src/lib.rs#L330C2-L332C19
  #[allow(clippy::module_name_repetitions)]
  #[inline]
  pub fn deserialize_erased<T: ?Sized>(
    self_: *const T,
    deserializer: &mut dyn erased_serde::Deserializer<'_>,
  ) -> Result<Box<T>, erased_serde::Error>
  where
    T: Sealed,
  {
    self_.deserialize_erased(deserializer).map(|raw| {
      println!("raw: {:?}", raw);
      unimplemented!()

      /*
      let object: *mut T = metatype::Type::fatten(raw.as_ptr(), metatype::Type::meta(self_));
      unsafe { Box::from_raw(object) }

      unsafe { Box::from_raw(self_) }
      */
    })
  }
}

struct Deserializer<T: Deserialize + ?Sized + 'static>(PhantomData<T>);
trait DeserializerTrait<T: Deserialize + ?Sized> {
  fn deserialize<'de, D>(deserializer: D) -> Result<Box<T>, D::Error>
    where D: serde::Deserializer<'de>;
}

impl<T: Deserialize> DeserializerTrait<T> for Deserializer<T> {
  #[inline]
  fn deserialize<'de, D>(deserializer: D) -> Result<Box<T>, D::Error>
    where D: serde::Deserializer<'de>
  {
    <T as deserialize::Sealed>::deserialize_box(deserializer)
  }
}

impl DeserializerTrait<str> for Deserializer<str> {
  #[inline]
  fn deserialize<'de, D>(deserializer: D) -> Result<Box<str>, D::Error>
    where D: serde::Deserializer<'de>
  {
    serde::de::Deserialize::deserialize(deserializer)
  }
}

impl<T: serde::de::DeserializeOwned> DeserializerTrait<[T]> for Deserializer<[T]> {
  #[inline]
  fn deserialize<'de, D>(deserializer: D) -> Result<Box<[T]>, D::Error>
    where D: serde::Deserializer<'de>
  {
    serde::de::Deserialize::deserialize(deserializer)
  }
}

impl<T: Deserialize + ?Sized + 'static> DeserializerTrait<T> for Deserializer<T> {
  #[inline]
  default fn deserialize<'de, D>(deserializer: D) -> Result<Box<T>, D::Error>
    where D: serde::Deserializer<'de>
  {
    deserializer.deserialize_map(TaggedVisitor(PhantomData))
  }
}

struct TaggedVisitor<T: Deserialize + ?Sized + 'static>(PhantomData<T>);
impl<'de, T: Deserialize + ?Sized + 'static> serde::de::Visitor<'de> for TaggedVisitor<T> {
  type Value = Box<T>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "a \"{}\" trait object", std::any::type_name::<T>())
  }

  #[inline]
  fn visit_map<M>(self, mut map: M) -> Result<Box<T>, M::Error>
    where M: serde::de::MapAccess<'de>
  {
    //let map_lookup: MapLookupVisitor<T> = MapLookupVisitor(PhantomData);

    let key:&str = match map.next_key()? {
      Some(value) => Ok(value),
      None => Err(serde::de::Error::invalid_length(0, &self)),
    }?;

    println!("key: {:}", &key);

    let binding = Registry::registry();
    let registry = binding.lock().unwrap();
    let registration = match registry.get_deserializer_by_name(key) {
      Some(deserializer) => Ok(deserializer),
      None => Err(serde::de::Error::unknown_variant(key, &[])),
    }?;

    //println!("what is this? {:?}", registration);

    //let des = DeserializeErased::<T>(registration, PhantomData);
    //let des = DeserializeErased::<T>(registration);

    //let ty: Box<T> = map.next_value_seed(DeserializeErased(registration, PhantomData))?; 

    //println!("ty {:?}", &ty);
    /*
    let type_id = &<T as deserialize::Sealed>::type_id(t0);
    let trait_name = std::any::type_name::<T>();

    let t1: &'static str = match seq.next_element()? {
      Some(value) => Ok(value),
      None => Err(serde::de::Error::invalid_length(0, &self)),
    }?;
    */

    //Ok(ty)
    unimplemented!()
  }
}

struct DeserializeErased<T: Deserialize + ?Sized + 'static>(DeserializeFn<dyn Any>, PhantomData<T>);
impl<'de, T: Deserialize + ?Sized + 'static> serde::de::DeserializeSeed<'de> for DeserializeErased<T> {
  type Value = Box<T>;

  #[inline]
  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: serde::de::Deserializer<'de>,
  {
    let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
    let ptr:Box<dyn Any> = (self.0)(&mut erased).map_err(serde::de::Error::custom)?;
    let boxed = unsafe { std::mem::transmute::<Box<dyn Any>, Box<T>>(ptr) };

    //let ptr = Box::into_raw((self.0)(&mut erased).map_err(serde::de::Error::custom)?);
    //let ty:Box<T> = unsafe { Box::from_raw(ptr as *const () as *mut _) };
    //println!("ptr {:?}", &ty);
    //Ok(ty)
    unimplemented!()
  }
}

/*
struct DeserializeErased<'a, T: Deserialize + ?Sized>(&'a dyn ErasedResource, PhantomData<T>);
impl<'de, T: 'static + Deserialize + ?Sized> serde::de::DeserializeSeed<'de> for DeserializeErased<'de, T> {
  type Value = Box<T>;

  #[inline]
  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: serde::de::Deserializer<'de>,
  {
    let mut erased = <dyn erased_serde::Deserializer<'_>>::erase(deserializer);
    self.0.deserialize(&mut erased).map_err(serde::de::Error::custom)
    //(self.0)(&mut erased).map_err(serde::de::Error::custom)
  }
}
*/

/*
#[doc(hidden)]

struct MapLookupVisitor<T: Deserialize + ?Sized>(PhantomData<T>);

impl<'de, T: Deserialize + ?Sized + 'static> serde::de::Visitor<'de> for MapLookupVisitor<T> {
  type Value = Box<T>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "a \"{}\" trait object", std::any::type_name::<T>())
  }

  #[inline]
  fn visit_str<E>(self, key: &str) -> Result<Self::Value, E>
    where E: serde::de::Error,
  {
    println!("key! {:}", &key);

    let binding = Registry::registry();
    let registry = binding.lock().unwrap();

    let registration = match registry.get_by_name(key) {
      Some(value) => Ok(value),
      None => {
        //let names = registry.get_names();
        Err(serde::de::Error::unknown_variant(key, &[]))
      },
    }?;

    println!("what is this? {:?}", &registration);

    let ty: Box<T> = match 
    /*
    let type_id = &<T as deserialize::Sealed>::type_id(t0);
    let trait_name = std::any::type_name::<T>();

    let t1: &'static str = match seq.next_element()? {
      Some(value) => Ok(value),
      None => Err(serde::de::Error::invalid_length(0, &self)),
    }?;
    */

    unimplemented!()
  }
}

impl<'de, T: Deserialize + ?Sized + 'static> serde::de::DeserializeSeed<'de> for MapLookupVisitor<T> {
  type Value = Box<T>;

  fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: serde::de::Deserializer<'de>,
  {
    deserializer.deserialize_str(self)
  }
}
*/


// ============
// Deserialize
// ============

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

pub fn deserialize<'de, T, B, D>(
  deserializer: D,
) -> Result<B, D::Error>
  where
    T: Deserialize + ?Sized + 'static,
    D: serde::Deserializer<'de>,
    Box<T>: Into<B>,
{
  Deserializer::<T>::deserialize(deserializer).map(<Box<T> as Into<B>>::into)
}
