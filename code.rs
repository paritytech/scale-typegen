pub mod types {
    use super::types;
    pub mod scale_typegen {
        use super::types;
        pub mod typegen {
            use super::types;
            pub mod tests {
                use super::types;
                #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
                pub struct A {
                    pub a: ::std::collections::BTreeMap<
                        ::core::primitive::u32,
                        types::scale_typegen::typegen::tests::A,
                    >,
                }
            }
        }
    }
}
