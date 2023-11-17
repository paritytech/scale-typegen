use proc_macro2::TokenStream;
use scale_info::PortableRegistry;

pub fn rust_type_example(id: u32, types: &PortableRegistry) -> anyhow::Result<TokenStream> {
    const MAGIC_SEED: u64 = 42;
    rust_type_example_from_seed(id, types, MAGIC_SEED)
}

pub fn rust_type_example_from_seed(
    id: u32,
    types: &PortableRegistry,
    seed: u64,
) -> anyhow::Result<TokenStream> {
    todo!()
}
