use crate::attrs::ManyAttrs;
use crate::Derive;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_derive_internals::Ctxt;
use std::collections::HashMap;
use syn::visit::Visit;
use syn::visit_mut::visit_field_mut;
use syn::{
    parenthesized, parse_quote, visit_mut::VisitMut, Attribute, DeriveInput, Error, Field, Ident,
    Path, Result, Variant,
};

pub struct SerdeImp<'a> {
    pub marker: Path,
    pub derive: Derive,
    pub original_ident: &'a Ident,
    pub data: DeriveInput,
}

pub struct Input<'a> {
    pub data: Vec<SerdeImp<'a>>,
}

impl<'a> Input<'a> {
    pub fn from_syn(i: &'a DeriveInput, derive: Derive) -> Result<Self> {
        let many = ManyAttrs::from_syn(&i.attrs)?.many;
        let mut visitor = AttrValidateVisitor::new(&many);
        visitor.visit_derive_input(i);
        visitor.result()?;
        let data = ManyAttrs::from_syn(&i.attrs)?
            .many
            .into_iter()
            .map(|(name, marker)| SerdeImp::from_syn(name, marker, i, derive))
            .collect::<Result<_>>()?;
        Ok(Self { data })
    }
}

impl<'a> SerdeImp<'a> {
    fn from_syn(name: Ident, marker: Path, i: &'a DeriveInput, derive: Derive) -> Result<Self> {
        let mut visitor = SerdeImpMutate::new(name, derive, &marker);
        let mut input = i.clone();
        visitor.visit_derive_input_mut(&mut input);

        visitor.result()?;

        input.ident = Ident::new("Duplicate", Span::call_site());

        Ok(Self {
            marker,
            derive,
            original_ident: &i.ident,
            data: input,
        })
    }
}

struct SerdeImpMutate<'a> {
    name: Ident,
    derive: Derive,
    marker: &'a Path,
    curr_variant_attrs: Option<serde_derive_internals::attr::Variant>,
    container_attrs: Option<serde_derive_internals::attr::Container>,
    ctx: Ctxt,
}

impl<'a> SerdeImpMutate<'a> {
    fn new(name: Ident, derive: Derive, marker: &'a Path) -> Self {
        Self {
            name,
            derive,
            marker,
            curr_variant_attrs: None,
            container_attrs: None,
            ctx: Ctxt::new(),
        }
    }

    fn result(self) -> Result<()> {
        self.ctx.check()
    }

    fn serde_with_attr(&self) -> Attribute {
        let marker = self.marker;
        if let Derive::Serialize = self.derive {
            let quote = quote! {::serde_many::SerializeMany::<#marker>::serialize}.to_string();
            parse_quote!(#[serde(serialize_with = #quote)])
        } else {
            let quote = quote! {::serde_many::DeserializeMany::<#marker>::deserialize}.to_string();
            parse_quote!(#[serde(deserialize_with = #quote)])
        }
    }
}

impl VisitMut for SerdeImpMutate<'_> {
    fn visit_attributes_mut(&mut self, i: &mut Vec<Attribute>) {
        let mut attributes = vec![];
        for attr in i.iter() {
            if !attr.path().is_ident("serde") {
                continue;
            }

            let res = attr.parse_nested_meta(|m| {
                let content;
                parenthesized!(content in m.input);
                let content: TokenStream = content.parse()?;
                if m.path.is_ident(&self.name) {
                    attributes.push(parse_quote!(#[serde(#content)]))
                }

                Ok(())
            });

            if let Err(e) = res {
                self.ctx.syn_error(e)
            }
        }

        *i = attributes;
    }

    fn visit_derive_input_mut(&mut self, i: &mut DeriveInput) {
        self.visit_attributes_mut(&mut i.attrs);
        self.container_attrs = Some(serde_derive_internals::attr::Container::from_ast(
            &self.ctx, i,
        ));
        self.visit_visibility_mut(&mut i.vis);
        self.visit_ident_mut(&mut i.ident);
        self.visit_generics_mut(&mut i.generics);
        self.visit_data_mut(&mut i.data);
    }

    fn visit_field_mut(&mut self, i: &mut Field) {
        visit_field_mut(self, i);
        if serde_derive_internals::attr::Field::from_ast(
            &self.ctx,
            0,
            i,
            self.curr_variant_attrs.as_ref(),
            self.container_attrs.as_ref().unwrap().default(),
        )
        .serialize_with()
        .is_none()
        {
            i.attrs.push(self.serde_with_attr())
        }
    }

    fn visit_variant_mut(&mut self, i: &mut Variant) {
        self.visit_attributes_mut(&mut i.attrs);
        self.curr_variant_attrs = Some(serde_derive_internals::attr::Variant::from_ast(
            &self.ctx, i,
        ));
        self.visit_ident_mut(&mut i.ident);
        self.visit_fields_mut(&mut i.fields);
        if let Some(it) = &mut i.discriminant {
            self.visit_expr_mut(&mut it.1);
        }
    }
}

struct AttrValidateVisitor<'a> {
    ctx: Ctxt,
    many: &'a HashMap<Ident, Path>,
}

impl<'a> AttrValidateVisitor<'a> {
    fn new(many: &'a HashMap<Ident, Path>) -> Self {
        Self {
            many,
            ctx: Ctxt::new(),
        }
    }

    fn result(self) -> Result<()> {
        self.ctx.check()
    }
}

impl Visit<'_> for AttrValidateVisitor<'_> {
    fn visit_attribute(&mut self, i: &Attribute) {
        if !i.path().is_ident("serde") {
            return;
        }
        let res = i.parse_nested_meta(|m| {
            let content;
            parenthesized!(content in m.input);
            let _: TokenStream = content.parse()?;
            if !self.many.contains_key(m.path.require_ident()?) {
                return Err(Error::new_spanned(
                    m.path,
                    "Unknown marker name, have you forgotten to add it to `#[serde_many(...)]`?",
                ));
            }

            Ok(())
        });
        if let Err(e) = res {
            self.ctx.syn_error(e)
        }
    }
}
