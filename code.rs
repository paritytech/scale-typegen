pub mod types {
    use super::types;
    pub mod scale_typegen {
        use super::types;
        pub mod typegen {
            use super::types;
            pub mod tests {
                use super::types;
                pub mod nested {
                    use super::types;
                    #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
                    pub enum Animal {
                        #[codec(index = 0)]
                        Cat,
                        #[codec(index = 1)]
                        Ant(::core::primitive::bool),
                        #[codec(index = 2)]
                        Monkey {
                            favorite_food: types::scale_typegen::typegen::tests::nested::Food,
                            length: types::scale_typegen::typegen::tests::nested::Centimeter,
                        },
                        #[codec(index = 3)]
                        Chimera {
                            base: types::scale_typegen::typegen::tests::nested::Animal,
                            mutations: types::scale_typegen::typegen::tests::nested::Animal,
                        },
                    }
                    #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
                    pub struct Centimeter(#[codec(compact)] pub ::core::primitive::u16);
                    #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
                    pub enum Food {
                        #[codec(index = 0)]
                        Banana,
                        #[codec(index = 1)]
                        Orange,
                        #[codec(index = 2)]
                        Apple,
                    }
                }
                #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
                pub struct Person {
                    pub name: ::std::string::String,
                    pub age: ::core::primitive::u8,
                }
            }
        }
    }
}
