use derives::DerivesRegistry;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use substitutes::TypeSubstitutes;
use syn::parse_quote;

use self::substitutes::absolute_path;

/// Settings for which derives should be applied on types
pub mod derives;
/// Settings for which types should be substituted by other types.
pub mod substitutes;

/// A struct containing all the settings for generating rust types from a type registry.
#[derive(Debug, Clone)]
pub struct TypeGeneratorSettings {
    /// The name of the module which will contain the generated types.
    pub types_mod_ident: Ident,
    /// If false, no docs are generated for the types.
    pub should_gen_docs: bool,
    /// Derive traits on generated types.
    pub derives: DerivesRegistry,
    /// User defined overrides for generated types.
    pub substitutes: TypeSubstitutes,
    /// Two generic parameters are expected on this type:
    /// - Store (e.g. `u8`/`u16`/`u32`/`u64`)
    /// - Order (e.g. LSB, MSB)
    ///
    /// subxt provides a `subxt::utils::DecodedBits` that can be used here.
    pub decoded_bits_type_path: Option<syn::Path>,
    /// TypePath to the CompactAs trait/derive macro.
    /// E.g. `subxt::ext::codec::CompactAs`
    pub compact_as_type_path: Option<syn::Path>,
    /// TypePath to the Compact<T> struct.
    /// E.g. `subxt::ext::codec::Compact`
    pub compact_type_path: Option<syn::Path>,
    /// If false, no codec attributes like `codec(index=0)` and `codec(compact)` are inserted.
    /// This is a useful option if we do not want to derive Decode and Encode on our types.
    pub insert_codec_attributes: bool,
    /// Configure a custom type path for the `alloc` crate, which is the base for generating type paths like
    /// `alloc::string::String`, `alloc::vec::Vec` and `alloc::boxed::Box`. The default is `AllocCratePath::Std` which
    /// uses the types from the `std` library instead.
    pub alloc_crate_path: AllocCratePath,
}

/// Information about how to construct the type paths for types that need allocation, e.g.
#[derive(Debug, Clone, Default)]
pub enum AllocCratePath {
    /// Equivalent to `AllocCratePath::Custom(quote!(::std))`. This is the default.
    #[default]
    Std,
    /// Custom path to the alloc crate, e.g. `::alloc` if `extern crate alloc;` is used
    /// or e.g. `::subxt_core::alloc` if the alloc crate is exported from another crate.
    ///
    /// We'd expect the Custom path to export `vec::Vec`, `string::String`, `boxed::Box`
    /// and `collections::BTreeMap`.
    Custom(syn::Path),
}

impl ToTokens for AllocCratePath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            AllocCratePath::Std => quote!(::std).to_tokens(tokens),
            AllocCratePath::Custom(alloc_path) => alloc_path.to_tokens(tokens),
        }
    }
}

impl Default for TypeGeneratorSettings {
    fn default() -> Self {
        Self {
            types_mod_ident: parse_quote!(types),
            should_gen_docs: true,
            substitutes: TypeSubstitutes::new(),
            derives: DerivesRegistry::new(),
            decoded_bits_type_path: None,
            compact_as_type_path: None,
            compact_type_path: None,
            insert_codec_attributes: false,
            alloc_crate_path: Default::default(),
        }
    }
}

impl TypeGeneratorSettings {
    /// Creates a new `TypeGeneratorSettings`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `type_mod_name` field.
    pub fn type_mod_name(mut self, type_mod_name: &str) -> Self {
        self.types_mod_ident =
            syn::parse_str(type_mod_name).expect("The provided type_mod_name is not a valid ident");
        self
    }

    /// Adds a rule, that a type with path `from` should be replaced with the path `to`.
    pub fn substitute(mut self, from: syn::Path, to: syn::Path) -> Self {
        self.substitutes
            .insert(from, absolute_path(to).unwrap())
            .unwrap();
        self
    }

    /// Sets the `compact_as_type_path` field.
    pub fn compact_as_type_path(mut self, path: syn::Path) -> Self {
        self.compact_as_type_path = Some(path);
        self
    }

    /// Sets the `compact_type_path` field.
    pub fn compact_type_path(mut self, path: syn::Path) -> Self {
        self.compact_type_path = Some(path);
        self
    }

    /// Sets the `decoded_bits_type_path` field.
    pub fn decoded_bits_type_path(mut self, path: syn::Path) -> Self {
        self.decoded_bits_type_path = Some(path);
        self
    }

    /// Sets the `should_gen_docs` field.
    pub fn should_gen_docs(mut self, should_gen_docs: bool) -> Self {
        self.should_gen_docs = should_gen_docs;
        self
    }

    /// Sets the `insert_codec_attributes` field.
    pub fn insert_codec_attributes(mut self) -> Self {
        self.insert_codec_attributes = true;
        self
    }

    /// Adds some derives for all types.
    pub fn add_derives_for_all(
        mut self,
        derive_paths: impl IntoIterator<Item = syn::Path>,
    ) -> Self {
        self.derives.add_derives_for_all(derive_paths);
        self
    }
}
