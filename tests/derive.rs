use serde_many::SerializeMany;
struct Test;
struct Test2;

#[derive(SerializeMany)]
#[serde_many(test = "Test", test2 = "Test2")]
pub struct A {
    a: i32,
    b: u32,
}
