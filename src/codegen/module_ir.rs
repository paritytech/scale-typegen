use std::collections::BTreeMap;

use crate::Settings;

use super::type_ir::TypeIR;
use proc_macro2::Span;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use quote::ToTokens;
use scale_info::form::PortableForm;

/// Represents a Rust `mod`, containing generated types and child `mod`s.
#[derive(Debug, Clone)]
pub struct ModuleIR {
    pub name: Ident,
    pub root_mod: Ident,
    pub children: BTreeMap<Ident, ModuleIR>,
    pub types: BTreeMap<scale_info::Path<PortableForm>, TypeIR>,
}

impl ToTokens for ModuleIR {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let root_mod = &self.root_mod;
        let modules = self.children.values();
        let types = self.types.values().clone();

        tokens.extend(quote! {
            pub mod #name {
                use super::#root_mod;

                #( #modules )*
                #( #types )*
            }
        })
    }
}

impl ModuleIR {
    /// Create a new [`Module`], with a reference to the root `mod` for resolving type paths.
    pub(crate) fn new(name: Ident, root_mod: Ident) -> Self {
        Self {
            name,
            root_mod,
            children: BTreeMap::new(),
            types: BTreeMap::new(),
        }
    }

    /// Returns the module ident.
    pub fn ident(&self) -> &Ident {
        &self.name
    }

    /// Returns this `Module`s child `mod`s.
    pub fn children(&self) -> impl Iterator<Item = (&Ident, &ModuleIR)> {
        self.children.iter()
    }

    /// Returns the generated types.
    pub fn types(&self) -> impl Iterator<Item = (&scale_info::Path<PortableForm>, &TypeIR)> {
        self.types.iter()
    }

    /// Returns the root `mod` used for resolving type paths.
    pub fn root_mod(&self) -> &Ident {
        &self.root_mod
    }

    pub fn get_or_insert_submodule(&mut self, namespace: &[String]) -> &mut ModuleIR {
        if namespace.is_empty() {
            return self;
        }
        let child_ident = Ident::new(&namespace[0], Span::call_site());
        let child = self
            .children
            .entry(child_ident.clone())
            .or_insert_with(|| ModuleIR::new(child_ident, self.root_mod.clone()));
        return child.get_or_insert_submodule(&namespace[1..]);
    }
}
