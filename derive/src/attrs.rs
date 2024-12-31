use proc_macro2::Span;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use syn::{Attribute, Ident, LitStr, Path, Result};

pub struct ManyAttrs {
    pub many: HashMap<Ident, Path>,
}

impl ManyAttrs {
    pub fn from_syn(input: &[Attribute]) -> Result<Self> {
        let mut attrs = ManyAttrs {
            many: HashMap::new(),
        };

        for attr in input {
            if attr.path().is_ident("serde_many") {
                attr.parse_nested_meta(|m| {
                    let ident = m.path.require_ident()?.clone();
                    match attrs.many.entry(ident) {
                        Entry::Occupied(o) => {
                            return Err(syn::Error::new(
                                o.key().span(),
                                format!("Duplicate key detected: {}", o.key()),
                            ))
                        }
                        Entry::Vacant(entry) => {
                            let parsed_value = m.value()?.parse::<LitStr>()?.parse()?;
                            entry.insert(parsed_value);
                        }
                    }
                    Ok(())
                })?;
            }
        }

        if attrs.many.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "No `serde_many` attributes found",
            ));
        }

        Ok(attrs)
    }
}
