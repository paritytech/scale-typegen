use parity_scale_codec::Compact;

use pretty_assertions::assert_eq;
use quote::quote;
use scale_info::TypeInfo;
use syn::parse_quote;

use crate::{
    tests::utils::{subxt_settings, Testgen},
    typegen::{
        error::SettingsValidationError,
        settings::{AllocCratePath, TypeGeneratorSettings},
        validation::{
            similar_type_paths_in_registry, validate_substitutes_and_derives_against_registry,
        },
    },
    utils::ensure_unique_type_paths,
    DerivesRegistry, TypeSubstitutes, TypegenError,
};

mod utils;

// // Dev note: It would be great to get this test passing, but the current type equality check
// // picks exactly one matching generic to line up to a type, and not a set of them. The problem with
// // a set would be that you can have eg three types like this in the registry:
// //
// // ```txt
// // 1: struct Foo<T = u8, U = u8, V = u8>(u8, u8, u8);
// // 2: struct Foo<T = u8, U = u16, V = u32>(u8, u16, u32);
// // 3: struct Foo<T = u8, U = u16, V = u32>(u32, u16, u8);
// // ```
// //
// // 1 == 2 and 1 == 3 (both could be represented by `Foo<T,U,V>(T,U,V)`) but 2 != 3.
// // needs more thought.
// //

#[test]
fn more_than_1_generic_parameters() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T, U, V, W> {
        a: T,
        b: U,
        c: V,
        d: W,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar {
        p: Foo<u32, u32, u64, u128>,
        q: Foo<u8, u8, u8, u8>,
    }

    let generated = Testgen::new()
        .with::<Bar>()
        .gen_tests_mod(Default::default());

    #[rustfmt::skip]
    let expected = quote!(
        pub mod tests {
            use super::types;
            pub struct Bar {
                pub p: types::scale_typegen::tests::Foo<
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                    ::core::primitive::u64,
                    ::core::primitive::u128
                >,
                pub q: types::scale_typegen::tests::Foo<
                    ::core::primitive::u8,
                    ::core::primitive::u8,
                    ::core::primitive::u8,
                    ::core::primitive::u8
                >,
            }
            pub struct Foo<_0, _1, _2, _3> {
                pub a: _0,
                pub b: _1,
                pub c: _2,
                pub d: _3,
            }
        }
    );

    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn more_than_1_generic_parameters_tuple() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T, U, V, W>(T, U, V, W);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar {
        p: Foo<u32, u32, u64, u128>,
        q: Foo<u8, u8, u8, u8>,
    }

    let generated = Testgen::new()
        .with::<Bar>()
        .gen_tests_mod(Default::default());

    #[rustfmt::skip]
    let expected = quote!(
        pub mod tests {
            use super::types;
            pub struct Bar {
                pub p: types::scale_typegen::tests::Foo<
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                    ::core::primitive::u64,
                    ::core::primitive::u128
                >,
                pub q: types::scale_typegen::tests::Foo<
                    ::core::primitive::u8,
                    ::core::primitive::u8,
                    ::core::primitive::u8,
                    ::core::primitive::u8
                >,
            }
            pub struct Foo<_0, _1, _2, _3>(pub _0, pub _1, pub _2, pub _3 ,);
        }
    );

    assert_eq!(generated.to_string(), expected.to_string());
}
#[test]
fn dupe_types_do_not_overwrite_each_other() {
    enum Foo {}
    impl TypeInfo for Foo {
        type Identity = Self;
        fn type_info() -> scale_info::Type {
            scale_info::Type::builder()
                .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                .variant(
                    scale_info::build::Variants::new()
                        .variant("FirstDupeTypeVariant", |builder| builder.index(0)),
                )
        }
    }
    enum Bar {}
    impl TypeInfo for Bar {
        type Identity = Self;
        fn type_info() -> scale_info::Type {
            scale_info::Type::builder()
                .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                .variant(
                    scale_info::build::Variants::new()
                        .variant("SecondDupeTypeVariant", |builder| builder.index(0)),
                )
        }
    }

    let generated = Testgen::new()
        .with::<Foo>()
        .with::<Bar>()
        .gen(Default::default())
        .to_string();

    let expected = quote!(
        pub mod types {
            use super::types;
            pub mod dupe_mod {
                use super::types;
                pub enum DuplicateType1 {
                    FirstDupeTypeVariant,
                }
                pub enum DuplicateType2 {
                    SecondDupeTypeVariant,
                }
            }
        }
    );

    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn generic_types_overwrite_each_other() {
    use scale_info::meta_type;

    // If we have two types mentioned in the registry that have generic params,
    // only one type will be output (the codegen assumes that the generic param will disambiguate)
    enum Foo {}
    impl TypeInfo for Foo {
        type Identity = Self;
        fn type_info() -> scale_info::Type {
            scale_info::Type::builder()
                .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                .type_params([scale_info::TypeParameter::new("T", Some(meta_type::<u8>()))])
                .variant(scale_info::build::Variants::new())
        }
    }
    enum Bar {}
    impl TypeInfo for Bar {
        type Identity = Self;
        fn type_info() -> scale_info::Type {
            scale_info::Type::builder()
                .path(scale_info::Path::new("DuplicateType", "dupe_mod"))
                .type_params([scale_info::TypeParameter::new("T", Some(meta_type::<u8>()))])
                .variant(scale_info::build::Variants::new())
        }
    }

    let generated = Testgen::new()
        .with::<Foo>()
        .with::<Bar>()
        .gen(Default::default())
        .to_string();

    // Since a generic is present on the type, we do _not_ expect two types to be generated:
    let expected = quote!(
        pub mod types {
            use super::types;
            pub mod dupe_mod {
                use super::types;
                pub enum DuplicateType<_0> {
                    __Ignore(::core::marker::PhantomData<_0>),
                }
            }
        }
    );

    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn substitutes_with_generics() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A {
        gen: Pair<u32, u64>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Pair<T, R> {
        t: T,
        r: R,
    }

    // replace with other generic type
    {
        let settings = TypeGeneratorSettings::default()
            .type_mod_name("my_types")
            .substitute(
                parse_quote!(::scale_typegen::tests::Pair<T, R>),
                parse_quote!(crate::NewPair<T, R>),
            );
        let code = Testgen::new().with::<A>().gen_tests_mod(settings);
        let expected_code = quote! {
            pub mod tests {
                use super::my_types;
                pub struct A {
                    pub gen: crate::NewPair<::core::primitive::u32, ::core::primitive::u64>,
                }
            }
        };
        assert_eq!(code.to_string(), expected_code.to_string());
    }

    // replace with fixed type
    {
        let settings = TypeGeneratorSettings::default()
            .type_mod_name("my_types")
            .substitute(
                parse_quote!(::scale_typegen::tests::Pair<T, R>),
                parse_quote!(crate::NewPair),
            );
        let code = Testgen::new().with::<A>().gen_tests_mod(settings);
        let expected_code = quote! {
            pub mod tests {
                use super::my_types;
                pub struct A {
                    pub gen: crate::NewPair,
                }
            }
        };
        assert_eq!(code.to_string(), expected_code.to_string());
    }
}

#[test]
fn substitutes_and_derives() {
    // set up settings
    let settings = TypeGeneratorSettings::default()
        .type_mod_name("my_types")
        .add_derives_for_all([
            parse_quote!(::parity_scale_codec::Decode),
            parse_quote!(::parity_scale_codec::Encode),
        ])
        .substitute(
            parse_quote!(BTreeMap),
            parse_quote!(::std::collections::HashMap),
        );

    // set up types
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A {
        children: std::collections::BTreeMap<u32, A>,
    }

    // check the generated code
    let code = Testgen::new().with::<A>().gen(settings);
    let expected_code = quote! {
        pub mod my_types {
            use super::my_types;
            pub mod scale_typegen {
                use super::my_types;
                pub mod tests {
                    use super::my_types;
                    #[derive(:: parity_scale_codec :: Decode, :: parity_scale_codec :: Encode)]
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
fn can_omit_compact_encoding() {
    #[allow(unused)]
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
    #[allow(unused)]
    pub struct Centimeter(Compact<u16>);

    #[allow(unused)]
    #[derive(TypeInfo)]
    pub enum Food {
        Banana,
        Orange,
        Apple,
    }
    let settings = TypeGeneratorSettings {
        compact_type_path: Some(parse_quote!(::subxt_path::ext::codec::Compact)),
        insert_codec_attributes: false, // this results in #[codec(compact)] not being inserted for the compact field of `Centimeter`.
        ..Default::default()
    };
    let code = Testgen::new().with::<Animal>().gen(settings);

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

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

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

    assert_eq!(code.to_string(), expected_code.to_string());
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

    let code = Testgen::new()
        .with::<Parent>()
        .gen_tests_mod(subxt_settings());

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

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generate_tuple_struct() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Parent(bool, Child);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Child(i32);

    let code = Testgen::new()
        .with::<Parent>()
        .gen_tests_mod(subxt_settings());

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

    assert_eq!(code.to_string(), expected_code.to_string());
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

    let code = Testgen::new()
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
        .gen_tests_mod(subxt_settings());

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

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generate_enum() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A,
        B(bool),
        C { a: u32 },
    }

    let code = Testgen::new().with::<E>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub enum E {
                # [codec (index = 0)]
                A,
                # [codec (index = 1)]
                B (::core::primitive::bool,),
                # [codec (index = 2)]
                C { a: ::core::primitive::u32, },
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn compact_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        #[codec(compact)]
        a: u32,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct TupleStruct(#[codec(compact)] u32);

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A {
            #[codec(compact)]
            a: u32,
        },
        B(#[codec(compact)] u32),
    }

    let code = Testgen::new()
        .with::<S>()
        .with::<TupleStruct>()
        .with::<E>()
        .gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub enum E {
                # [codec (index = 0)]
                A {
                    #[codec(compact)]
                    a: ::core::primitive::u32,
                },
                # [codec (index = 1)]
                B( #[codec(compact)] ::core::primitive::u32,),
            }

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                #[codec(compact)] pub a: ::core::primitive::u32,
            }

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct TupleStruct(#[codec(compact)] pub ::core::primitive::u32,);
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn compact_generic_parameter() {
    use parity_scale_codec::Compact;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: Option<<u128 as parity_scale_codec::HasCompact>::Type>,
        nested: Option<Result<Compact<u128>, u8>>,
        vector: Vec<Compact<u16>>,
        array: [Compact<u8>; 32],
        tuple: (Compact<u8>, Compact<u16>),
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: ::core::option::Option<::subxt_path::ext::codec::Compact<::core::primitive::u128> >,
                pub nested: ::core::option::Option<::core::result::Result<::subxt_path::ext::codec::Compact<::core::primitive::u128>, ::core::primitive::u8 > >,
                pub vector: ::std::vec::Vec<::subxt_path::ext::codec::Compact<::core::primitive::u16> >,
                pub array: [::subxt_path::ext::codec::Compact<::core::primitive::u8>; 32usize],
                pub tuple: (::subxt_path::ext::codec::Compact<::core::primitive::u8>, ::subxt_path::ext::codec::Compact<::core::primitive::u16>,),
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generate_array_field() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: [u8; 32],
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: [::core::primitive::u8; 32usize],
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn option_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: Option<bool>,
        b: Option<u32>,
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: ::core::option::Option<::core::primitive::bool>,
                pub b: ::core::option::Option<::core::primitive::u32>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn box_fields_struct() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: std::boxed::Box<bool>,
        b: Box<u32>,
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: ::std::boxed::Box<::core::primitive::bool>,
                pub b: ::std::boxed::Box<::core::primitive::u32>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn box_fields_enum() {
    use std::boxed::Box;

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum E {
        A(Box<bool>),
        B { a: Box<u32> },
    }

    let code = Testgen::new().with::<E>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub enum E {
                # [codec (index = 0)]
                A(::std::boxed::Box<::core::primitive::bool>,),
                # [codec (index = 1)]
                B { a: ::std::boxed::Box<::core::primitive::u32>, },
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn range_fields() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        a: core::ops::Range<u32>,
        b: core::ops::RangeInclusive<u32>,
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub a: ::core::ops::Range<::core::primitive::u32>,
                pub b: ::core::ops::RangeInclusive<::core::primitive::u32>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generics_foo() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T> {
        a: T,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar {
        b: Foo<u32>,
        c: Foo<u8>,
    }

    let code = Testgen::new().with::<Bar>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Bar {
                pub b: root::scale_typegen::tests::Foo<::core::primitive::u32>,
                pub c: root::scale_typegen::tests::Foo<::core::primitive::u8>,
            }
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Foo<_0> {
                pub a: _0,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generics_nested() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Foo<T, U> {
        a: T,
        b: Option<(T, U)>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Bar<T> {
        b: Foo<T, u32>,
    }

    let code = Testgen::new()
        .with::<Bar<bool>>()
        .gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Bar<_0> {
                pub b: root::scale_typegen::tests::Foo<_0, ::core::primitive::u32>,
            }

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct Foo<_0, _1> {
                pub a: _0,
                pub b: ::core::option::Option<(_0, _1,)>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generate_bitvec() {
    use bitvec::{
        order::{Lsb0, Msb0},
        vec::BitVec,
    };

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        lsb: BitVec<u8, Lsb0>,
        msb: BitVec<u16, Msb0>,
    }

    let code = Testgen::new().with::<S>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub lsb: ::subxt_path::utils::bits::DecodedBits<::core::primitive::u8, ::subxt_path::utils::bits::Lsb0>,
                pub msb: ::subxt_path::utils::bits::DecodedBits<::core::primitive::u16, ::subxt_path::utils::bits::Msb0>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn generics_with_alias_adds_phantom_data_marker() {
    trait Trait {
        type Type;
    }

    impl Trait for bool {
        type Type = u32;
    }

    type Foo<T> = <T as Trait>::Type;
    type Bar<T, U> = (<T as Trait>::Type, <U as Trait>::Type);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct NamedFields<T: Trait> {
        b: Foo<T>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct UnnamedFields<T: Trait, U: Trait>(Bar<T, U>);

    let code = Testgen::new()
        .with::<NamedFields<bool>>()
        .with::<UnnamedFields<bool, bool>>()
        .gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::CompactAs, ::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct NamedFields<_0> {
                pub b: ::core::primitive::u32,
                #[codec(skip)]
                pub __ignore: ::core::marker::PhantomData<_0>
            }
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct UnnamedFields<_0, _1> (
                pub (::core::primitive::u32, ::core::primitive::u32,),
                #[codec(skip)]
                pub ::core::marker::PhantomData<(_0, _1)>
            );
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn modules() {
    mod m {
        pub mod a {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo;

            pub mod b {
                #[allow(unused)]
                #[derive(scale_info::TypeInfo)]
                pub struct Bar {
                    a: super::Foo,
                }
            }
        }

        pub mod c {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct Foo {
                a: super::a::b::Bar,
            }
        }
    }

    let code = Testgen::new()
        .with::<m::c::Foo>()
        .gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            pub mod m {
                use super::root;
                pub mod a {
                    use super::root;

                    pub mod b {
                        use super::root;

                        #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
                    #[codec(crate = ::subxt_path::ext::codec)]
                    #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
                        pub struct Bar {
                            pub a: root::scale_typegen::tests::m::a::Foo,
                        }
                    }

                    #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
                    #[codec(crate = ::subxt_path::ext::codec)]
                    #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
                    pub struct Foo;
                }

                pub mod c {
                    use super::root;

                    #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
                    #[codec(crate = ::subxt_path::ext::codec)]
                    #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
                    pub struct Foo {
                        pub a: root::scale_typegen::tests::m::a::b::Bar,
                    }
                }
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn dont_force_struct_names_camel_case() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct AB;

    let code = Testgen::new().with::<AB>().gen_tests_mod(subxt_settings());

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct AB;
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn apply_user_defined_derives_for_all_types() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A(B);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct B;

    let mut settings = subxt_settings();
    settings
        .derives
        .add_derives_for_all([parse_quote!(Clone), parse_quote!(Eq)]);
    settings
        .derives
        .add_attributes_for_all([parse_quote!(#[some_attribute])]);

    let code = Testgen::new().with::<A>().gen_tests_mod(settings);

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Clone, Debug, Eq)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            #[some_attribute]
            pub struct A(pub root :: scale_typegen :: tests :: B,);

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Clone, Debug, Eq)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            #[some_attribute]
            pub struct B;
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn apply_user_defined_derives_for_specific_types() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A(B);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct B(C);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct C;

    let mut settings = subxt_settings();
    settings.derives.add_derives_for_all([parse_quote!(Eq)]);
    settings.derives.add_derives_for(
        parse_quote!(scale_typegen::tests::B),
        [parse_quote!(Hash)],
        false,
    );
    settings.derives.add_attributes_for(
        parse_quote!(scale_typegen::tests::B),
        [parse_quote!(#[some_attribute])],
        false,
    );

    settings.derives.add_derives_for(
        parse_quote!(scale_typegen::tests::C),
        [
            parse_quote!(Eq),
            parse_quote!(Ord),
            parse_quote!(PartialOrd),
        ],
        false,
    );
    let code = Testgen::new().with::<A>().gen_tests_mod(settings);

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug, Eq)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct A(pub root :: scale_typegen :: tests :: B,);

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug, Eq, Hash)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            #[some_attribute]
            pub struct B(pub root ::scale_typegen :: tests :: C,);

            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug, Eq, Ord, PartialOrd)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct C;
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn apply_recursive_derives() {
    use std::collections::BTreeMap;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Human {
        organ_status: BTreeMap<Organ, Status>,
        profession: Profession,
        organs: Vec<Organ>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum Organ {
        Heart,
        Stomach,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct Status {
        damage: Compact<u32>,
    }

    #[allow(unused)]
    #[derive(TypeInfo)]
    enum Profession {
        Student { college: String },
        Programmer,
    }

    let mut derives = DerivesRegistry::new();

    derives.add_derives_for(
        parse_quote!(scale_typegen::tests::Human),
        vec![parse_quote!(Reflect)],
        false,
    );

    derives.add_attributes_for(
        parse_quote!(scale_typegen::tests::Human),
        vec![parse_quote!(#[is_human])],
        false,
    );

    derives.add_derives_for(
        parse_quote!(scale_typegen::tests::Human),
        vec![parse_quote!(Clone)],
        true,
    );

    derives.add_attributes_for(
        parse_quote!(scale_typegen::tests::Human),
        vec![parse_quote!(#[is_nice])],
        true,
    );

    let settings = TypeGeneratorSettings {
        derives,
        ..subxt_settings()
    };
    let code = Testgen::new().with::<Human>().gen_tests_mod(settings);

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(Clone, Reflect)]
            #[is_human]
            #[is_nice]
            pub struct Human {
                pub organ_status: ::subxt_path::utils::KeyedVec<
                    root::scale_typegen::tests::Organ,
                    root::scale_typegen::tests::Status
                >,
                pub profession: root::scale_typegen::tests::Profession,
                pub organs: ::std::vec::Vec<root::scale_typegen::tests::Organ>,
            }
            #[derive(Clone)]
            #[is_nice]
            pub enum Organ {
                #[codec(index = 0)]
                Heart,
                #[codec(index = 1)]
                Stomach,
            }
            #[derive(Clone)]
            #[is_nice]
            pub enum Profession {
                #[codec(index = 0)]
                Student { college: ::std::string::String , },
                #[codec(index = 1)]
                Programmer,
            }
            #[derive(Clone)]
            #[is_nice]
            pub struct Status {
                #[codec(compact)]
                pub damage: ::core::primitive::u32,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn apply_derives() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct A(B);

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct B;

    let mut derives = DerivesRegistry::new();
    derives.add_derives_for_all(vec![parse_quote!(Clone), parse_quote!(Eq)]);
    derives.add_attributes_for_all(vec![parse_quote!(#[some_attribute])]);

    derives.add_derives_for(
        parse_quote!(scale_typegen::tests::B),
        vec![parse_quote!(Hash)],
        false,
    );

    derives.add_attributes_for(
        parse_quote!(scale_typegen::tests::B),
        vec![parse_quote!(#[some_other_attribute])],
        false,
    );

    let settings = TypeGeneratorSettings {
        derives,
        ..subxt_settings()
    };
    let code = Testgen::new().with::<A>().gen_tests_mod(settings);

    let expected_code = quote! {
        pub mod tests {
            use super::root;

            #[derive(Clone, Eq)]
            #[some_attribute]
            pub struct A(pub root :: scale_typegen :: tests :: B,);

            #[derive(Clone, Eq, Hash)]
            #[some_attribute]
            #[some_other_attribute]
            pub struct B;
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

/// By default a BTreeMap would be replaced by a KeyedVec.
/// This test demonstrates that it does not happen if we opt out of default type substitutes.
#[test]
fn apply_custom_substitutes() {
    use std::collections::BTreeMap;

    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S {
        map: BTreeMap<u8, u8>,
    }

    let substitutes = TypeSubstitutes::new();

    let settings = TypeGeneratorSettings {
        substitutes,
        ..subxt_settings()
    };
    let code = Testgen::new().with::<S>().gen_tests_mod(settings);

    let expected_code = quote! {
        pub mod tests {
            use super::root;
            #[derive(::subxt_path::ext::codec::Decode, ::subxt_path::ext::codec::Encode, ::subxt_path::ext::scale_decode::DecodeAsType, ::subxt_path::ext::scale_encode::EncodeAsType, Debug)]
            #[codec(crate = ::subxt_path::ext::codec)]
            #[decode_as_type(crate_path = ":: subxt_path :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_path :: ext :: scale_encode")]
            pub struct S {
                pub map: ::std::collections::BTreeMap<:: core :: primitive :: u8,:: core :: primitive :: u8>,
            }
        }
    };

    assert_eq!(code.to_string(), expected_code.to_string());
}

#[test]
fn ensure_unique_type_paths_test() {
    use scale_info::PortableRegistry;

    #[allow(unused)]
    #[derive(TypeInfo)]
    #[scale_info(skip_type_params(Hash))]
    struct Header<Hash, AccountId> {
        id: AccountId,
        hash: Hash,
    }

    let mut registry = Testgen::new()
        .with::<Header<String, u32>>()
        .with::<Header<u8, u32>>()
        .with::<Header<u16, u32>>()
        .into_portable_registry();

    let sorted_type_paths = |registry: &PortableRegistry| -> Vec<String> {
        let mut lines = registry
            .types
            .iter()
            .map(|t| t.ty.path.segments.clone().join("::"))
            .filter(|e| !e.is_empty())
            .collect::<Vec<String>>();
        lines.sort();
        lines
    };

    let e1 = sorted_type_paths(&registry);
    assert_eq!(
        e1,
        vec![
            "scale_typegen::tests::Header",
            "scale_typegen::tests::Header",
            "scale_typegen::tests::Header",
        ]
    );

    ensure_unique_type_paths(&mut registry).expect("Corrupted PortableRegistry");

    let e2 = sorted_type_paths(&registry);
    assert_eq!(
        e2,
        vec![
            "scale_typegen::tests::Header1",
            "scale_typegen::tests::Header2",
            "scale_typegen::tests::Header3",
        ]
    );
}

#[test]
fn validation_errors() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S;

    let registry = Testgen::new().with::<S>().into_portable_registry();
    let mut settings = TypeGeneratorSettings::new();
    settings.derives = {
        let mut d = DerivesRegistry::new();
        d.add_derives_for(
            parse_quote!(scale_typegen::tests::S),
            [parse_quote!(Clone), parse_quote!(Reflect)],
            true,
        );

        d.add_attributes_for(
            parse_quote!(scale_typegen::tests::S),
            [parse_quote!(#[nice])],
            true,
        );

        d.add_derives_for(
            parse_quote!(scale_typegen::tests::T),
            [parse_quote!(Clone), parse_quote!(Reflect)],
            true,
        );

        d.add_attributes_for(
            parse_quote!(scale_typegen::tests::T),
            [parse_quote!(#[nice])],
            true,
        );
        d
    };
    settings = settings
        .substitute(
            parse_quote!(scale_typegen::tests::S),
            parse_quote!(::hello::S),
        )
        .substitute(
            parse_quote!(scale_typegen::tests::T),
            parse_quote!(::hello::T),
        );

    let err = validate_substitutes_and_derives_against_registry(
        &settings.substitutes,
        &settings.derives,
        &registry,
    )
    .unwrap_err();

    // we expect only errors for type T because S is in the registry, T is not.
    let expected_err = SettingsValidationError {
        derives_for_unknown_types: vec![(
            parse_quote!(scale_typegen::tests::T),
            [parse_quote!(Clone), parse_quote!(Reflect)].into(),
        )],
        attributes_for_unknown_types: vec![(
            parse_quote!(scale_typegen::tests::T),
            [parse_quote!(#[nice])].into(),
        )],
        substitutes_for_unknown_types: vec![(
            parse_quote!(scale_typegen::tests::T),
            parse_quote!(::hello::T),
        )],
    };

    assert_eq!(err, expected_err);
}

#[test]
fn find_similar_type_paths() {
    #[allow(unused)]
    #[derive(TypeInfo)]
    struct S;
    mod types {
        #[allow(unused)]
        #[derive(scale_info::TypeInfo)]
        pub struct T;

        #[allow(unused)]
        #[derive(scale_info::TypeInfo)]
        pub struct S;

        pub mod abc {
            #[allow(unused)]
            #[derive(scale_info::TypeInfo)]
            pub struct S;
        }
    }

    let registry = Testgen::new()
        .with::<S>()
        .with::<types::S>()
        .with::<types::T>()
        .with::<types::abc::S>()
        .into_portable_registry();

    // user gives the wrong type location types::xyz::S => show them suggestions for correct paths.
    let similar = similar_type_paths_in_registry(&registry, &parse_quote!(types::xyz::S));

    let expected: Vec<syn::Path> = vec![
        parse_quote!(scale_typegen::tests::S),
        parse_quote!(scale_typegen::tests::types::S),
        parse_quote!(scale_typegen::tests::types::abc::S),
    ];
    assert_eq!(similar, expected);
}

#[test]
fn assoc_types_skip_params() {
    pub trait Config {
        type Inner: TypeInfo;
    }

    pub struct A;
    pub struct B;

    impl Config for A {
        type Inner = ();
    }

    impl Config for B {
        type Inner = u32;
    }

    #[derive(TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct X<T: Config> {
        #[allow(dead_code)]
        pub inner: T::Inner,
    }

    let res_no_dedup = Testgen::new()
        .with::<X<A>>()
        .with::<X<B>>()
        .try_gen_tests_mod(subxt_settings(), false);
    assert!(matches!(
        res_no_dedup,
        Err(TypegenError::DuplicateTypePath(_))
    ));

    let dedup_code = Testgen::new()
        .with::<X<A>>()
        .with::<X<B>>()
        .try_gen_tests_mod(Default::default(), true)
        .unwrap();

    let expected_code = quote! {
        pub mod tests {
            use super::types;
            pub struct X1 {
                pub inner: (),
            }
            pub struct X2 {
                pub inner: ::core::primitive::u32,
            }
        }
    };

    assert_eq!(dedup_code.to_string(), expected_code.to_string());
}

#[test]
fn assoc_types_no_skip_params() {
    pub trait Config {
        type Inner: TypeInfo;
    }

    #[derive(TypeInfo)]
    pub struct A;

    #[derive(TypeInfo)]
    pub struct B;

    impl Config for A {
        type Inner = ();
    }
    impl Config for B {
        type Inner = u32;
    }

    #[derive(TypeInfo)]
    pub struct X<T: Config> {
        #[allow(dead_code)]
        pub inner: T::Inner,
    }

    let res_no_dedup = Testgen::new()
        .with::<X<A>>()
        .with::<X<B>>()
        .try_gen_tests_mod(subxt_settings(), false);
    assert!(matches!(
        res_no_dedup,
        Err(TypegenError::DuplicateTypePath(_))
    ));

    let dedup_code = Testgen::new()
        .with::<X<A>>()
        .with::<X<B>>()
        .try_gen_tests_mod(Default::default(), true)
        .unwrap();

    // Unfortunately the structs A and B are still part of the type registry and X1 and X2 have generic parameters, linking to them.
    let expected_code = quote! {
        pub mod tests {
            use super::types;
            pub struct A;
            pub struct B;
            pub struct X1<_0> {
                pub inner: (),
                pub __ignore: ::core::marker::PhantomData<_0>
            }
            pub struct X2<_0> {
                pub inner: ::core::primitive::u32,
                pub __ignore: ::core::marker::PhantomData<_0>
            }
        }
    };

    assert_eq!(dedup_code.to_string(), expected_code.to_string());
}

#[test]
fn alloc_crate_path_replacement() {
    use std::collections::BTreeMap;

    #[derive(TypeInfo)]
    #[allow(unused)]
    pub struct Foo {
        a: Vec<Foo>,
        b: Box<u8>,
        c: String,
        e: BTreeMap<String, Box<u8>>,
    }

    let std_code = Testgen::new()
        .with::<Foo>()
        .try_gen_tests_mod(Default::default(), false)
        .unwrap();

    let no_std_code = Testgen::new()
        .with::<Foo>()
        .try_gen_tests_mod(
            TypeGeneratorSettings {
                alloc_crate_path: AllocCratePath::Custom(parse_quote!(::subxt_core::alloc)),
                ..Default::default()
            },
            false,
        )
        .unwrap();

    let expected_std_code = quote! {
        pub mod tests {
            use super::types;
            pub struct Foo {
                pub a: ::std::vec::Vec<types::scale_typegen::tests::Foo>,
                pub b: ::std::boxed::Box<::core::primitive::u8>,
                pub c: ::std::string::String,
                pub e: ::std::boxed::Box<
                    ::std::collections::BTreeMap<::std::string::String, ::core::primitive::u8>
                >,
            }
        }
    };

    let expected_no_std_code = quote! {
        pub mod tests {
            use super::types;
            pub struct Foo {
                pub a: ::subxt_core::alloc::vec::Vec<types::scale_typegen::tests::Foo>,
                pub b: ::subxt_core::alloc::boxed::Box<::core::primitive::u8>,
                pub c: ::subxt_core::alloc::string::String,
                pub e: ::subxt_core::alloc::boxed::Box<
                    ::subxt_core::alloc::collections::BTreeMap<
                        ::subxt_core::alloc::string::String,
                        ::core::primitive::u8
                    >
                >,
            }
        }
    };

    assert_eq!(std_code.to_string(), expected_std_code.to_string());
    assert_eq!(no_std_code.to_string(), expected_no_std_code.to_string());
}
