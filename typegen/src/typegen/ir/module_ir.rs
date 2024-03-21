use std::collections::BTreeMap;

use crate::TypeGeneratorSettings;

use super::type_ir::TypeIR;
use super::ToTokensWithSettings;
use proc_macro2::Span;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use scale_info::form::PortableForm;

/// Represents a Rust `mod`, containing generated types and child `mod`s.
#[derive(Debug, Clone)]
pub struct ModuleIR<'a> {
    /// Name of this module.
    pub name: Ident,
    /// Root module identifier.
    pub root_mod: Ident,
    /// Submodules of this module.
    pub children: BTreeMap<Ident, ModuleIR<'a>>,
    /// Types in this module.
    pub types:
        BTreeMap<scale_info::Path<PortableForm>, (&'a scale_info::Type<PortableForm>, TypeIR)>,
}

impl<'a> ToTokensWithSettings for ModuleIR<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, settings: &TypeGeneratorSettings) {
        let name = &self.name;
        let root_mod = &self.root_mod;
        let modules = self
            .children
            .values()
            .map(|ir| ir.to_token_stream(settings));
        let types = self
            .types
            .values()
            .map(|(_, ir)| ir.to_token_stream(settings))
            .clone();

        tokens.extend(quote! {
            pub mod #name {
                use super::#root_mod;

                #( #modules )*
                #( #types )*
            }
        })
    }
}

impl<'a> ModuleIR<'a> {
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
        self.types.iter().map(|(k, v)| (k, &v.1))
    }

    /// Returns the root `mod` used for resolving type paths.
    pub fn root_mod(&self) -> &Ident {
        &self.root_mod
    }

    /// Recursively creates submodules for the given namespace and returns a mutable reference to the innermost module created this way.
    /// Returns itself, if the namespace is empty.
    pub fn get_or_insert_submodule(&mut self, namespace: &[String]) -> &mut ModuleIR<'a> {
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
