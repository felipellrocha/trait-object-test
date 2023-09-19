use serde::{Serialize, Deserialize};

#[typetag::serde]
trait Resource:std::fmt::Debug {
}

#[derive(Debug, Serialize, Deserialize)]
struct Dog {
  test: u32,
}

#[typetag::serde]
impl Resource for Dog {
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {}

#[typetag::serde]
impl Resource for Person {
}

#[derive(Debug, Serialize, Deserialize)]
enum Cat {
  Test1,
  Test2(u8),
}

#[typetag::serde]
impl Resource for Cat {
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
  one: u32,
  two: u32,
  three: u32,
  beings: Vec<Box<dyn Resource>>,
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

  //println!("Hello, world! {:?}", &test);
  println!("Hello, world! {:}", &serde_json::to_string(&test).unwrap());
}
