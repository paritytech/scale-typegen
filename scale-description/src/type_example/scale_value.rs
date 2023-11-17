use std::cell::RefCell;

use anyhow::anyhow;
use rand::SeedableRng;
use rand::{seq::SliceRandom, Rng};

use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef, TypeDefPrimitive};
use scale_value::{BitSequence, Composite, Primitive, Value, ValueDef, Variant};

use crate::transformer::Transformer;

type ValueTransformer<'a> = Transformer<'a, Value, RefCell<rand_chacha::ChaCha8Rng>>;

pub fn scale_value_example(id: u32, types: &PortableRegistry) -> anyhow::Result<Value> {
    const MAGIC_SEED: u64 = 42;
    scale_value_example_from_seed(id, types, MAGIC_SEED)
}

pub fn scale_value_example_from_seed(
    id: u32,
    types: &PortableRegistry,
    seed: u64,
) -> anyhow::Result<Value> {
    fn error_on_recurse(ty: &Type<PortableForm>) -> anyhow::Result<Value> {
        Err(anyhow!(
            "Cannot generate scale value example for recursive type: {ty:?}"
        ))
    }
    let state = RefCell::new(rand_chacha::ChaCha8Rng::seed_from_u64(seed));
    let transformer = ValueTransformer::new(type_example, error_on_recurse, state, types);
    transformer.resolve(id)
}

fn type_example(ty: &Type<PortableForm>, transformer: &ValueTransformer) -> anyhow::Result<Value> {
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
            let first = variant
                .variants
                .choose(&mut *transformer.state().borrow_mut())
                .ok_or_else(|| anyhow!("Variant type should have at least one variant"))?;
            let fields = first.fields.iter().map(|e| (e.name.as_ref(), e.ty.id));
            let composite = fields_type_example(fields, transformer)?;
            Ok(Value {
                value: ValueDef::Variant(Variant {
                    name: first.name.clone(),
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
