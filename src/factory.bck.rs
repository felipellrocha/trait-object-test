use std::marker::PhantomData;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use erased_serde::{Deserializer, Serialize as _, Serializer};

pub trait ErasedFactory: Send + Sync + 'static {
  fn create(&self) -> Box<dyn GameComponent>;
}

pub trait GameComponent {
  fn say(&self);
}

pub struct FactoryImpl<T> {
  name: &'static str,
  _phantom: PhantomData<*const T>,
}

unsafe impl<T> Send for FactoryImpl<T> {}
unsafe impl<T> Sync for FactoryImpl<T> {}

impl<T> FactoryImpl<T> {
  pub fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: PhantomData,
    }
  }
}

impl<T> ErasedFactory for FactoryImpl<T>
  where
    T: Serialize + for<'a> Deserialize<'a> + 'static
{
  fn create(&self) -> Box<dyn GameComponent> {
    Box::new(T)
  }
}


/*
#[derive(Debug, Serialize, Deserialize)]
struct Cow;

impl GameComponent for Cow {
  fn say(&self) {
    println!("Moo");
  }
}

#[derive(Default)]
struct CowFactory;

impl ErasedFactory for CowFactory {
  fn create(&self) -> Box<dyn GameComponent> {
    Box::new(Cow)
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Horse;

impl GameComponent for Horse {
  fn say(&self) {
    println!("Neigh");
  }
}

#[derive(Default)]
struct HorseFactory;

impl ErasedFactory for HorseFactory {
  fn create(&self) -> Box<dyn GameComponent> {
    Box::new(Horse)
  }
}
*/

/*
fn main() {
  let mut registry: HashMap<&str, Box<dyn ErasedFactory>> = HashMap::new();
  registry.insert("Horse", Box::new(HorseErasedFactory));
  registry.insert("Cow", Box::new(CowErasedFactory));

  let mut animal = registry["Horse"].create();
  animal.say();

  animal = registry["Cow"].create();
  animal.say();
}
*/
