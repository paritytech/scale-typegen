use super::{
    type_path::TypePath,
    unused_type_params::{self, UnusedTypeParams},
};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use scale_info::{form::PortableForm, Type};
use syn::Item;

use super::derives::Derives;

#[derive(Debug, Clone)]
pub struct TypeIR {
    pub(crate) derives: Derives,
    pub(crate) kind: TypeIRKind,
    pub(crate) docs: TokenStream,
}

#[derive(Debug, Clone)]
pub enum TypeIRKind {
    Struct(CompositeIR),
    Enum(EnumIR),
}

#[derive(Debug, Clone)]
pub struct CompositeIR {
    pub(crate) name: Ident,
    pub(crate) kind: CompositeIRKind,
}

impl CompositeIR {
    /// adds a PhantomData field to the composite
    pub fn add_phantom_data(&mut self, unused_type_params: UnusedTypeParams) {
        let phantom_type_path = TypePath::phantom(unused_type_params);
        let kind = &mut self.kind;
        match kind {
            CompositeIRKind::NoFields => *kind = CompositeIRKind::UnNamed(vec![phantom_type_path]),
            CompositeIRKind::Named(fields) => {
                let ident = syn::parse_str::<Ident>("__ignore").unwrap();
                fields.push((ident, phantom_type_path))
            }
            CompositeIRKind::UnNamed(fields) => fields.push(phantom_type_path),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumIR {
    pub(crate) name: Ident,
    pub(crate) variants: Vec<(u8, CompositeIR)>,
}

impl EnumIR {
    pub fn add_phantom_data(&mut self, unused_type_params: UnusedTypeParams) {
        let new_index = self
            .variants
            .iter()
            .map(|e| e.0)
            .max()
            .map(|e| e + 1)
            .unwrap_or(0);
        let phantom_type_path = TypePath::phantom(unused_type_params);
        let phantom_composite = CompositeIR {
            name: syn::parse_str::<Ident>("__Ignore").unwrap(),
            kind: CompositeIRKind::UnNamed(vec![phantom_type_path]),
        };
        self.variants.push((new_index, phantom_composite))
    }
}

#[derive(Debug, Clone)]
pub enum CompositeIRKind {
    NoFields,
    Named(Vec<(Ident, TypePath)>),
    UnNamed(Vec<TypePath>),
}

impl ToTokens for TypeIR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let docs to tokens

        // derives to tokens

        match &self.kind {
            TypeIRKind::Struct(composite_ir) => composite_ir.to_tokens(tokens),
            TypeIRKind::Enum(enum_ir) => {
                // let mut variants = variants
                //     .iter()
                //     .map(|(index, def)| {
                //         let index = proc_macro2::Literal::u8_unsuffixed(*index);
                //         quote! {
                //             #[codec(index = #index)]
                //             #def
                //         }
                //     })
                //     .collect::<Vec<_>>();

                // if let Some(phantom) = self.type_params.unused_params_phantom_data() {
                //     variants.push(quote! {
                //         __Ignore(#phantom)
                //     })
                // }

                // let enum_ident = format_ident!("{}", type_name);
                // let type_params = &self.type_params;
                // let derives = &self.derives;
                // let docs = &self.ty_docs;
                // let ty_toks = quote! {
                //     #derives
                //     #docs
                //     pub enum #enum_ident #type_params {
                //         #( #variants, )*
                //     }
                // };
                // tokens.extend(ty_toks);

                todo!()
            }
        }
    }
}

impl ToTokens for CompositeIR {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}
