use std::marker::PhantomData;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use erased_serde::{Serializer};
use erased_serde::serialize_trait_object;

#[derive(Serialize)]
struct ErasedBeing {
  name: &'static str,
  value: Box<&'static dyn Being>,
}

trait Being:'static + Send + Sync + Debug + erased_serde::Serialize {
  fn say(&self);
}

//erased_serde::serialize_trait_object!(Being);

#[derive(Debug, Serialize, Deserialize)]
struct Dog {
  test: u32,
}
impl Being for Dog {
  fn say(&self) {
    println!("Woof");
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {}
impl Being for Person {
  fn say(&self) {
    println!("A lot of words");
  }
}

#[derive(Debug, Serialize, Deserialize)]
enum Cat {
  Test1,
  Test2(u8),
  //Test3(u8, u8, u8, u8),
  //Test4{ a: u8, b: u8, c: u8, d: u8 },
}
impl Being for Cat {
  fn say(&self) {
    println!("Meow");
  }
}


#[derive(Debug, Serialize)]
struct Test {
  one: u32,
  two: u32,
  three: u32,
  beings: Vec<Box<dyn Being>>,
}

pub fn test() {
  use serde_json::Serializer;
  //let json_serializer = Serializer::
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
}
