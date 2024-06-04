use std::collections::BTreeMap;

use crate::typegen::settings::substitutes::TryIntoSynPath;
use crate::TypeGeneratorSettings;

use super::type_ir::TypeIR;
use super::ToTokensWithSettings;
use proc_macro2::Span;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use scale_info::form::PortableForm;

/// Represents a Rust `mod`, containing generated types and child `mod`s.
#[derive(Debug, Clone)]
pub struct ModuleIR {
    /// Name of this module.
    pub name: Ident,
    /// Root module identifier.
    pub root_mod: Ident,
    /// Submodules of this module.
    pub children: BTreeMap<Ident, ModuleIR>,
    /// Types in this module.
    pub types: BTreeMap<scale_info::Path<PortableForm>, (u32, TypeIR)>,
}

impl ToTokensWithSettings for ModuleIR {
    fn to_tokens(&self, tokens: &mut TokenStream, settings: &TypeGeneratorSettings) {
        let name = &self.name;
        let root_mod = &self.root_mod;
        let modules = self
            .children
            .values()
            .map(|ir| ir.to_token_stream(settings));
        let types = self
            .types
            .iter()
            .map(|(path, (_, ir))| {
                let parent_path = path.syn_path().map(|mut path| {
                    // add the root module to the parent_path
                    let extension = syn::PathSegment {
                        ident: settings.types_mod_ident.clone(),
                        arguments: syn::PathArguments::None,
                    };
                    let punctuated = {
                        let mut buf = syn::punctuated::Punctuated::new();
                        buf.push_value(extension);
                        buf.push_punct(syn::token::PathSep::default());
                        buf.extend(path.segments);
                        buf
                    };
                    path.segments = punctuated;
                    path
                });
                let res =
                    settings.with_parent_path(parent_path, |settings| ir.to_token_stream(settings));
                res
            })
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
        self.types.iter().map(|(k, v)| (k, &v.1))
    }

    /// Returns the root `mod` used for resolving type paths.
    pub fn root_mod(&self) -> &Ident {
        &self.root_mod
    }

    /// Recursively creates submodules for the given namespace and returns a mutable reference to the innermost module created this way.
    /// Returns itself, if the namespace is empty.
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
