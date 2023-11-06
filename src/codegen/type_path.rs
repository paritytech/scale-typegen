use proc_macro2::Ident;
use quote::ToTokens;
use syn::parse_quote;

#[derive(Clone, Debug)]
pub enum TypePath {
    Path {
        path: syn::Path,
        generics: Vec<TypePath>,
    },
    Parameter(TypeParameter),
    BuiltIn(TypePathBuiltIn),
    // External(TypePathExternal),
}

#[derive(Clone, Debug)]
pub enum TypePathBuiltIn {
    Vec { of: Box<TypePath> },
    Array { len: usize, of: Box<TypePath> },
    Tuple { elements: Vec<TypePath> },
    Primitive { def: scale_info::TypeDefPrimitive },
}

// #[derive(Clone, Debug)]
// pub enum TypePathExternal {
//     Compact {
//         inner: Box<TypePath>,
//         is_field: bool,
//         /// e.g. `subxt::ext::codec::Compact`,
//         compact_type_path: syn::Path,
//     },
//     BitVec {
//         bit_order_type: Box<TypePath>,
//         bit_store_type: Box<TypePath>,
//         /// e.g. `subxt::utils::bits::DecodedBits`,
//         decoded_bits_type_path: syn::Path,
//         compact_type_path: syn::Path,
//     },
// }

impl TypePath {
    fn to_syn_type(&self) -> syn::Type {
        match &self {
            TypePath::Path { path, generics } => {
                let path = if generics.is_empty() {
                    parse_quote! { #path }
                } else {
                    parse_quote! { #path< #( #generics ),* > }
                };
                syn::Type::Path(path)
            }
            TypePath::Parameter(param) => {
                let ident = &param.name;
                parse_quote!(#ident)
            }
            TypePath::BuiltIn(built_in) => built_in.to_syn_type(),
            // TypePath::External(external) => external.to_syn_type(),
        }
    }
}

impl TypePathBuiltIn {
    fn to_syn_type(&self) -> syn::Type {
        use scale_info::TypeDefPrimitive::*;
        match self {
            TypePathBuiltIn::Vec { of } => {
                let type_path = parse_quote! { ::std::vec::Vec<#of> };
                syn::Type::Path(type_path)
            }
            TypePathBuiltIn::Array { len, of } => {
                let array = parse_quote! { [#of; #len] };
                syn::Type::Array(array)
            }
            TypePathBuiltIn::Tuple { elements } => {
                let tuple = parse_quote! { (#( # elements, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypePathBuiltIn::Primitive { def } => syn::Type::Path(match def {
                Bool => parse_quote!(::core::primitive::bool),
                Char => parse_quote!(::core::primitive::char),
                Str => parse_quote!(::std::string::String),
                U8 => parse_quote!(::core::primitive::u8),
                U16 => parse_quote!(::core::primitive::u16),
                U32 => parse_quote!(::core::primitive::u32),
                U64 => parse_quote!(::core::primitive::u64),
                U128 => parse_quote!(::core::primitive::u128),
                U256 => unimplemented!("not a rust primitive"),
                I8 => parse_quote!(::core::primitive::i8),
                I16 => parse_quote!(::core::primitive::i16),
                I32 => parse_quote!(::core::primitive::i32),
                I64 => parse_quote!(::core::primitive::i64),
                I128 => parse_quote!(::core::primitive::i128),
                I256 => unimplemented!("not a rust primitive"),
            }),
        }
    }
}

// impl TypePathExternal {
//     fn to_syn_type(&self) -> syn::Type {
//         match self {
//             TypePathExternal::Compact {
//                 inner,
//                 is_field,
//                 compact_type_path,
//             } => {
//                 let path = if *is_field {
//                     // compact fields can use the inner compact type directly and be annotated with
//                     // the `compact` attribute e.g. `#[codec(compact)] my_compact_field: u128`
//                     parse_quote! ( #inner )
//                 } else {
//                     parse_quote! ( #compact_type_path<#inner> )
//                 };
//                 syn::Type::Path(path)
//             }
//             TypePathExternal::BitVec {
//                 bit_order_type,
//                 bit_store_type,
//                 decoded_bits_type_path,
//             } => {
//                 let type_path =
//                     parse_quote! { #decoded_bits_type_path<#bit_store_type, #bit_order_type> };
//                 syn::Type::Path(type_path)
//             }
//         }
//     }
// }

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeParameter {
    pub concrete_type_id: u32,
    pub original_name: String,
    pub name: Ident,
}

impl ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let syn_type = self.to_syn_type();
        syn_type.to_tokens(tokens)
    }
}
