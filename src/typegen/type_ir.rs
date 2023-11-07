use super::{type_params::TypeParameters, type_path::TypePath};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use scale_info::{form::PortableForm, Type};
use syn::Item;

use super::derives::Derives;

#[derive(Debug, Clone)]
pub struct TypeIR {
    pub(crate) type_params: TypeParameters,
    pub(crate) derives: Derives,
    pub(crate) kind: TypeIRKind,
    pub(crate) docs: TokenStream,
}

#[derive(Debug, Clone)]
pub enum TypeIRKind {
    Struct(CompositeIR),
    Enum(EnumIR),
}

impl TypeIR {
    fn ident(&self) -> &Ident {
        match &self.kind {
            TypeIRKind::Struct(e) => &e.name,
            TypeIRKind::Enum(e) => &e.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeIR {
    pub(crate) name: Ident,
    pub(crate) kind: CompositeIRKind,
}

#[derive(Debug, Clone)]
pub struct EnumIR {
    pub(crate) name: Ident,
    pub(crate) variants: Vec<(u8, CompositeIR)>,
}

#[derive(Debug, Clone)]
pub enum CompositeIRKind {
    NoFields,
    Named(Vec<(Ident, IsCompact, TypePath)>),
    Unnamed(Vec<(IsCompact, TypePath)>),
}

#[derive(Debug, Clone)]
pub struct IsCompact(pub bool);

impl IsCompact {
    pub fn compact_attr(&self) -> Option<TokenStream> {
        self.0.then(|| quote!( #[codec(compact)] ))
    }
}

impl ToTokens for TypeIR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let derives = &self.derives;
        let docs = &self.docs;
        let type_params = &self.type_params;
        let ident = self.ident();

        match &self.kind {
            TypeIRKind::Struct(composite_ir) => {
                let phantom_data = self.type_params.unused_params_phantom_data();
                let fields = composite_ir.struct_field_tokens(phantom_data);
                let trailing_semicolon = matches!(
                    composite_ir.kind,
                    CompositeIRKind::NoFields | CompositeIRKind::Unnamed(_)
                )
                .then(|| quote!(;));

                let tokenstream = quote! {
                    #derives
                    #docs
                    pub struct #ident #type_params #fields #trailing_semicolon
                };
                tokens.extend(tokenstream);
            }
            TypeIRKind::Enum(enum_ir) => {
                let mut variants = enum_ir
                    .variants
                    .iter()
                    .map(|(index, composite)| {
                        let index = proc_macro2::Literal::u8_unsuffixed(*index);
                        let ident = &composite.name;
                        let fields = composite.enum_field_tokens();
                        quote! {
                            #[codec(index = #index)]
                            #ident #fields
                        }
                    })
                    .collect::<Vec<_>>();

                if let Some(phantom) = self.type_params.unused_params_phantom_data() {
                    variants.push(quote! {
                        #[codec(skip)]
                        __Ignore(#phantom)
                    })
                }

                let tokenstream = quote! {
                    #derives
                    #docs
                    pub enum #ident #type_params {
                        #( #variants, )*
                    }
                };
                tokens.extend(tokenstream);
            }
        }
    }
}

impl CompositeIR {
    fn struct_field_tokens(&self, phantom_data: Option<syn::TypePath>) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => {
                if let Some(phantom_data) = phantom_data {
                    quote! { ( #phantom_data ) }
                } else {
                    quote! {}
                }
            }
            CompositeIRKind::Named(fields) => {
                let fields = fields.iter().map(|(name, is_compact, ty)| {
                    let compact_attr = is_compact.compact_attr();
                    quote! { #compact_attr pub #name: #ty }
                });
                let marker = phantom_data.map(|phantom_data| {
                    quote!(
                        #[codec(skip)]
                        pub __ignore: #phantom_data
                    )
                });
                quote!(
                    {
                        #( #fields, )*
                        #marker
                    }
                )
            }
            CompositeIRKind::Unnamed(fields) => {
                let fields = fields.iter().map(|(is_compact, ty)| {
                    let compact_attr = is_compact.compact_attr();
                    quote! { #compact_attr pub #ty }
                });
                let marker = phantom_data.map(|phantom_data| {
                    quote!(
                        #[codec(skip)]
                        pub __ignore #phantom_data
                    )
                });
                quote! {
                    (
                        #( #fields, )*
                        #marker
                    )
                }
            }
        }
    }

    fn enum_field_tokens(&self) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => quote! {},
            CompositeIRKind::Named(ref fields) => {
                let fields = fields.iter().map(|(name, is_compact, ty)| {
                    let compact_attr = is_compact.compact_attr();
                    quote! { #compact_attr #name: #ty }
                });
                quote!( { #( #fields, )* } )
            }
            CompositeIRKind::Unnamed(ref fields) => {
                let fields = fields.iter().map(|(is_compact, ty)| {
                    let compact_attr = is_compact.compact_attr();
                    quote! { #compact_attr #ty }
                });
                quote! { ( #( #fields, )* ) }
            }
        }
    }
}
