// Copyright 2019-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A crate for turning a type from a [`scale_info::PortableRegistry`] into some other, fully resolved, tree-like representation.
//! Currently we can generate these representations for a type:
//! - A human readable description of the type via [`crate::type_description`].
//! - An exemplary rust value of the type via [`crate::rust_value`].
//! - An exemplary scale value of the type via [`crate::scale_value`].

mod description;
mod formatting;
pub mod transformer;

#[cfg(feature = "type-example")]
pub use scale_typegen;

/// Create type examples for a type registry.
#[cfg(feature = "type-example")]
pub mod type_example;

#[cfg(feature = "type-example")]
pub use type_example::{
    rust_value::{example as rust_value, example_from_seed as rust_value_from_seed},
    scale_value::{example as scale_value, example_from_seed as scale_value_from_seed},
};

pub use description::type_description;
pub use formatting::format_type_description;

#[cfg(test)]
mod tests {
    // Note: indoc is used to make it easier to represent multi-line strings.
    use indoc::indoc;

    use parity_scale_codec::Compact;
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use scale_info::{PortableRegistry, TypeInfo};
    use scale_typegen::TypeGeneratorSettings;

    use crate::{type_description, type_example::rust_value};

    fn make_type<T: TypeInfo + 'static>() -> (u32, PortableRegistry) {
        let mut registry = scale_info::Registry::new();
        let m = scale_info::MetaType::new::<T>();
        let ty = registry.register_type(&m);
        (ty.id, registry.into())
    }

    #[test]
    fn structs() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Human {
            name: String,
            age: u32,
            male: bool,
        }

        let (type_id, type_registry) = make_type::<Human>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct Human {
                name: String,
                age: u32,
                male: bool
            }"}
        );
    }

    #[test]
    fn enums() {
        use parity_scale_codec::Compact;

        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Shape<T> {
            Inivisible,
            Circle(u64),
            Rect(Compact<u64>, Compact<u64>),
            Polygon {
                corners: u8,
                radius: u64,
            },
            Multi {
                shapes: Vec<Shape<u64>>,
                t: T,
                operation: Operation,
            },
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Operation {
            Add,
            Intersect,
            Difference,
        }

        let (type_id, type_registry) = make_type::<Shape<u8>>();
        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "enum Shape<u8> {
                Inivisible,
                Circle(u64),
                Rect(Compact<u64>, Compact<u64>),
                Polygon {
                    corners: u8,
                    radius: u64
                },
                Multi {
                    shapes: Vec<
                        enum Shape<u64> {
                            Inivisible,
                            Circle(u64),
                            Rect(Compact<u64>, Compact<u64>),
                            Polygon {
                                corners: u8,
                                radius: u64
                            },
                            Multi {
                                shapes: Vec<Shape<u64>>,
                                t: u64,
                                operation: enum Operation {
                                    Add,
                                    Intersect,
                                    Difference
                                }
                            }
                        }
                    >,
                    t: u8,
                    operation: Operation
                }
            }"}
        );
    }

    #[test]
    fn recursive_structs() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Human {
            name: String,
            friends: Vec<Human>,
            dad: Box<Human>,
            home: House,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct House {
            inhabitants: Vec<Human>,
        }

        let (type_id, type_registry) = make_type::<Human>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct Human {
                name: String,
                friends: Vec<Human>,
                dad: Box<Human>,
                home: struct House {
                    inhabitants: Vec<Human>
                }
            }"}
        );
    }

    #[test]
    fn recursive_containers() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct A {
            bees: Vec<B>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct B {
            id: u8,
            others: Vec<B>,
        }

        let (type_id, type_registry) = make_type::<A>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct A {
                bees: Vec<
                    struct B {
                        id: u8,
                        others: Vec<B>
                    }
                >
            }"}
        );
    }

    #[test]
    fn recursive_generics() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Vec2<T> {
            x: Box<T>,
            y: Box<T>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct A {
            bees: Vec2<B>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct B {
            id: u8,
            others: Vec2<B>,
        }

        let (type_id, type_registry) = make_type::<A>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct A {
                bees: struct Vec2<B> {
                    x: Box<
                        struct B {
                            id: u8,
                            others: Vec2<B>
                        }
                    >,
                    y: Box<B>
                }
            }"}
        );
    }

    #[test]
    fn multiple_fields_with_same_type() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct A {
            x: B,
            y: B,
            z: B,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct B {
            id: u8,
        }

        let (type_id, type_registry) = make_type::<A>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct A {
                x: struct B {
                    id: u8
                },
                y: B,
                z: B
            }"}
        );
    }

    #[test]
    fn tuple_fields_with_same_type() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct A {
            tup: (B, B, B),
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct B {
            id: u8,
        }

        let (type_id, type_registry) = make_type::<A>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "struct A {
                tup: (
                    struct B {
                        id: u8
                    },
                    B,
                    B
                )
            }"}
        );
    }

    #[test]
    fn rust_value_compact() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S0 {
            #[codec(compact)]
            n: u8,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S1 {
            n: Compact<u8>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct T0(#[codec(compact)] u8);

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct T1(Compact<u8>);

        use quote::quote;
        use syn::parse_quote;

        fn get_example<T: TypeInfo + 'static>() -> TokenStream {
            let (type_id, type_registry) = make_type::<T>();
            let settings = TypeGeneratorSettings::new().compact_type_path(parse_quote!(Compact));
            rust_value::example(type_id, &type_registry, &settings).unwrap()
        }

        // Note: The 161 is pretty random and depends on the default seed of the RNG.
        assert_eq!(
            get_example::<S0>().to_string(),
            quote! {types::scale_typegen_description::tests::S0{ n: 161u8, }}.to_string()
        );
        assert_eq!(
            get_example::<S1>().to_string(),
            quote! {types::scale_typegen_description::tests::S1{ n: Compact(161u8), }}.to_string()
        );
        assert_eq!(
            get_example::<T0>().to_string(),
            quote! {types::scale_typegen_description::tests::T0( 161u8, )}.to_string()
        );
        assert_eq!(
            get_example::<T1>().to_string(),
            quote! {types::scale_typegen_description::tests::T1( Compact(161u8), )}.to_string()
        );
    }
}
