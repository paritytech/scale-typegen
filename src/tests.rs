use std::collections::{BTreeMap, HashMap};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use scale_info::{form::MetaForm, PortableRegistry, Registry, TypeInfo};
use syn::parse_quote;

use crate::{
    path_segments,
    typegen::{
        settings::{
            substitutes::{absolute_path, AbsolutePath, PathSegments},
            TypeGeneratorSettings,
        },
        TypeGenerator,
    },
};

use self::nested::Animal;

mod nested {
    use parity_scale_codec::Compact;
    use scale_info::TypeInfo;

    #[derive(TypeInfo)]
    pub enum Animal {
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
}

#[derive(TypeInfo)]
pub struct Person {
    name: Box<String>,
    age: u8,
}

struct CodeGenBuilder {
    registry: scale_info::Registry,
}

impl CodeGenBuilder {
    fn new() -> Self {
        CodeGenBuilder {
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
    let code = CodeGenBuilder::new().with::<A>().gen(settings).unwrap();
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

// #[test]
// fn test_code_gen() {
//     let mut types = RegistryBuilder::new()
//         .with::<Person>()
//         // .with::<Animal>()
//         .build();
//     let mut settings = TypeGeneratorSettings::default();
//     settings.derives.extend_for_all(
//         [
//             syn::parse_str("::parity_scale_codec::Decode").unwrap(),
//             syn::parse_str("::parity_scale_codec::Encode").unwrap(),
//         ],
//         [],
//     );
//     let type_gen = TypeGenerator::new(&types, settings).unwrap();

//     let m = type_gen.generate_types_mod().unwrap();
//     let code = m.into_token_stream();
//     // std::fs::write("code.rs", code.to_string());
// }
