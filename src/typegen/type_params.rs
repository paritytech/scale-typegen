// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::TypeParameter;
use quote::{format_ident, quote};
use scale_info::form::PortableForm;
use std::collections::BTreeSet;

/// Represents the set of generic type parameters for generating a type definition e.g. the `T` in
/// `Foo<T>`.
///
/// Additionally this allows generating a `PhantomData` type for any type params which are unused
/// in the type definition itself.
#[derive(Clone, Debug, Default)]
pub struct TypeParameters {
    params: Vec<TypeParameter>,
    unused: BTreeSet<TypeParameter>,
}

impl TypeParameters {
    /// Create a new [`TypeParameters`] instance.
    pub fn from_scale_info(params: &[scale_info::TypeParameter<PortableForm>]) -> Self {
        let params = params
            .iter()
            .enumerate()
            .filter_map(|(i, tp)| {
                tp.ty.as_ref().map(|ty| {
                    let tp_name = format_ident!("_{}", i);
                    TypeParameter {
                        concrete_type_id: ty.id,
                        original_name: tp.name.clone(),
                        name: tp_name,
                    }
                })
            })
            .collect::<Vec<_>>();

        let unused = params.iter().cloned().collect();
        Self { params, unused }
    }

    /// Construct a [`core::marker::PhantomData`] for the type unused type params.
    pub fn unused_params_phantom_data(&self) -> Option<syn::TypePath> {
        if self.unused.is_empty() {
            return None;
        }
        let params = if self.unused.len() == 1 {
            let param = self
                .unused
                .iter()
                .next()
                .expect("Checked for exactly one unused param");
            quote! { #param }
        } else {
            let params = self.unused.iter();
            quote! { ( #( #params ), * ) }
        };
        Some(syn::parse_quote! {::core::marker::PhantomData<#params> })
    }

    /// Returns the set of type parameters.
    pub fn params(&self) -> &[TypeParameter] {
        &self.params
    }

    /// Returns true if there are any unused type params
    pub fn has_unused_type_params(&self) -> bool {
        !self.unused.is_empty()
    }

    pub fn mark_used(&mut self, param: &TypeParameter) {
        self.unused.remove(param);
    }
}

impl quote::ToTokens for TypeParameters {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.params.is_empty() {
            let params = &self.params;
            tokens.extend(quote! { < #( #params ),* > })
        }
    }
}
