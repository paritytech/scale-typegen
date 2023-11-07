use quote::ToTokens;
use scale_info::{form::MetaForm, PortableRegistry, Registry, TypeInfo};

use self::nested::Animal;

use super::{
    derives::{Derives, DerivesRegistry},
    TypeGenerator, TypeGeneratorSettings,
};

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

struct RegistryBuilder {
    reg: scale_info::Registry,
}

impl RegistryBuilder {
    fn new() -> Self {
        RegistryBuilder {
            reg: scale_info::Registry::new(),
        }
    }

    fn with<T: TypeInfo + 'static>(mut self) -> Self {
        let m = scale_info::MetaType::new::<T>();
        self.reg.register_type(&m);
        self
    }
    fn build(self) -> PortableRegistry {
        self.reg.into()
    }
}

#[test]
fn test_code_gen() {
    let mut types = RegistryBuilder::new()
        .with::<Person>()
        // .with::<Animal>()
        .build();
    let mut settings = TypeGeneratorSettings::default();
    settings.derives.extend_for_all(
        [
            syn::parse_str("::parity_scale_codec::Decode").unwrap(),
            syn::parse_str("::parity_scale_codec::Encode").unwrap(),
        ],
        [],
    );
    let type_gen = TypeGenerator::new(&types, settings).unwrap();

    let m = type_gen.generate_types_mod().unwrap();
    let code = m.into_token_stream();
    std::fs::write("code.rs", code.to_string());
}
