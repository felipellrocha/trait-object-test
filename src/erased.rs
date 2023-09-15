use erased_serde::{Serialize, Serializer};
use std::collections::BTreeMap as Map;
use std::io;

pub fn test() {
    // Construct some serializers.
    let json = &mut serde_json::Serializer::new(io::stdout());

    // The values in this map are boxed trait objects. Ordinarily this would not
    // be possible with serde::Serializer because of object safety, but type
    // erasure makes it possible with erased_serde::Serializer.
    let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Serializer>::erase(json)));

    // These are boxed trait objects as well. Same thing here - type erasure
    // makes this possible.
    let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
    values.insert("vec", Box::new(vec!["a", "b"]));
    values.insert("int", Box::new(65536));

    // Pick a Serializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    // Pick a Serialize out of the values map.
    let value = values.get("vec").unwrap();

    // This line prints `["a","b"]` to stdout.
    value.erased_serialize(format).unwrap();
}
