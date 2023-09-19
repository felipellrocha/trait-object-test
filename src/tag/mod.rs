mod convenience;
mod registry;
mod factory;

pub use convenience::*;

pub mod tagged;

use serde::{Serialize};
use convenience::{Box as TaggedBox};
use registry::Registry;

trait Resource:std::fmt::Debug + tagged::Serialize + factory::Resource {
}

#[derive(Debug, Serialize)]
struct Dog {
  test: u32,
}

impl Resource for Dog {
}

impl factory::Resource for Dog {
}

#[derive(Debug, Serialize)]
struct Person {}

impl Resource for Person {
}

impl factory::Resource for Person {
}

#[derive(Debug, Serialize)]
enum Cat {
  Test1,
  Test2(u8),
}

impl Resource for Cat {
}

impl factory::Resource for Cat {
}

#[derive(Debug, Serialize)]
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
  //println!("Hello, world! {:?}", &test);
  println!("Hello, world! {:}", &serde_json::to_string(&test).unwrap());
}

