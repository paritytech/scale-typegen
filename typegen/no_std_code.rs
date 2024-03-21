pub mod tests {
    use super::types;
    pub struct Foo {
        pub a: ::subxt_core::alloc::vec::Vec<types::scale_typegen::tests::Foo>,
        pub b: ::subxt_core::alloc::boxed::Box<::core::primitive::u8>,
        pub c: ::subxt_core::alloc::string::String,
        pub e: ::subxt_core::alloc::boxed::Box<
            ::subxt_core::alloc::collections::BTreeMap<
                ::subxt_core::alloc::string::String,
                ::core::primitive::u8,
            >,
        >,
    }
}
