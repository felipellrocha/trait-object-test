use std::any::Any;
use std::sync::{Arc, Mutex};
use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use lazy_static::lazy_static;

use crate::tag::factory::{ErasedResource, Resource, ResourceImpl};

// TODO: Write another backing structure here. Call it `WriteThenRead` (or something more creative), 
// that would allow us to write to it only until a `lock()` method is called. After that, writing
// panics, and only reads are allowed. But, at that point, one should be able to read concurrently.
lazy_static! {
  static ref COMPONENTS: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
}

pub type DeserializeFn<T> = fn(&mut dyn erased_serde::Deserializer<'_>) -> erased_serde::Result<Box<T>>;

#[derive(Default)]
pub struct Registry {
  impls: HashMap<TypeId, Box<dyn ErasedResource>>,
  deserializers: HashMap<TypeId, DeserializeFn<dyn Any>>,
  names: BTreeMap<&'static str, TypeId>,
  type_ids: BTreeMap<TypeId, &'static str>,
}

impl Registry {
  pub fn new() -> Self {
    Self {
      impls: HashMap::new(),
      deserializers: HashMap::new(),
      names: BTreeMap::new(),
      type_ids: BTreeMap::new(),
    }
  }

  pub fn registry() -> Arc<Mutex<Registry>> {
    COMPONENTS.clone()
  }

  pub fn register<T>(&mut self, name: &'static str)
    where T: Resource + std::fmt::Debug + serde::de::DeserializeOwned,
  {
    let id = TypeId::of::<T>();
    let def = ResourceImpl::<T>::new(name);
    self.impls.insert(id, Box::new(def));
    self.names.insert(name, id);
    self.deserializers.insert(id, (|deserializer| {
      //T::deserialize(deserializer).map(|item| Box::new(item) as Box<dyn Any>)
      Ok(erased_serde::deserialize::<T>(deserializer).map(|item| Box::new(item) as Box<dyn Any>)?)
    }) as DeserializeFn<dyn Any>);
    self.type_ids.insert(id, name);
  }

  pub fn get_names(&self) -> Vec<&'static str> {
    let mut names = vec![];
    for (name, _) in &self.names {
      names.push(name.clone());
    }
    names
  }

  /*
  pub fn iter(&self) -> impl Iterator<Item = (TypeId, &dyn ErasedResource)> + '_ {
    self.names
      .iter()
      .filter_map(|(_name, type_id)| self.impls.get_key_value(type_id))
      .map(|(id, component)| (*id, component.as_ref()))
  }

  pub fn register(self) {
    if COMPONENTS.set(self).is_err() {
      panic!("Cannot call register_components more than once");
    }
  }

  */
  pub fn get_by_name(&self, name: &str) -> Option<&dyn ErasedResource> {
    let type_id = self.names.get(name)?;
    self.impls.get(type_id).map(|v| &**v)
  }

  pub fn get_name(&self, type_id: TypeId) -> Option<&'static str> {
    self.type_ids.get(&type_id).cloned()
  }

  pub fn get_deserializer(&self, type_id: TypeId) -> Option<DeserializeFn<dyn Any>> {
    self.deserializers.get(&type_id).cloned()
  }

  pub fn get_deserializer_by_name(&self, name: &str) -> Option<DeserializeFn<dyn Any>> {
    let type_id = self.names.get(name)?;
    self.deserializers.get(&type_id).cloned()
  }

  pub fn get(&self, type_id: TypeId) -> Option<&dyn ErasedResource> {
    self.impls.get(&type_id).map(|v| v.as_ref())
  }
}

pub trait Strictest {
  type Object: ?Sized;
}
