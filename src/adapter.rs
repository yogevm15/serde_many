use core::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{DeserializeMany, SerializeMany};

/// Serde Adapter for Marker-Based Serialization/Deserialization
///
/// `AsSerde` is a wrapper around a type `T` that allows it to be serialized and deserialized
/// using a specific marker type `M`.
///
/// By associating a marker `M` with the wrapped data, `AsSerde` helps to distinguish between
/// different serialization strategies for the same underlying data structure.
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "derive")]
/// # {
/// use serde_many::{AsSerde, DeserializeMany, SerializeMany};
/// use serde::Serialize;
///
/// struct SpecialMarker;
///
/// #[derive(Debug, PartialEq, SerializeMany, DeserializeMany)]
/// #[serde_many(_ = "SpecialMarker")]
/// struct MyData {
///     value: i32,
/// }
/// let my_data = MyData { value: 42 };
/// let wrapped_data = AsSerde::<MyData, SpecialMarker>::new(my_data);
///
/// let serialized = serde_json::to_string(&wrapped_data).unwrap();
/// let my_data = wrapped_data.into_inner();
/// let deserialized = serde_json::from_str::<AsSerde<MyData, SpecialMarker>>(&serialized)
///                     .unwrap()
///                     .into_inner();
/// assert_eq!(my_data, deserialized)
/// # }
/// ```
pub struct AsSerde<T, M> {
    data: T,
    marker: PhantomData<M>,
}

impl<T, M> AsSerde<T, M> {
    /// Wraps a value `T` with the given marker `M`.
    pub fn new(value: T) -> Self {
        value.into()
    }

    /// Consumes the `AsSerde` wrapper and returns the inner value.
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T, M> From<T> for AsSerde<T, M> {
    fn from(value: T) -> Self {
        Self {
            data: value,
            marker: PhantomData,
        }
    }
}

impl<T, M> Serialize for AsSerde<T, M>
where
    T: SerializeMany<M>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T, M> Deserialize<'de> for AsSerde<T, M>
where
    T: DeserializeMany<'de, M>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            data: T::deserialize(deserializer)?,
            marker: PhantomData,
        })
    }
}
