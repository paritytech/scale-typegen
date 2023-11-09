use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::typegen::{
    settings::derives::Derives, type_params::TypeParameters, type_path::TypePath,
};

#[derive(Debug, Clone)]
pub struct TypeIR {
    pub(crate) type_params: TypeParameters,
    pub(crate) derives: Derives,
    pub(crate) insert_codec_attributes: bool,
    pub(crate) kind: TypeIRKind,
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

    fn docs(&self) -> &TokenStream {
        match &self.kind {
            TypeIRKind::Struct(e) => &e.docs,
            TypeIRKind::Enum(e) => &e.docs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeIR {
    pub name: Ident,
    pub kind: CompositeIRKind,
    pub docs: TokenStream,
}

impl CompositeIR {
    pub fn new(name: Ident, kind: CompositeIRKind, docs: TokenStream) -> Self {
        Self { name, kind, docs }
    }
}

#[derive(Debug, Clone)]
pub struct EnumIR {
    pub(crate) docs: TokenStream,
    pub(crate) name: Ident,
    pub(crate) variants: Vec<(u8, CompositeIR)>,
}

#[derive(Debug, Clone)]
pub enum CompositeIRKind {
    NoFields,
    Named(Vec<(Ident, CompositeFieldIR)>),
    Unnamed(Vec<CompositeFieldIR>),
}

#[derive(Debug, Clone)]
pub struct CompositeFieldIR {
    pub type_path: TypePath,
    pub is_compact: bool,
    pub is_boxed: bool,
}

impl CompositeFieldIR {
    pub fn new(type_path: TypePath, is_compact: bool, is_boxed: bool) -> Self {
        CompositeFieldIR {
            type_path,
            is_compact,
            is_boxed,
        }
    }

    pub fn compact_attr(&self) -> Option<TokenStream> {
        self.is_compact.then(|| quote!( #[codec(compact)] ))
    }
}

impl ToTokens for TypeIR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let derives = &self.derives;
        let type_params = &self.type_params;
        let docs = self.docs();
        let ident = self.ident();

        match &self.kind {
            TypeIRKind::Struct(composite_ir) => {
                let phantom_data = self.type_params.unused_params_phantom_data();
                let fields =
                    composite_ir.struct_field_tokens(phantom_data, self.insert_codec_attributes);
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
                        let fields = composite.enum_field_tokens(self.insert_codec_attributes);
                        let codec_index = self
                            .insert_codec_attributes
                            .then(|| quote!(#[codec(index = #index)]));
                        quote! {
                            #codec_index
                            #ident #fields
                        }
                    })
                    .collect::<Vec<_>>();

                if let Some(phantom) = self.type_params.unused_params_phantom_data() {
                    let codec_skip = self.insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    variants.push(quote! {
                        #codec_skip
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
    fn struct_field_tokens(
        &self,
        phantom_data: Option<syn::TypePath>,
        insert_codec_attributes: bool,
    ) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => {
                if let Some(phantom_data) = phantom_data {
                    quote! { ( #phantom_data ) }
                } else {
                    quote! {}
                }
            }
            CompositeIRKind::Named(fields) => {
                let fields = fields.iter().map(|(name, field)| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    quote! { #compact_attr pub #name: #field }
                });
                let marker = phantom_data.map(|phantom_data| {
                    let codec_skip = insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    quote!(
                        #codec_skip
                        __ignore: #phantom_data
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
                let fields = fields.iter().map(|field| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    quote! { #compact_attr pub #field }
                });
                let marker = phantom_data.map(|phantom_data| {
                    let codec_skip = insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    quote!(
                        #codec_skip
                        #phantom_data
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

    fn enum_field_tokens(&self, insert_codec_attributes: bool) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => quote! {},
            CompositeIRKind::Named(ref fields) => {
                let fields = fields.iter().map(|(name, field)| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    quote! { #compact_attr #name: #field }
                });
                quote!( { #( #fields, )* } )
            }
            CompositeIRKind::Unnamed(ref fields) => {
                let fields = fields.iter().map(|field| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    quote! { #compact_attr #field }
                });
                quote! { ( #( #fields, )* ) }
            }
        }
    }
}

impl ToTokens for CompositeFieldIR {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty_path = &self.type_path;
        if self.is_boxed {
            tokens.extend(quote! { ::std::boxed::Box<#ty_path> })
        } else {
            tokens.extend(quote! { #ty_path })
        }
    }
}
