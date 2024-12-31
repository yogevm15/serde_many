//! # Serde Many
//!
//! Serde Many enables multiple serialization/deserialization implementations for the same type.
//!
//! The design ensures seamless integration with the [serde] crate.
//!
//! # Design
//!
//! The core design of this crate revolves around the [`SerializeMany`] and [`DeserializeMany`] traits.
//! These traits are similar to the [serde] [`Serialize`] and [`Deserialize`] traits,
//! but are generic over a marker type, allowing multiple implementations for different markers.
//!
//! To ensure seamless integration with [serde], any type that implements [serde]'s
//! [`Serialize`] and [`Deserialize`] automatically implements
//! [`SerializeMany`] and [`DeserializeMany`] for all markers. This means that types which
//! manually implement [`SerializeMany`] and [`DeserializeMany`] cannot also implement
//! [serde]'s [`Serialize`] and [`Deserialize`].
//!
//! # Derive
//!
//! Implementing serialization and deserialization by hand can be tedious. To simplify this process,
//! this crate (with the `derive` feature) provides derive macros to automatically generate implementations
//! of the [`SerializeMany`] and [`DeserializeMany`] traits.
//!
//! The derive macros use the actual [serde] derive macros under the hood, meaning all of
//! [serde]'s attributes are supported.
//!
//! # Example
//! ```
//! use serde_many::{DeserializeMany, SerializeMany};
//!
//! /// Marker for the default serde implementation.
//! struct Default;
//!
//! /// Marker for a special serde implementation.
//! struct Special;
//!
//! #[derive(SerializeMany, DeserializeMany)]
//! #[serde_many(default = "Default", special = "Special")] // Declaring the implementation markers.
//! struct Point {
//!     #[serde(special(rename = "x_value"))]
//!     x: i32,
//!     #[serde(special(rename = "y_value"))]
//!     y: i32,
//! }
//! ```
#![no_std]
pub use adapter::AsSerde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod adapter;

#[cfg(feature = "derive")]
pub use serde_many_derive::{DeserializeMany, SerializeMany};

/// A trait for serializing a value with a specific marker type.
pub trait SerializeMany<M> {
    /// Serializes the value with the given serializer.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

/// A trait for deserializing a value with a specific marker type.
pub trait DeserializeMany<'de, M>: Sized {
    /// Deserializes the value with the given deserializer.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}

// Default implementation of `SerializeMany` for any type that implements `Serialize`.
impl<T: Serialize + ?Sized, M> SerializeMany<M> for T {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(self, serializer)
    }
}

// Default implementation of `DeserializeMany` for any type that implements `Deserialize`.
impl<'de, T: Deserialize<'de>, M> DeserializeMany<'de, M> for T {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer)
    }
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use serde;
}
