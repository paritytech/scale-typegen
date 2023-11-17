use anyhow::anyhow;
use proc_macro2::TokenStream;
use scale_info::{form::PortableForm, PortableRegistry, Type};
use std::cell::RefCell;

use crate::transformer::Transformer;



type CodeTransformer<'a> = Transformer<'a, TokenStream, >;

struct CodeTransformerState{
rng: RefCell<rand_chacha::ChaCha8Rng>,
type_path_resolver: TypePathResolver

}

pub fn rust_type_example(id: u32, types: &PortableRegistry, scale) -> anyhow::Result<TokenStream> {
    const MAGIC_SEED: u64 = 42;
    rust_type_example_from_seed(id, types, MAGIC_SEED)


// todo use scale typegen settings here!!!

}

pub fn rust_type_example_from_seed(
    id: u32,
    types: &PortableRegistry,
    seed: u64,
) -> anyhow::Result<TokenStream> {
    fn error_on_recurse(ty: &Type<PortableForm>) -> anyhow::Result<TokenStream> {
        Err(anyhow!(
            "Cannot generate rust type example for recursive type: {ty:?}"
        ))
    }

    let state = RefCell::new(rand_chacha::ChaCha8Rng::seed_from_u64(seed));
    let transformer = CodeTransformer::new(type_example, error_on_recurse, state, types);
    transformer.resolve(id)
}

fn type_example(
    ty: &Type<PortableForm>,
    transformer: &CodeTransformer,
) -> anyhow::Result<TokenStream> {



}