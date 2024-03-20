use std::marker::PhantomData;

use parity_scale_codec::Decode;
use proc_macro2::TokenStream;
use scale_info::PortableRegistry;
use scale_typegen::{typegen::ir::ToTokensWithSettingsT, TypeGenerator, TypeGeneratorSettings};
use syn::parse_quote;

#[allow(unused)]
pub struct DecodedBits<Store, Order> {
    bits: scale_bits::Bits,
    _marker: PhantomData<(Store, Order)>,
}

/// This example shows how to use metadata from a polkadot node to generate rust types.
pub fn main() {
    let type_registry = read_registry_from_scale_metadata();
    let settings = TypeGeneratorSettings::default()
        .type_mod_name("my_types")
        .decoded_bits_type_path(parse_quote!(DecodedBits))
        .compact_as_type_path(parse_quote!(parity_scale_codec::CompactAs))
        .compact_type_path(parse_quote!(parity_scale_codec::Compact))
        .add_derives_for_all([parse_quote!(Debug), parse_quote!(Clone)]);

    let type_generator = TypeGenerator::new(&type_registry, &settings);
    let code = type_generator
        .generate_types_mod()
        .unwrap()
        .to_token_stream(&settings);

    write_pretty_tokens(code, "./artifacts/generated_polkadot.rs");
}

fn read_registry_from_scale_metadata() -> PortableRegistry {
    let bytes = std::fs::read("./artifacts/polkadot_metadata.scale")
        .expect("File polkadot_metadata not found!");
    let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(&mut &bytes[..])
        .expect("Metadata decoding failed");
    match metadata.1 {
        frame_metadata::RuntimeMetadata::V14(m) => m.types,
        frame_metadata::RuntimeMetadata::V15(m) => m.types,
        _ => panic!("Metadata too old, needs to be V14 or V15"),
    }
}

fn write_pretty_tokens(tokens: TokenStream, path: &str) {
    let syn_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    std::fs::write(path, pretty).unwrap();
}
