use proc_macro2::TokenStream;
use scale_info::PortableRegistry;

pub fn example_rust_type(id: u32, types: &PortableRegistry) -> anyhow::Result<TokenStream> {
    const MAGIC_SEED: u64 = 42;
    example_rust_type_from_seed(id, types, MAGIC_SEED)
}

pub fn example_rust_type_from_seed(
    id: u32,
    types: &PortableRegistry,
    seed: u64,
) -> anyhow::Result<TokenStream> {
    todo!()
}
