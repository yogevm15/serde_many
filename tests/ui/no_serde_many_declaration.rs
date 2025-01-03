use serde_many::{DeserializeMany, SerializeMany};

#[derive(SerializeMany, DeserializeMany)]
struct A;

fn main() {}
