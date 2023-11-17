mod rust_type;
mod scale_value;

pub use rust_type::rust_type_example;
pub use scale_value::scale_value_example;

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use parity_scale_codec::Compact;
    use pretty_assertions::assert_eq;
    use scale_info::{PortableRegistry, TypeInfo};

    use crate::type_example::scale_value::scale_value_example_from_seed;

    use super::rust_type_example;
    use super::scale_value_example;

    fn make_type<T: TypeInfo + 'static>() -> (u32, PortableRegistry) {
        let mut registry = scale_info::Registry::new();
        let m = scale_info::MetaType::new::<T>();
        let ty = registry.register_type(&m);
        (ty.id, registry.into())
    }

    /// for certain types we cannot create any valid scale_value type examples because they are infinitely nested. We need to return an error in those cases.
    /// Otherwise we would get a stack overflow and the program crashes...
    #[test]
    fn recursion_does_not_cause_stack_overflow() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Human {
            name: String,
            mom: Box<Human>,
            dad: Box<Human>,
        }
        let (id, types) = make_type::<Human>();
        // Make sure recursion does not panic. An error should be yielded instead.
        assert!(scale_value_example(id, &types).is_err());
    }

    #[test]
    fn seeding_works() {
        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Human {
            name: String,
            age: u32,
            male: bool,
            eye_color: Color,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        enum Color {
            Black,
            White,
            Green(i32),
        }

        let (id, types) = make_type::<Human>();

        let a1 = scale_value_example_from_seed(id, &types, 20).unwrap();
        let a2 = scale_value_example_from_seed(id, &types, 20).unwrap();
        let a3 = scale_value_example_from_seed(id, &types, 20).unwrap();
        let b1 = scale_value_example_from_seed(id, &types, 30).unwrap();
        let b2 = scale_value_example_from_seed(id, &types, 30).unwrap();

        // The examples can be checked manually by comparing them side by side:
        // println!("{b2}\n{a1}");

        assert_eq!(a1, a2);
        assert_eq!(a1, a3);
        assert_eq!(b1, b2);
        assert_ne!(a1, b1);
    }
}
