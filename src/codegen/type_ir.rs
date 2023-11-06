use super::type_path::{TypePath, TypePathBuiltIn};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use scale_info::{form::PortableForm, Type};

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
    Enum(Ident, Vec<(u8, CompositeIR)>),
}

#[derive(Debug, Clone)]
pub struct CompositeIR {
    pub(crate) name: Ident,
    pub(crate) kind: CompositeIRKind,
}

#[derive(Debug, Clone)]
pub enum CompositeIRKind {
    NoFields,
    Named(Vec<(Ident, TypePath)>),
    UnNamed(Vec<TypePath>),
}

impl ToTokens for TypeIR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
