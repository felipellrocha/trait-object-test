use std::any::Any;
use std::sync::{Arc, Mutex};
use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;

use crate::factory::{ErasedResource, Resource, ResourceImpl};

lazy_static! {
  static ref COMPONENTS: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
}

#[derive(Debug, Default)]
pub struct Registry {
  impls: HashMap<TypeId, Box<dyn ErasedResource>>,
  names: BTreeMap<&'static str, TypeId>,
}

impl Registry {
  pub fn new() -> Self {
    Self {
      impls: HashMap::new(),
      names: BTreeMap::new(),
    }
  }

  pub fn get() -> Arc<Mutex<Registry>> {
    COMPONENTS.clone()
  }

  pub fn iter(&self) -> impl Iterator<Item = (TypeId, &dyn ErasedResource)> + '_ {
    self.names
      .iter()
      .filter_map(|(_name, type_id)| self.impls.get_key_value(type_id))
      .map(|(id, component)| (*id, component.as_ref()))
  }

  pub fn register<T: Resource + std::fmt::Debug>(&mut self, name: &'static str) {
    let id = TypeId::of::<T>();
    let def = ResourceImpl::<T>::new(name);
    self.impls.insert(id, Box::new(def));
    self.names.insert(name, id);
  }

  /*
  pub fn register(self) {
    if COMPONENTS.set(self).is_err() {
      panic!("Cannot call register_components more than once");
    }
  }
  */

  pub fn get_component(&self, name: &str) -> Option<&dyn ErasedResource> {
    let type_id = self.names.get(name)?;
    self.impls.get(type_id).map(|v| &**v)
  }

  pub fn get_component_by_id(&self, type_id: TypeId) -> Option<&dyn ErasedResource> {
    self.impls.get(&type_id).map(|v| v.as_ref())
  }
}
