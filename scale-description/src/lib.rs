mod description;
mod formatting;

#[cfg(feature = "type-example")]
pub mod type_example;

pub use description::type_description;

#[cfg(test)]
mod tests {
    // Note: indoc is used to make it easier to represent multi-line strings.
    use indoc::indoc;
    use parity_scale_codec::Compact;
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
            type_description(type_id, &type_registry).unwrap(),
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
        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Shape {
            Inivisible,
            Circle(u64),
            Rect(Compact<u64>, Compact<u64>),
            Polygon {
                corners: u8,
                radius: u64,
            },
            MultiShape {
                shapes: Vec<Shape>,
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

        let (type_id, type_registry) = make_type::<Shape>();

        assert_eq!(
            type_description(type_id, &type_registry).unwrap(),
            indoc! {
            "enum Shape {
                Inivisible,
                Circle(u64),
                Rect(
                    Compact<u64>,
                    Compact<u64>
                ),
                Polygon  {
                    corners: u8,
                    radius: u64
                },
                MultiShape  {
                    shapes: Vec<Shape>,
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
            type_description(type_id, &type_registry).unwrap(),
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
}
