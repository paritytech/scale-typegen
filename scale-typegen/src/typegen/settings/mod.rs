use derives::DerivesRegistry;

use proc_macro2::Ident;
use substitutes::TypeSubstitutes;
use syn::parse_quote;

use self::substitutes::absolute_path;

pub mod derives;
pub mod substitutes;

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
        }
    }
}

impl TypeGeneratorSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn type_mod_name(mut self, type_mod_name: &str) -> Self {
        self.types_mod_ident =
            syn::parse_str(type_mod_name).expect("The provided type_mod_name is not a valid ident");
        self
    }

    pub fn substitute(mut self, from: syn::Path, to: syn::Path) -> Self {
        self.substitutes
            .insert(from, absolute_path(to).unwrap())
            .unwrap();
        self
    }

    pub fn decoded_bits_type_path(mut self, path: syn::Path) -> Self {
        self.decoded_bits_type_path = Some(path);
        self
    }

    pub fn should_gen_docs(mut self, should_gen_docs: bool) -> Self {
        self.should_gen_docs = should_gen_docs;
        self
    }

    pub fn insert_codec_attributes(mut self) -> Self {
        self.insert_codec_attributes = true;
        self
    }

    pub fn derive_on_all(mut self, derive_paths: impl IntoIterator<Item = syn::Path>) -> Self {
        self.derives.extend_for_all(derive_paths, []);
        self
    }
}
