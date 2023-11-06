```rs
pub struct Settings {
    /// The name of the module which will contain the generated types.
    mod_ident: Ident,
    // /// User defined overrides for generated types.
    // type_substitutes: TypeSubstitutes,
    should_gen_docs: bool,
    derives: DerivesRegistry,
    /// e.g. `subxt::utils::bits::DecodedBits`,
    decoded_bits_type_path: syn::Path,
    /// e.g. `subxt::ext::codec::Compact`,
    compact_type_path: syn::Path,
}


pub struct Generator<'a> {
    registry: &'a PortableRegistry,
    settings: Settings,
}


pub struct TypeGenerator<'a> {
    generator: &'a Generator<'a>,
    ty_id: u32,
    ty: &'a Type<PortableForm>,
}

```
