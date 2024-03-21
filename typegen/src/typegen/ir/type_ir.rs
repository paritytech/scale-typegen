use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    typegen::{
        ir::ToTokensWithSettings, settings::derives::Derives, type_params::TypeParameters,
        type_path::TypePath,
    },
    TypeGeneratorSettings,
};

use super::ToTokensWithSettingsT;

/// Intermediate Representation of a Rust type.
#[derive(Debug, Clone)]
pub struct TypeIR {
    /// Generic type parameters.
    pub type_params: TypeParameters,
    /// Derived traits for his type.
    pub derives: Derives,
    /// whether or not `#[codec(...)]` attributes should be inserted.
    /// Only makes sense if the derives include `Encode`/`Decode`.
    pub insert_codec_attributes: bool,
    /// Is this type an enum or struct.
    pub kind: TypeIRKind,
}

/// An enum or struct.
#[derive(Debug, Clone)]
pub enum TypeIRKind {
    /// A struct.
    Struct(CompositeIR),
    /// An enum.
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

/// A composite. Could be a struct or a variant of an enum.
#[derive(Debug, Clone)]
pub struct CompositeIR {
    /// Struct name or enum variant name.
    pub name: Ident,
    /// Named, Unnamed or NoFields.
    pub kind: CompositeIRKind,
    /// Docs for the composite.
    pub docs: TokenStream,
}

impl CompositeIR {
    /// Creates a new `CompositeIR`.
    pub fn new(name: Ident, kind: CompositeIRKind, docs: TokenStream) -> Self {
        Self { name, kind, docs }
    }
}

/// A rust enum.
#[derive(Debug, Clone)]
pub struct EnumIR {
    /// Docs for the enum.
    pub(crate) docs: TokenStream,
    pub(crate) name: Ident,
    pub(crate) variants: Vec<(u8, CompositeIR)>,
}

/// Named, Unnamed or NoFields.
#[derive(Debug, Clone)]
pub enum CompositeIRKind {
    /// A zero-sized, empty composite.
    NoFields,
    /// Composite with named fields, e.g. a struct.
    Named(Vec<(Ident, CompositeFieldIR)>),
    /// Composite with unnamed fields, e.g. a tuple.
    Unnamed(Vec<CompositeFieldIR>),
}

impl CompositeIRKind {
    /// Returns true if this composite be compact encoded. This is only true if the composite has exactly one field which could be compact encoded.
    pub fn could_derive_as_compact(&self) -> bool {
        // has to have only a single field:
        let single_field = match self {
            CompositeIRKind::NoFields => return false,
            CompositeIRKind::Named(fields) => {
                if fields.len() != 1 {
                    return false;
                }
                &fields[0].1
            }
            CompositeIRKind::Unnamed(fields) => {
                if fields.len() != 1 {
                    return false;
                }
                &fields[0]
            }
        };
        single_field.type_path.is_uint_up_to_u128()
    }
}

/// A field of a composite.
#[derive(Debug, Clone)]
pub struct CompositeFieldIR {
    /// type path of the field.
    pub type_path: TypePath,
    /// Is this field compact encoded?
    /// Having this as `true` may insert a `#[codec(compact)]` attribute during code generation.
    pub is_compact: bool,
    /// Is this field actually boxed? e.g. `Box<type_path>` instead of just `type_path`.
    pub is_boxed: bool,
}

impl CompositeFieldIR {
    /// Creates a new [`CompositeFieldIR`].
    pub fn new(type_path: TypePath, is_compact: bool, is_boxed: bool) -> Self {
        CompositeFieldIR {
            type_path,
            is_compact,
            is_boxed,
        }
    }

    /// Returns a `#[codec(compact)]` attribute if the field should be compact encoded.
    fn compact_attr(&self) -> Option<TokenStream> {
        self.is_compact.then(|| quote!( #[codec(compact)] ))
    }
}

impl ToTokensWithSettingsT for TypeIR {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, settings: &TypeGeneratorSettings) {
        let derives = &self.derives;
        let type_params = &self.type_params;
        let docs = self.docs();
        let ident = self.ident();

        match &self.kind {
            TypeIRKind::Struct(composite_ir) => {
                let phantom_data = self.type_params.unused_params_phantom_data();
                let fields = composite_ir.struct_field_tokens(
                    phantom_data,
                    self.insert_codec_attributes,
                    settings,
                );
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
                        let variant_docs = &composite.docs;
                        let fields =
                            composite.enum_field_tokens(self.insert_codec_attributes, settings);
                        let codec_index = self
                            .insert_codec_attributes
                            .then(|| quote!(#[codec(index = #index)]));
                        quote! {
                            #codec_index
                            #variant_docs
                            #ident #fields
                        }
                    })
                    .collect::<Vec<_>>();

                if let Some(phantom) = self.type_params.unused_params_phantom_data() {
                    // Note: not sure if a #[codec(skip)] is appropriate here or not.
                    // let codec_skip = self.insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    variants.push(quote! {
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
        settings: &TypeGeneratorSettings,
    ) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => {
                if let Some(phantom_data) = phantom_data {
                    quote! { ( pub #phantom_data ) }
                } else {
                    quote! {}
                }
            }
            CompositeIRKind::Named(fields) => {
                let fields = fields.iter().map(|(name, field)| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    let field = ToTokensWithSettings::new(field, settings);
                    quote! { #compact_attr pub #name: #field }
                });
                let marker = phantom_data.map(|phantom_data| {
                    let codec_skip = insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    quote!(
                        #codec_skip
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
                let fields = fields.iter().map(|field| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    let field = ToTokensWithSettings::new(field, settings);
                    quote! { #compact_attr pub #field }
                });
                let marker = phantom_data.map(|phantom_data| {
                    let codec_skip = insert_codec_attributes.then(|| quote!(#[codec(skip)]));
                    quote!(
                        #codec_skip
                        pub #phantom_data
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

    fn enum_field_tokens(
        &self,
        insert_codec_attributes: bool,
        settings: &TypeGeneratorSettings,
    ) -> TokenStream {
        match &self.kind {
            CompositeIRKind::NoFields => quote! {},
            CompositeIRKind::Named(ref fields) => {
                let fields = fields.iter().map(|(name, field)| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    let field = ToTokensWithSettings::new(field, settings);
                    quote! { #compact_attr #name: #field }
                });
                quote!( { #( #fields, )* } )
            }
            CompositeIRKind::Unnamed(ref fields) => {
                let fields = fields.iter().map(|field| {
                    let compact_attr = field.compact_attr().filter(|_| insert_codec_attributes);
                    let field = ToTokensWithSettings::new(field, settings);
                    quote! { #compact_attr #field }
                });
                quote! { ( #( #fields, )* ) }
            }
        }
    }
}

impl ToTokensWithSettingsT for CompositeFieldIR {
    fn to_tokens(&self, tokens: &mut TokenStream, settings: &TypeGeneratorSettings) {
        let ty_path = &self.type_path.to_syn_type(&settings.alloc_crate_path);
        if self.is_boxed {
            let alloc_path = &settings.alloc_crate_path;
            tokens.extend(quote! { #alloc_path::boxed::Box<#ty_path> })
        } else {
            tokens.extend(quote! { #ty_path })
        }
    }
}
