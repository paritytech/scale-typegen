use std::collections::{HashMap, HashSet};

use quote::ToTokens;


/// A struct containing the derives that we'll be applying to types;
/// a combination of some common derives for all types, plus type
/// specific derives.
#[derive(Debug, Clone)]
pub struct DerivesRegistry {
    default_derives: Derives,
    specific_type_derives: HashMap<syn::TypePath, Derives>,
}

impl DerivesRegistry {
    /// Creates a new `DerivesRegistry` with no default derives.
    pub fn new() -> Self {
        Self {
            default_derives: Derives::new(),
            specific_type_derives: Default::default(),
        }
    }

    /// Insert derives to be applied to all generated types.
    pub fn extend_for_all(
        &mut self,
        derives: impl IntoIterator<Item = syn::Path>,
        attributes: impl IntoIterator<Item = syn::Attribute>,
    ) {
        self.default_derives.derives.extend(derives);
        self.default_derives.attributes.extend(attributes);
    }

    /// Insert derives to be applied to a specific generated type.
    pub fn extend_for_type(
        &mut self,
        ty: syn::TypePath,
        derives: impl IntoIterator<Item = syn::Path>,
        attributes: impl IntoIterator<Item = syn::Attribute>,
    ) {
        let type_derives = self.specific_type_derives.entry(ty).or_default();
        type_derives.derives.extend(derives);
        type_derives.attributes.extend(attributes);
    }

    /// Returns the derives to be applied to all generated types.
    pub fn default_derives(&self) -> &Derives {
        &self.default_derives
    }

    /// Resolve the derives for a generated type. Includes:
    ///     - The default derives for all types e.g. `scale::Encode, scale::Decode`
    ///     - Any user-defined derives for all types via `generated_type_derives`
    ///     - Any user-defined derives for this specific type
    pub fn resolve(&self, ty: &syn::TypePath) -> Derives {
        let mut resolved_derives = self.default_derives.clone();
        if let Some(specific) = self.specific_type_derives.get(ty) {
            resolved_derives.extend_from(specific.clone());
        }
        resolved_derives
    }
}

/// A struct storing the set of derives and derive attributes that we'll apply
/// to generated types.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Derives {
    derives: HashSet<syn::Path>,
    attributes: HashSet<syn::Attribute>,
}



impl FromIterator<syn::Path> for Derives {
    fn from_iter<T: IntoIterator<Item = syn::Path>>(iter: T) -> Self {
        let derives = iter.into_iter().collect();
        Self {
            derives,
            attributes: HashSet::new(),
        }
    }
}

impl Derives {
    /// Creates an empty instance of `Derives` (with no default derives).
    pub fn new() -> Self {
        Self {
            derives: HashSet::new(),
            attributes: HashSet::new(),
        }
    }

    /// Extend this set of `Derives` from another.
    pub fn extend_from(&mut self, other: Derives) {
        self.derives.extend(other.derives);
        self.attributes.extend(other.attributes);
    }

    /// Extend the set of derives by providing an iterator of paths to derive macros.
    pub fn extend(&mut self, derives: impl Iterator<Item = syn::Path>) {
        for derive in derives {
            self.insert_derive(derive)
        }
    }

    /// Insert a single derive.
    pub fn insert_derive(&mut self, derive: syn::Path) {
        self.derives.insert(derive);
    }

    /// Insert a single attribute to be applied to types.
    pub fn insert_attribute(&mut self, attribute: syn::Attribute) {
        self.attributes.insert(attribute);
    }
}

impl ToTokens for Derives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.derives.is_empty() {
            let mut sorted = self.derives.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #[derive(#( #sorted ),*)]
            })
        }
        if !self.attributes.is_empty() {
            let mut sorted = self.attributes.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #( #sorted )*
            })
        }
    }
}
