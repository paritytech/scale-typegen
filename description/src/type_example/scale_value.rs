use std::cell::RefCell;

use anyhow::anyhow;
use rand::SeedableRng;
use rand::{seq::SliceRandom, Rng};

use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef, TypeDefPrimitive};
use scale_value::{BitSequence, Composite, Primitive, Value, ValueDef, Variant};

use crate::transformer::Transformer;

type ValueTransformer<'a> = Transformer<'a, Value, RefCell<rand_chacha::ChaCha8Rng>>;

/// Generates a random scale value for a type from the registry.
pub fn example(id: u32, types: &PortableRegistry) -> anyhow::Result<Value> {
    const MAGIC_SEED: u64 = 42;
    example_from_seed(id, types, MAGIC_SEED)
}

/// Generates a random scale value for a type from the registry. You can specify the seed to get reproducable results.
pub fn example_from_seed(id: u32, types: &PortableRegistry, seed: u64) -> anyhow::Result<Value> {
    fn error_on_recurse(
        _type_id: u32,
        ty: &Type<PortableForm>,
        _transformer: &ValueTransformer,
    ) -> Option<anyhow::Result<Value>> {
        Some(Err(anyhow!(
            "Cannot generate scale value example for recursive type: {ty:?}"
        )))
    }

    /// Note: because None is returned here, the transformer will just continue its work.
    fn compute_another_example(
        _type_id: u32,
        _ty: &Type<PortableForm>,
        _cached_value: &Value,
        _transformer: &ValueTransformer,
    ) -> Option<anyhow::Result<Value>> {
        None
    }

    let state = RefCell::new(rand_chacha::ChaCha8Rng::seed_from_u64(seed));
    let transformer = ValueTransformer::new(
        ty_example,
        error_on_recurse,
        compute_another_example,
        state,
        types,
    );
    transformer.resolve(id)
}

fn ty_example(
    _type_id: u32,
    ty: &Type<PortableForm>,
    transformer: &ValueTransformer,
) -> anyhow::Result<Value> {
    match &ty.type_def {
        TypeDef::Composite(composite) => {
            let fields = composite.fields.iter().map(|e| (e.name.as_ref(), e.ty.id));
            let composite = fields_type_example(fields, transformer)?;
            Ok(Value {
                value: ValueDef::Composite(composite),
                context: (),
            })
        }
        TypeDef::Variant(variant) => {
            let random_variant = variant
                .variants
                .choose(&mut *transformer.state().borrow_mut())
                .ok_or_else(|| anyhow!("Variant type should have at least one variant"))?;
            let fields = random_variant
                .fields
                .iter()
                .map(|e| (e.name.as_ref(), e.ty.id));
            let composite = fields_type_example(fields, transformer)?;
            Ok(Value {
                value: ValueDef::Variant(Variant {
                    name: random_variant.name.clone(),
                    values: composite,
                }),
                context: (),
            })
        }
        TypeDef::Sequence(sequence) => {
            // sequence with 2 elements should be enough:
            let example1 = transformer.resolve(sequence.type_param.id)?;
            let example2 = transformer.resolve(sequence.type_param.id)?;
            Ok(Value::unnamed_composite([example1, example2]))
        }
        TypeDef::Array(array) => {
            let elements: Vec<_> = (0..array.len)
                .map(|_| transformer.resolve(array.type_param.id))
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(Value::unnamed_composite(elements))
        }
        TypeDef::Tuple(tuple) => {
            let fields = tuple.fields.iter().map(|e| (None::<&str>, e.id));
            let composite = fields_type_example(fields, transformer)?;
            Ok(Value {
                value: ValueDef::Composite(composite),
                context: (),
            })
        }
        TypeDef::Primitive(primitive) => Ok(primitive_type_def_example(
            primitive,
            &mut *transformer.state().borrow_mut(),
        )),
        TypeDef::Compact(compact) => transformer.resolve(compact.type_param.id),
        TypeDef::BitSequence(_) => {
            let mut bit_sequence = BitSequence::new();
            let rng = &mut *transformer.state().borrow_mut();
            for _ in 0..rng.gen_range(3..7) {
                bit_sequence.push(rng.gen());
            }
            Ok(Value::bit_sequence(bit_sequence))
        }
    }
}

fn primitive_type_def_example(primitive: &TypeDefPrimitive, rng: &mut impl rand::Rng) -> Value {
    let primitive: Primitive = match primitive {
        TypeDefPrimitive::Bool => Primitive::Bool(rng.gen()),
        TypeDefPrimitive::Char => {
            Primitive::Char(*['a', 'b', 'c', 'd', 'e', 'f', 'g'].choose(rng).unwrap())
        }
        TypeDefPrimitive::Str => {
            Primitive::String((*["Foo", "Bar", "Fizz", "Buzz"].choose(rng).unwrap()).into())
        }
        TypeDefPrimitive::U8 => Primitive::U128(rng.gen::<u8>() as u128),
        TypeDefPrimitive::U16 => Primitive::U128(rng.gen::<u16>() as u128),
        TypeDefPrimitive::U32 => Primitive::U128(rng.gen::<u32>() as u128),
        TypeDefPrimitive::U64 => Primitive::U128(rng.gen::<u64>() as u128),
        TypeDefPrimitive::U128 => Primitive::U128(rng.gen()),
        TypeDefPrimitive::U256 => Primitive::U256(rng.gen()),
        TypeDefPrimitive::I8 => Primitive::I128(rng.gen::<i8>() as i128),
        TypeDefPrimitive::I16 => Primitive::I128(rng.gen::<i16>() as i128),
        TypeDefPrimitive::I32 => Primitive::I128(rng.gen::<i32>() as i128),
        TypeDefPrimitive::I64 => Primitive::I128(rng.gen::<i64>() as i128),
        TypeDefPrimitive::I128 => Primitive::I128(rng.gen()),
        TypeDefPrimitive::I256 => Primitive::I256(rng.gen()),
    };
    Value::primitive(primitive)
}

fn fields_type_example(
    fields: impl Iterator<Item = (Option<impl AsRef<str>>, u32)> + Clone,
    transformer: &ValueTransformer,
) -> anyhow::Result<Composite<()>> {
    let all_fields_named = fields.clone().all(|e| e.0.is_some());
    let all_fields_unnamed = fields.clone().all(|e| e.0.is_none());
    match (all_fields_named, all_fields_unnamed) {
        (true, true) => Ok(Composite::Unnamed(vec![])),
        (true, false) => {
            let mut elements: Vec<(String, Value)> = vec![];
            for (name, id) in fields {
                let field_value = transformer.resolve(id)?;
                let name = name.unwrap().as_ref().into();
                elements.push((name, field_value));
            }
            Ok(Composite::named(elements))
        }
        (false, true) => {
            let mut elements = vec![];
            for (_, id) in fields {
                let field_value = transformer.resolve(id)?;
                elements.push(field_value);
            }
            Ok(Composite::unnamed(elements))
        }
        (false, false) => Err(anyhow!(
            "Composite should not have unnamed and named fields"
        )),
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use scale_info::{PortableRegistry, TypeInfo};

    use crate::formatting::format_type_description;

    use super::example;
    use super::example_from_seed;

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
        assert!(example(id, &types).is_err());
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

        let s = example_from_seed(id, &types, 2).unwrap().to_string();
        let s = format_type_description(&s);
        println!("{}", s);

        let a1 = example_from_seed(id, &types, 2).unwrap();
        let a2 = example_from_seed(id, &types, 2).unwrap();
        let a3 = example_from_seed(id, &types, 2).unwrap();
        let b1 = example_from_seed(id, &types, 3).unwrap();
        let b2 = example_from_seed(id, &types, 3).unwrap();

        // The examples can be checked manually by comparing them side by side:
        // println!("{b2}\n{a1}");

        assert_eq!(a1, a2);
        assert_eq!(a1, a3);
        assert_eq!(b1, b2);
        assert_ne!(a1, b1);
    }
}
