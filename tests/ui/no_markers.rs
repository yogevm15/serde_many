use serde_many::{DeserializeMany, SerializeMany};

#[derive(SerializeMany, DeserializeMany)]
#[serde_many()]
struct A;

fn main() {}
