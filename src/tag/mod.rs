mod convenience;
mod registry;
mod factory;

pub use convenience::*;

pub mod tagged;

use serde::{Serialize, Deserialize};
use convenience::{Box as TaggedBox};
use registry::Registry;

trait Resource:std::fmt::Debug + tagged::Serialize + tagged::Deserialize + factory::Resource {
}

#[derive(Debug, Serialize, Deserialize)]
struct Dog {
  test: u32,
}

impl Resource for Dog {
}

impl factory::Resource for Dog {
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {}

impl Resource for Person {
}

impl factory::Resource for Person {
}

#[derive(Debug, Serialize, Deserialize)]
enum Cat {
  Test1,
  Test2(u8),
}

impl Resource for Cat {
}

impl factory::Resource for Cat {
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
  one: u32,
  two: u32,
  three: u32,
  beings: Vec<TaggedBox<dyn Resource>>,
}

pub fn test() {
  let test = Test {
    one: 1,
    two: 2,
    three: 3,
    beings: vec![
      Box::new(Dog { test: 0 }),
      Box::new(Person {}),
      Box::new(Cat::Test1),
    ],
  };

  let binding = Registry::registry();
  {
    let mut registry = binding.lock().unwrap();
    registry.register::<Dog>("Dog");
    registry.register::<Person>("Person");
    registry.register::<Cat>("Cat");
  }
  println!("Original! {:?}", &test);

  let serialized = serde_json::to_string(&test).unwrap();
  println!("Serialized! {:}", &serialized);
  let deserialized: Test = serde_json::from_str(&serialized).unwrap();
  println!("Deserialized! {:?}", &deserialized);
}

