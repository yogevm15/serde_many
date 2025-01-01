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
