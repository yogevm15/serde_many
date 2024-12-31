mod ast;
mod attrs;
mod expand;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[derive(Copy, Clone)]
enum Derive {
    Serialize,
    Deserialize,
}

#[proc_macro_derive(SerializeMany, attributes(serde_many, serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand::derive_serde(input, Derive::Serialize).unwrap_or_else(|e| e.to_compile_error().into())
}

#[proc_macro_derive(DeserializeMany, attributes(serde_many, serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand::derive_serde(input, Derive::Deserialize).unwrap_or_else(|e| e.to_compile_error().into())
}
