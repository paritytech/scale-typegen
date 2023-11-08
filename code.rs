pub mod my_types {
    use super::my_types;
    pub mod scale_typegen {
        use super::my_types;
        pub mod tests {
            use super::my_types;
            #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Emcode)]
            pub struct A {
                pub children: ::std::collections::HashMap<
                    ::core::primitive::u32,
                    my_types::scale_typegen::tests::A,
                >,
            }
        }
    }
}
