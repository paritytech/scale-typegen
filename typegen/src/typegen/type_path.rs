// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens};
use scale_info::{form::PortableForm, Path, TypeDefPrimitive};
use std::collections::BTreeSet;
use syn::parse_quote;

use crate::TypeGeneratorSettings;

use super::ir::ToTokensWithSettings;
use super::settings::AllocCratePath;

/// An opaque struct representing a type path. The main usage of this is
/// to spit out as tokens in some `quote!{ ... }` macro; the inner structure
/// should be unimportant.
#[derive(Clone, Debug)]
pub struct TypePath(TypePathInner);

/// The type path to either a concrete type or a generic type parameter
#[derive(Clone, Debug)]
pub enum TypePathInner {
    /// Generic type parameter
    Parameter(TypeParameter),
    /// Concrete type
    Type(TypePathType),
}

impl ToTokensWithSettings for TypePath {
    fn to_tokens(&self, tokens: &mut TokenStream, settings: &TypeGeneratorSettings) {
        let syn_type = self.to_syn_type(&settings.alloc_crate_path);
        syn_type.to_tokens(tokens)
    }
}

impl TypePath {
    /// Construct a [`TypePath`] from a [`TypeParameter`]
    pub fn from_parameter(param: TypeParameter) -> TypePath {
        TypePath(TypePathInner::Parameter(param))
    }

    /// Construct a [`TypePath`] from a [`TypeParameter`]
    pub fn from_type(ty: TypePathType) -> TypePath {
        TypePath(TypePathInner::Type(ty))
    }

    /// Construct a [`TypePath`] from a [`syn::TypePath`]
    pub fn from_syn_path(path: syn::Path) -> TypePath {
        // Note; this doesn't parse the parameters or anything, but since nothing external
        // can inspect this structure, and the ToTokens impl works either way, it should be ok.
        TypePath(TypePathInner::Type(TypePathType::Path {
            path,
            params: Vec::new(),
        }))
    }

    pub(crate) fn to_syn_type(&self, alloc_crate_path: &AllocCratePath) -> syn::Type {
        match &self.0 {
            TypePathInner::Parameter(ty_param) => syn::Type::Path(parse_quote! { #ty_param }),
            TypePathInner::Type(ty) => ty.to_syn_type(alloc_crate_path),
        }
    }

    /// Returns true, if this is a concrete compact type.
    pub fn is_compact(&self) -> bool {
        matches!(&self.0, TypePathInner::Type(ty) if ty.is_compact())
    }

    /// Returns true, if this is a concrete string type.
    pub fn is_string(&self) -> bool {
        matches!(&self.0, TypePathInner::Type(ty) if ty.is_string())
    }

    /// Returns true, if this is an unsigned integer (anywhere between u8 and u128).
    pub fn is_uint_up_to_u128(&self) -> bool {
        matches!(
            &self.0,
            TypePathInner::Type(TypePathType::Primitive {
                def: TypeDefPrimitive::U8
                    | TypeDefPrimitive::U16
                    | TypeDefPrimitive::U32
                    | TypeDefPrimitive::U64
                    | TypeDefPrimitive::U128
            })
        )
    }

    /// Returns the type parameters in a path which are inherited from the containing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: Vec<Option<T>>, // the parent type param here is `T`
    /// }
    /// ```
    pub fn parent_type_params(&self) -> BTreeSet<TypeParameter> {
        let mut acc = BTreeSet::<TypeParameter>::new();
        self.parent_type_params_recurse(&mut acc);
        acc
    }

    fn parent_type_params_recurse(&self, acc: &mut BTreeSet<TypeParameter>) {
        match &self.0 {
            TypePathInner::Parameter(type_parameter) => {
                acc.insert(type_parameter.clone());
            }
            TypePathInner::Type(type_path) => type_path.parent_type_params(acc),
        }
    }

    /// Gets the vector type parameter if the data is represented as `TypeDef::Sequence`.
    ///
    /// **Note:** Utilized for transforming `std::vec::Vec<T>` into slices `&[T]` for the storage API.
    pub fn vec_type_param(&self) -> Option<&TypePath> {
        let ty = match &self.0 {
            TypePathInner::Type(ty) => ty,
            _ => return None,
        };

        match ty {
            TypePathType::Vec { of } => Some(of),
            _ => None,
        }
    }
}

/// The path of a Concrete type
#[derive(Clone, Debug)]
pub enum TypePathType {
    /// A user-defined type (non-builtin struct or enum)
    Path {
        /// Type path
        path: syn::Path,
        /// Generic type parameters
        params: Vec<TypePath>,
    },
    /// A variable sized sequences of elements of some type. See [`std::vec::Vec`].
    Vec {
        /// Type of elements in the vector.
        of: Box<TypePath>,
    },
    /// A fixed length array that contains `len` elements of some type.
    Array {
        /// number of elements in the array
        len: usize,
        /// Type path
        of: Box<TypePath>,
    },
    /// A Tuple type
    Tuple {
        /// Types that make up this tuple
        elements: Vec<TypePath>,
    },
    /// Primitive type
    Primitive {
        /// A primitive Rust type.
        def: TypeDefPrimitive,
    },
    /// A compact encoded type
    Compact {
        /// The type that is being compact encoded
        inner: Box<TypePath>,
        /// is this type used as a field of a struct or enum right now?
        is_field: bool,
        /// path to the `Compact` type (usually [`parity_scale_codec::Compact`])
        compact_type_path: syn::Path,
    },
    /// A bit vector
    BitVec {
        /// Order type
        bit_order_type: Box<TypePath>,
        /// Store type
        bit_store_type: Box<TypePath>,
        /// A user defined wrapper type around scale_bits::Bits. Should be generic over the `order` and `store` types.
        decoded_bits_type_path: syn::Path,
    },
}

impl TypePathType {
    /// Constructs a [`TypePathType`] from some context information.
    pub fn from_type_def_path(
        path: &Path<PortableForm>,
        root_mod_ident: Ident,
        params: Vec<TypePath>,
        alloc_crate_path: &AllocCratePath,
    ) -> Self {
        let path_segments = &*path.segments;

        let path: syn::Path = match path_segments {
            [] => panic!("Type has no ident"),
            [ident] => {
                // paths to prelude types
                match ident.as_str() {
                    "Option" => parse_quote!(::core::option::Option),
                    "Result" => parse_quote!(::core::result::Result),
                    "Cow" => parse_quote!(#alloc_crate_path::borrow::Cow),
                    "BTreeMap" => parse_quote!(#alloc_crate_path::collections::BTreeMap),
                    "BTreeSet" => parse_quote!(#alloc_crate_path::collections::BTreeSet),
                    "BinaryHeap" => parse_quote!(#alloc_crate_path::collections::BinaryHeap),
                    "VecDeque" => parse_quote!(#alloc_crate_path::collections::VecDeque),
                    "LinkedList" => parse_quote!(#alloc_crate_path::collections::LinkedList),
                    "Range" => parse_quote!(::core::ops::Range),
                    "RangeInclusive" => parse_quote!(::core::ops::RangeInclusive),
                    "NonZeroI8" => parse_quote!(::core::num::NonZeroI8),
                    "NonZeroU8" => parse_quote!(::core::num::NonZeroU8),
                    "NonZeroI16" => parse_quote!(::core::num::NonZeroI16),
                    "NonZeroU16" => parse_quote!(::core::num::NonZeroU16),
                    "NonZeroI32" => parse_quote!(::core::num::NonZeroI32),
                    "NonZeroU32" => parse_quote!(::core::num::NonZeroU32),
                    "NonZeroI64" => parse_quote!(::core::num::NonZeroI64),
                    "NonZeroU64" => parse_quote!(::core::num::NonZeroU64),
                    "NonZeroI128" => parse_quote!(::core::num::NonZeroI128),
                    "NonZeroU128" => parse_quote!(::core::num::NonZeroU128),
                    "NonZeroIsize" => parse_quote!(::core::num::NonZeroIsize),
                    "NonZeroUsize" => parse_quote!(::core::num::NonZeroUsize),
                    "Duration" => parse_quote!(::core::time::Duration),
                    ident => panic!("Unknown prelude type '{ident}'"),
                }
            }
            _ => {
                // paths to generated types in the root types module
                let mut ty_path = path_segments
                    .iter()
                    .map(|s| syn::PathSegment::from(format_ident!("{}", s)))
                    .collect::<syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]>>();
                ty_path.insert(0, syn::PathSegment::from(root_mod_ident));
                parse_quote!( #ty_path )
            }
        };
        Self::Path { path, params }
    }

    /// Visits a type path, collecting all the generic type parameters from the containing type.
    ///
    /// # Example
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: Vec<Option<T>>, // the parent type param here is `T`
    /// }
    /// ```
    fn parent_type_params(&self, acc: &mut BTreeSet<TypeParameter>) {
        match self {
            TypePathType::Path { params, .. } => {
                for p in params {
                    p.parent_type_params_recurse(acc)
                }
            }
            TypePathType::Vec { of } => of.parent_type_params_recurse(acc),
            TypePathType::Array { of, .. } => of.parent_type_params_recurse(acc),
            TypePathType::Tuple { elements } => {
                for e in elements {
                    e.parent_type_params_recurse(acc)
                }
            }
            TypePathType::Primitive { .. } => (),
            TypePathType::Compact { inner, .. } => inner.parent_type_params_recurse(acc),
            TypePathType::BitVec {
                bit_order_type,
                bit_store_type,
                ..
            } => {
                bit_order_type.parent_type_params_recurse(acc);
                bit_store_type.parent_type_params_recurse(acc);
            }
        }
    }

    /// Returns true, if this is a concrete compact type.
    pub fn is_compact(&self) -> bool {
        matches!(self, TypePathType::Compact { .. })
    }

    /// Returns true, if this is a string type.
    pub fn is_string(&self) -> bool {
        matches!(
            self,
            TypePathType::Primitive {
                def: TypeDefPrimitive::Str
            }
        )
    }

    fn to_syn_type(&self, alloc_crate_path: &AllocCratePath) -> syn::Type {
        match &self {
            TypePathType::Path { path, params } => {
                let path = if params.is_empty() {
                    parse_quote! { #path }
                } else {
                    let params = params.iter().map(|e| e.to_syn_type(alloc_crate_path));
                    parse_quote! { #path< #( #params ),* > }
                };
                syn::Type::Path(path)
            }
            TypePathType::Vec { of } => {
                let of = of.to_syn_type(alloc_crate_path);
                let type_path = parse_quote! { #alloc_crate_path::vec::Vec<#of> };
                syn::Type::Path(type_path)
            }
            TypePathType::Array { len, of } => {
                let of = of.to_syn_type(alloc_crate_path);
                let array = parse_quote! { [#of; #len] };
                syn::Type::Array(array)
            }
            TypePathType::Tuple { elements } => {
                let elements = elements.iter().map(|e| e.to_syn_type(alloc_crate_path));
                let tuple = parse_quote! { (#( # elements, )* ) };
                syn::Type::Tuple(tuple)
            }
            TypePathType::Primitive { def } => syn::Type::Path(match def {
                TypeDefPrimitive::Bool => parse_quote!(::core::primitive::bool),
                TypeDefPrimitive::Char => parse_quote!(::core::primitive::char),
                TypeDefPrimitive::Str => parse_quote!(#alloc_crate_path::string::String),
                TypeDefPrimitive::U8 => parse_quote!(::core::primitive::u8),
                TypeDefPrimitive::U16 => parse_quote!(::core::primitive::u16),
                TypeDefPrimitive::U32 => parse_quote!(::core::primitive::u32),
                TypeDefPrimitive::U64 => parse_quote!(::core::primitive::u64),
                TypeDefPrimitive::U128 => parse_quote!(::core::primitive::u128),
                TypeDefPrimitive::U256 => unimplemented!("not a rust primitive"),
                TypeDefPrimitive::I8 => parse_quote!(::core::primitive::i8),
                TypeDefPrimitive::I16 => parse_quote!(::core::primitive::i16),
                TypeDefPrimitive::I32 => parse_quote!(::core::primitive::i32),
                TypeDefPrimitive::I64 => parse_quote!(::core::primitive::i64),
                TypeDefPrimitive::I128 => parse_quote!(::core::primitive::i128),
                TypeDefPrimitive::I256 => unimplemented!("not a rust primitive"),
            }),
            TypePathType::Compact {
                inner,
                is_field,
                compact_type_path,
            } => {
                let inner = inner.to_syn_type(alloc_crate_path);
                let path = if *is_field {
                    // compact fields can use the inner compact type directly and be annotated with
                    // the `compact` attribute e.g. `#[codec(compact)] my_compact_field: u128`
                    parse_quote! ( #inner )
                } else {
                    parse_quote! ( #compact_type_path<#inner> )
                };
                syn::Type::Path(path)
            }
            TypePathType::BitVec {
                bit_order_type,
                bit_store_type,
                decoded_bits_type_path,
            } => {
                let bit_order_type = bit_order_type.to_syn_type(alloc_crate_path);
                let bit_store_type = bit_store_type.to_syn_type(alloc_crate_path);
                let type_path =
                    parse_quote! { #decoded_bits_type_path<#bit_store_type, #bit_order_type> };
                syn::Type::Path(type_path)
            }
        }
    }
}

/// A generic type parameter
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeParameter {
    pub(super) concrete_type_id: u32,
    pub(super) original_name: String,
    pub(super) name: Ident,
}

impl quote::ToTokens for TypeParameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens)
    }
}
