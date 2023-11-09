use std::marker::PhantomData;

use proc_macro2::TokenStream;
use quote::ToTokens;
use scale_info::PortableRegistry;
use scale_typegen::typegen::{settings::TypeGeneratorSettings, TypeGenerator};
use serde_json::Value;
use syn::parse_quote;

pub struct DecodedBits<Store, Order> {
    bits: scale_bits::Bits,
    _marker: PhantomData<(Store, Order)>,
}

/// This example shows how to use metadata from a polkadot node to generate rust types.
///
/// Some types might need
pub fn main() {
    let type_registry = read_registry_from_json();
    let settings = TypeGeneratorSettings::default()
        .type_mod_name("my_types")
        .decoded_bits_type_path(parse_quote!(DecodedBits));

    let type_generator = TypeGenerator::new(&type_registry, settings).unwrap();
    let code = type_generator
        .generate_types_mod()
        .unwrap()
        .to_token_stream();

    write_pretty_tokens(code, "./src/polkadot.rs");
}

fn read_registry_from_json() -> PortableRegistry {
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string("./artifacts/polkadot.json").unwrap())
            .unwrap();
    let types = json.as_array().unwrap()[1]
        .as_object()
        .unwrap()
        .get("V15")
        .unwrap()
        .get("types")
        .unwrap();
    let registry: PortableRegistry = serde_json::from_value(types.clone()).unwrap();
    registry
}

fn write_pretty_tokens(tokens: TokenStream, path: &str) {
    let syn_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    std::fs::write(path, pretty).unwrap();
}
