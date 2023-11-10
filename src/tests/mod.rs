use parity_scale_codec::Compact;

use quote::{quote};
use scale_info::{TypeInfo};
use syn::parse_quote;

use crate::{
    tests::utils::{subxt_settings, Codegen},
    typegen::{settings::TypeGeneratorSettings},
};

mod utils;

#[derive(TypeInfo)]
pub struct Person {
    name: Box<String>,
    age: u8,
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
    let code = Codegen::new().with::<A>().gen(settings).unwrap();
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
    let code = Codegen::new().with::<Animal>().gen(settings).unwrap();

    let expected_code = quote! {
        pub mod types {
            use super::types;
            pub mod scale_typegen {
                use super::types;
                pub mod tests {
                    use super::types;
                    pub enum Animal {
                        Cat,
                        Ant(::core::primitive::bool,),
                        Monkey {
                            favorite_food: types::scale_typegen::tests::Food,
                            length: types::scale_typegen::tests::Centimeter,
                        },
                        Chimera {
                            base: ::std::boxed::Box<types::scale_typegen::tests::Animal>,
                            mutations: ::std::boxed::Box<types::scale_typegen::tests::Animal>,
                        },
                    }
                    pub struct Centimeter(pub ::core::primitive::u16,);
                    pub enum Food {
                        Banana,
                        Orange,
                        Apple,
                    }
                }
            }
        }
    };
    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generate_struct_with_primitives() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: bool,
        b: u32,
        c: char,
    }

    let code = Codegen::new()
        .with::<S>()
        .gen_tests_mod(subxt_settings())
        .unwrap();

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: ::core::primitive::bool,
                pub b: ::core::primitive::u32,
                pub c: ::core::primitive::char,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string())
}

#[test]
fn generate_struct_with_a_struct_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent {
        a: bool,
        b: Child,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child {
        a: i32,
    }

    let code = Codegen::new()
        .with::<Parent>()
        .gen_tests_mod(subxt_settings())
        .unwrap();

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Child {
                pub a: ::core::primitive::i32,
            }

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Parent {
                pub a: ::core::primitive::bool,
                pub b: root::scale_typegen::tests::Child,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string())
}

#[test]
fn generate_tuple_struct() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent(bool, Child);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child(i32);

    let code = Codegen::new()
        .with::<Parent>()
        .gen_tests_mod(subxt_settings())
        .unwrap();

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Child(pub ::core::primitive::i32,);

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Parent(pub ::core::primitive::bool, pub root::scale_typegen::tests::Child,);
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string())
}

#[test]
fn derive_compact_as_for_uint_wrapper_structs() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su8 {
        a: u8,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu8(u8);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su16 {
        a: u16,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu16(u16);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su32 {
        a: u32,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu32(u32);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su64 {
        a: u64,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu64(u64);
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Su128 {
        a: u128,
    }
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TSu128(u128);

    let code = Codegen::new()
        .with::<Su8>()
        .with::<TSu8>()
        .with::<Su16>()
        .with::<TSu16>()
        .with::<Su32>()
        .with::<TSu32>()
        .with::<Su64>()
        .with::<TSu64>()
        .with::<Su128>()
        .with::<TSu128>()
        .gen_tests_mod(subxt_settings())
        .unwrap();

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Su128 { pub a: ::core::primitive::u128, }

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Su16 { pub a: ::core::primitive::u16, }

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Su32 { pub a: ::core::primitive::u32, }

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Su64 { pub a: ::core::primitive::u64, }

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Su8 { pub a: ::core::primitive::u8, }

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TSu128(pub ::core::primitive::u128,);

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TSu16(pub ::core::primitive::u16,);

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TSu32(pub ::core::primitive::u32,);

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TSu64(pub ::core::primitive::u64,);

            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TSu8(pub ::core::primitive::u8,);
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string())
}
