use parity_scale_codec::Compact;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use scale_info::{PortableRegistry, TypeInfo};
use syn::parse_quote;

use crate::typegen::{settings::TypeGeneratorSettings, TypeGenerator};

#[derive(TypeInfo)]
pub struct Person {
    name: Box<String>,
    age: u8,
}

struct CodegenBuilder {
    registry: scale_info::Registry,
}

impl CodegenBuilder {
    fn new() -> Self {
        CodegenBuilder {
            registry: scale_info::Registry::new(),
        }
    }

    fn with<T: TypeInfo + 'static>(mut self) -> Self {
        let m = scale_info::MetaType::new::<T>();
        self.registry.register_type(&m);
        self
    }
    fn gen(self, settings: TypeGeneratorSettings) -> anyhow::Result<TokenStream> {
        let registry: PortableRegistry = self.registry.into();
        let type_gen = TypeGenerator::new(&registry, settings)?;
        let m = type_gen.generate_types_mod()?;
        Ok(m.to_token_stream())
    }
}

#[test]
fn substitutes_and_derives_work() {
    // set up settings
    let settings = TypeGeneratorSettings::default()
        .type_mod_name("my_types")
        .derive_on_all([
            parse_quote!(::parity_scale_codec::Decode),
            parse_quote!(::parity_scale_codec::Emcode),
        ])
        .substitute(
            parse_quote!(BTreeMap),
            parse_quote!(::std::collections::HashMap),
        );

    // set up types
    #[derive(TypeInfo)]
    struct A {
        children: std::collections::BTreeMap<u32, A>,
    }

    // check the generated code
    let code = CodegenBuilder::new().with::<A>().gen(settings).unwrap();
    let expected_code = quote! {
        pub mod my_types {
            use super::my_types;
            pub mod scale_typegen {
                use super::my_types;
                pub mod tests {
                    use super::my_types;
                    #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Emcode)]
                    pub struct A {
                        pub children: ::std::collections::HashMap< ::core::primitive::u32, my_types::scale_typegen::tests::A>,
                    }
                }
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn different_structures_work() {
    #[derive(TypeInfo)]
    pub enum Animal {
        #[codec(index = 99)]
        Cat,
        Ant(bool),
        Monkey {
            favorite_food: Food,
            length: Centimeter,
        },
        Chimera {
            base: Box<Animal>,
            mutations: Box<Animal>,
        },
    }

    #[derive(TypeInfo)]
    pub struct Centimeter(Compact<u16>);

    #[derive(TypeInfo)]
    pub enum Food {
        Banana,
        Orange,
        Apple,
    }
    let settings = TypeGeneratorSettings::default();
    let code = CodegenBuilder::new()
        .with::<Animal>()
        .gen(settings)
        .unwrap();

    let expected_code = quote! {
        pub mod types {
            use super::types;
            pub mod scale_typegen {
                use super::types;
                pub mod tests {
                    use super::types;
                    pub enum Animal {
                        #[codec(index = 99)]
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
    };
    assert_eq!(code.to_string(), expected_code.to_string());
}
