use erased_serde::serialize_trait_object;
use erased_serde::Deserializer;
use erased_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_traitobject;
use std::fmt::Debug;
use std::marker::PhantomData;

trait Being:
    'static + Send + Sync + Debug + serde_traitobject::Serialize + serde_traitobject::Deserialize
{
    fn say(&self);
}

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

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    one: u32,
    two: u32,
    three: u32,
    beings: Vec<serde_traitobject::Box<dyn Being>>,
}

pub fn test() {
    //let json_serializer = Serializer::
    let mut test = Test {
        one: 1,
        two: 2,
        three: 3,
        beings: vec![
            serde_traitobject::Box::new(Dog { test: 0 }),
            serde_traitobject::Box::new(Person {}),
            serde_traitobject::Box::new(Cat::Test1),
        ],
    };

    println!("Original! {:?}", &test);
    test.beings[0] = serde_traitobject::Box::new(Dog { test: 1 });
    let serialized = serde_json::to_string(&test).unwrap();
    println!("\nSerialized {:?}", &serialized);
    let deserialized: serde_traitobject::Box<Test> = serde_json::from_str(&serialized).unwrap();
    println!("\nDeserialized {:?}", (&deserialized));
}
