pub mod registry;
pub mod erased;
pub mod global;
pub mod factory;

use serde::{Serialize, Deserialize};
use crate::{
  global::Registry,
  factory::Resource,
};

#[derive(Debug, Serialize, Deserialize)]
struct Dog {
  test: u32,
}
impl Resource for Dog {
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {}
impl Resource for Person {
}

#[derive(Debug, Serialize, Deserialize)]
enum Cat {
  Test1,
  Test2(u8),
}
impl Resource for Cat {
}

/*
#[derive(Debug, Serialize)]
struct Test {
  one: u32,
  two: u32,
  three: u32,
  beings: Vec<Box<dyn Resource>>,
}
*/

fn main() {
  let binding = Registry::get();
  let mut registry = binding.lock().unwrap();
  registry.register::<Dog>("Dog");
  registry.register::<Person>("Person");
  registry.register::<Cat>("Cat");
  println!("registry: {:?}", &registry);

  /*
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

  println!("Hello, world! {:?}", &test);
  println!("Hello, world! {:?}", &serde_json::to_string(&test).unwrap());
  */
}
