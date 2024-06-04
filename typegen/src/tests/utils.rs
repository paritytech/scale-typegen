use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use scale_info::{PortableRegistry, TypeInfo};
use syn::parse_quote;

use crate::typegen::ir::module_ir::ModuleIR;
use crate::typegen::ir::ToTokensWithSettings;
use crate::utils::ensure_unique_type_paths;
use crate::TypegenError;
use crate::{
    typegen::settings::substitutes::absolute_path, DerivesRegistry, TypeGenerator,
    TypeGeneratorSettings, TypeSubstitutes,
};

pub(super) struct Testgen {
    registry: scale_info::Registry,
}

impl Testgen {
    pub fn new() -> Self {
        Testgen {
            registry: scale_info::Registry::new(),
        }
    }

    pub fn with<T: TypeInfo + 'static>(mut self) -> Self {
        let m = scale_info::MetaType::new::<T>();
        self.registry.register_type(&m);
        self
    }

    pub fn into_portable_registry(self) -> PortableRegistry {
        self.registry.into()
    }

    pub fn gen(self, settings: TypeGeneratorSettings) -> TokenStream {
        let mut registry: PortableRegistry = self.registry.into();
        ensure_unique_type_paths(&mut registry);
        let type_gen = TypeGenerator::new(&registry, &settings);
        let module = type_gen.generate_types_mod().unwrap();
        module.to_token_stream(&settings)
    }

    pub fn try_gen_tests_mod(
        self,
        settings: TypeGeneratorSettings,
        deduplicate: bool,
    ) -> Result<TokenStream, TypegenError> {
        let mut registry: PortableRegistry = self.registry.into();
        if deduplicate {
            ensure_unique_type_paths(&mut registry)
        }
        let type_gen = TypeGenerator::new(&registry, &settings);
        type_gen.generate_types_mod().map(|module| {
            get_mod(&module, TESTS_MOD_PATH)
                .unwrap()
                .to_token_stream(&settings)
        })
    }

    pub fn gen_tests_mod(self, settings: TypeGeneratorSettings) -> TokenStream {
        self.try_gen_tests_mod(settings, false)
            .unwrap()
            .to_token_stream()
    }
}

pub(super) fn subxt_settings() -> TypeGeneratorSettings {
    TypeGeneratorSettings {
        types_mod_ident: parse_quote!(root),
        should_gen_docs: true,
        derives: subxt_default_derives(),
        substitutes: subxt_default_substitutes(),
        decoded_bits_type_path: Some(parse_quote!(::subxt_path::utils::bits::DecodedBits)),
        insert_codec_attributes: true,
        compact_as_type_path: Some(parse_quote!(::subxt_path::ext::codec::CompactAs)),
        compact_type_path: Some(parse_quote!(::subxt_path::ext::codec::Compact)),
        alloc_crate_path: Default::default(),
        parent_path: std::cell::RefCell::default(),
    }
}
/// Derives mirroring the subxt default derives
pub(super) fn subxt_default_derives() -> DerivesRegistry {
    let crate_path: syn::Path = parse_quote!(::subxt_path);

    let encode_crate_path = quote! { #crate_path::ext::scale_encode }.to_string();
    let decode_crate_path = quote! { #crate_path::ext::scale_decode }.to_string();

    let derives: [syn::Path; 5] = [
        parse_quote!(#crate_path::ext::scale_encode::EncodeAsType),
        parse_quote!(#crate_path::ext::scale_decode::DecodeAsType),
        parse_quote!(#crate_path::ext::codec::Encode),
        parse_quote!(#crate_path::ext::codec::Decode),
        parse_quote!(Debug),
    ];

    let attributes: [syn::Attribute; 3] = [
        parse_quote!(#[encode_as_type(crate_path = #encode_crate_path)]),
        parse_quote!(#[decode_as_type(crate_path = #decode_crate_path)]),
        parse_quote!(#[codec(crate = #crate_path::ext::codec)]),
    ];

    let mut derives_registry = DerivesRegistry::new();

    derives_registry.add_derives_for_all(derives);
    derives_registry.add_attributes_for_all(attributes);
    derives_registry
}

pub(super) fn subxt_default_substitutes() -> TypeSubstitutes {
    let crate_path: syn::Path = parse_quote!(::subxt_path);
    let mut type_substitutes = TypeSubstitutes::new();

    let defaults: [(syn::Path, syn::Path); 11] = [
        (
            parse_quote!(bitvec::order::Lsb0),
            parse_quote!(#crate_path::utils::bits::Lsb0),
        ),
        (
            parse_quote!(bitvec::order::Msb0),
            parse_quote!(#crate_path::utils::bits::Msb0),
        ),
        (
            parse_quote!(sp_core::crypto::AccountId32),
            parse_quote!(#crate_path::utils::AccountId32),
        ),
        (
            parse_quote!(sp_runtime::multiaddress::MultiAddress),
            parse_quote!(#crate_path::utils::MultiAddress),
        ),
        (
            parse_quote!(primitive_types::H160),
            parse_quote!(#crate_path::utils::H160),
        ),
        (
            parse_quote!(primitive_types::H256),
            parse_quote!(#crate_path::utils::H256),
        ),
        (
            parse_quote!(primitive_types::H512),
            parse_quote!(#crate_path::utils::H512),
        ),
        (
            parse_quote!(frame_support::traits::misc::WrapperKeepOpaque),
            parse_quote!(#crate_path::utils::WrapperKeepOpaque),
        ),
        // BTreeMap and BTreeSet impose an `Ord` constraint on their key types. This
        // can cause an issue with generated code that doesn't impl `Ord` by default.
        // Decoding them to Vec by default (KeyedVec is just an alias for Vec with
        // suitable type params) avoids these issues.
        (
            parse_quote!(BTreeMap),
            parse_quote!(#crate_path::utils::KeyedVec),
        ),
        (parse_quote!(BTreeSet), parse_quote!(::std::vec::Vec)),
        // The `UncheckedExtrinsic(pub Vec<u8>)` is part of the runtime API calls.
        // The inner bytes represent the encoded extrinsic, however when deriving the
        // `EncodeAsType` the bytes would be re-encoded. This leads to the bytes
        // being altered by adding the length prefix in front of them.
        (
            parse_quote!(sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic),
            parse_quote!(#crate_path::utils::UncheckedExtrinsic),
        ),
    ];

    let defaults = defaults.into_iter().map(|(from, to)| {
        (
            from,
            absolute_path(to).expect("default substitutes should be all absolute paths"),
        )
    });
    type_substitutes
        .extend(defaults)
        .expect("default substitutes should never error");
    type_substitutes
}

const TESTS_MOD_PATH: &[&str] = &["scale_typegen", "tests"];

fn get_mod<'a>(module: &'a ModuleIR, path_segs: &[&'static str]) -> Option<&'a ModuleIR> {
    let (mod_name, rest) = path_segs.split_first()?;
    let mod_ident: Ident = syn::parse_str(mod_name).unwrap();
    let module = module.children.get(&mod_ident)?;
    if rest.is_empty() {
        Some(module)
    } else {
        get_mod(module, rest)
    }
}
