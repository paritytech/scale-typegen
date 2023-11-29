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

#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]

// Because of `unused_crate_dependencies` flag:
#[cfg(test)]
use parity_scale_codec as _;

mod description;
mod formatting;
mod transformer;
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

    use pretty_assertions::assert_eq;
    use scale_info::{PortableRegistry, TypeInfo};

    use crate::type_description;

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
            "Human {
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
            MultiShape {
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
            "enum Shape {
                Inivisible,
                Circle(u64),
                Rect(Compact<u64>, Compact<u64>),
                Polygon  {
                    corners: u8,
                    radius: u64
                },
                MultiShape  {
                    shapes: Vec<
                        enum Shape {
                            Inivisible,
                            Circle(u64),
                            Rect(Compact<u64>, Compact<u64>),
                            Polygon  {
                                corners: u8,
                                radius: u64
                            },
                            MultiShape  {
                                shapes: Vec<Shape>,
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
                    operation: enum Operation {
                        Add,
                        Intersect,
                        Difference
                    }
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
            "Human {
                name: String,
                friends: Vec<Human>,
                dad: Box<Human>,
                home: House {
                    inhabitants: Vec<Human>
                }
            }"}
        );
    }

    #[test]
    fn recursive_containers() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Container {
            shapes: Vec<S>,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct S {
            u: u8,
            others: Vec<S>,
        }

        let (type_id, type_registry) = make_type::<Container>();

        assert_eq!(
            type_description(type_id, &type_registry, true).unwrap(),
            indoc! {
            "Container {
                shapes: Vec<S {
                    u: u8,
                    others: Vec<S>
                }>
            }"}
        );
    }
}
