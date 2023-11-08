pub mod types {
    use super::types;
    pub mod scale_typegen {
        use super::types;
        pub mod tests {
            use super::types;
            pub enum Animal {
                #[codec(index = 0)]
                Cat,
                #[codec(index = 1)]
                Ant(::core::primitive::bool),
                #[codec(index = 2)]
                Monkey {
                    favorite_food: types::scale_typegen::tests::Food,
                    length: types::scale_typegen::tests::Centimeter,
                },
                #[codec(index = 3)]
                Chimera {
                    base: ::std::boxed::Box<types::scale_typegen::tests::Animal>,
                    mutations: ::std::boxed::Box<types::scale_typegen::tests::Animal>,
                },
            }
            pub struct Centimeter(#[codec(compact)] pub ::core::primitive::u16);
            pub enum Food {
                #[codec(index = 0)]
                Banana,
                #[codec(index = 1)]
                Orange,
                #[codec(index = 2)]
                Apple,
            }
        }
    }
}
