
# Serde Many &emsp; [![Latest Version]][crates.io]
[Latest Version]: https://img.shields.io/crates/v/serde_many.svg
[crates.io]: https://crates.io/crates/serde_many

Serde Many enables multiple serialization/deserialization implementations for the same type.
The design ensures seamless integration with the [serde](https://crates.io/crates/serde) crate.

## Example

```rust
use serde_many::{DeserializeMany, SerializeMany};

/// Marker for the default serde implementation.
struct Default;

/// Marker for a special serde implementation.
struct Special;

#[derive(SerializeMany, DeserializeMany)]
#[serde_many(default = "Default", special = "Special")] // Declaring the implementation markers.
struct Point {
    #[serde(special(rename = "x_value"))]
    x: i32,
    #[serde(special(rename = "y_value"))]
    y: i32,
}

#[test]
fn it_works() {
    let mut serialized = Vec::new();
    let mut serializer = serde_json::Serializer::pretty(&mut serialized);
    let val = Point { x: 0, y: 0 };

    SerializeMany::<Default>::serialize(&val, &mut serializer).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&serialized),
        "{
  \"x\": 0,
  \"y\": 0
}"
    );

    serialized.clear();
    let mut serializer = serde_json::Serializer::pretty(&mut serialized);
    SerializeMany::<Special>::serialize(&val, &mut serializer).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&serialized),
        "{
  \"x_value\": 0,
  \"y_value\": 0
}"
    )
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
