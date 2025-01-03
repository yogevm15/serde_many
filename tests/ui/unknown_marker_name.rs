use serde_many::{DeserializeMany, SerializeMany};

struct Default;

#[derive(SerializeMany, DeserializeMany)]
#[serde_many(default = "Default")]
#[serde(foo(aaa))]
struct A(i32);

fn main() {}