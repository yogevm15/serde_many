use crate::ast::{Input, SerdeImp};
use crate::Derive;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, DeriveInput, Generics, Result};

pub fn derive_serde(input: DeriveInput, derive: Derive) -> Result<TokenStream> {
    let input = Input::from_syn(&input, derive)?;
    let imps = input.data;

    Ok(quote! {
        const _: () = {
            #(#imps)*
        };
    })
}

impl ToTokens for SerdeImp<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let imp = if let Derive::Serialize = self.derive {
            self.serialize_imp()
        } else {
            self.deserialize_imp()
        };

        tokens.extend(imp)
    }
}

impl SerdeImp<'_> {
    fn serialize_imp(&self) -> TokenStream {
        let data = &self.data;
        let original_name = &self.original_ident;
        let original_name_quoted = self.original_ident.to_string();
        let name = &self.data.ident;
        let marker = &self.marker;
        let (impl_generics, ty_generics, where_clause) = self.data.generics.split_for_impl();
        quote! {
            impl #impl_generics ::serde_many::SerializeMany<#marker> for #original_name #ty_generics #where_clause {
                fn serialize<S: ::serde_many::__private::serde::Serializer>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> {
                    #[derive(::serde_many::__private::serde::Serialize)]
                    #[serde(remote = #original_name_quoted)]
                    #data

                    #name::serialize(self, serializer)
                }
            }
        }
    }

    fn deserialize_imp(&self) -> TokenStream {
        let data = &self.data;
        let original_name = &self.original_ident;
        let original_name_quoted = self.original_ident.to_string();
        let name = &self.data.ident;
        let marker = &self.marker;
        let (_, ty_generics, where_clause) = self.data.generics.split_for_impl();
        let impl_generics = DeImplGenerics(&self.data.generics);
        quote! {
            impl #impl_generics ::serde_many::DeserializeMany<'de, #marker> for #original_name #ty_generics #where_clause {
                fn deserialize<D: ::serde_many::__private::serde::Deserializer<'de>>(deserializer: D) -> ::core::result::Result<#original_name #ty_generics, D::Error> {
                    #[derive(::serde_many::__private::serde::Deserialize)]
                    #[serde(remote = #original_name_quoted)]
                    #data

                    #name::deserialize(deserializer)
                }
            }
        }
    }
}

struct DeImplGenerics<'a>(&'a Generics);

impl ToTokens for DeImplGenerics<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.0.clone();
        generics.params = Some(syn::GenericParam::Lifetime(parse_quote!('de)))
            .into_iter()
            .chain(generics.params)
            .collect();
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}