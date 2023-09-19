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

#[derive(Debug, Default)]
pub struct Registry {
  impls: HashMap<TypeId, (&'static str, Box<dyn ErasedResource>)>,
  names: BTreeMap<&'static str, TypeId>,
}

impl Registry {
  pub fn new() -> Self {
    Self {
      impls: HashMap::new(),
      names: BTreeMap::new(),
    }
  }

  pub fn registry() -> Arc<Mutex<Registry>> {
    COMPONENTS.clone()
  }

  pub fn register<T>(&mut self, name: &'static str)
    where T: Resource + std::fmt::Debug,
  {
    let id = TypeId::of::<T>();
    let def = ResourceImpl::<T>::new(name);
    self.impls.insert(id, (name, Box::new(def)));
    self.names.insert(name, id);
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

  pub fn get_component(&self, name: &str) -> Option<&dyn ErasedResource> {
    let type_id = self.names.get(name)?;
    self.impls.get(type_id).map(|v| &**v)
  }
  */

  pub fn get(&self, type_id: TypeId) -> Option<&(&'static str, Box<dyn ErasedResource>)> {
    self.impls.get(&type_id)
  }
}
