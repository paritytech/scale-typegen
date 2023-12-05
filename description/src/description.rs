use anyhow::anyhow;
use scale_info::{
    form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeDefArray, TypeDefBitSequence,
    TypeDefCompact, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};

use crate::transformer::Transformer;

use super::formatting::format_type_description;

/// Describes the type that is registered under the given `type_id`. This type description
/// is supposed to be very close to actual rust types, with some minar differences:
/// - The `struct` keyword is omitted. So the description of `struct Human { age: u8 }` is just `Human { age: u8 }`.
/// - Types are presented in a nested fashion, similar to how structures can be defined in e.g. the C programming language.
///
/// If the `format` flag is enabled, the end result is formatted across multiple lines. Otherwise the description will be one single line string.
pub fn type_description(
    type_id: u32,
    type_registry: &PortableRegistry,
    format: bool,
) -> anyhow::Result<String> {
    fn return_type_name_on_recurse(
        _type_id: u32,
        ty: &Type<PortableForm>,
        _transformer: &Transformer<String>,
    ) -> anyhow::Result<String> {
        if let Some(type_name) = ty.path.ident() {
            return Ok(type_name);
        }
        Err(anyhow!("Recursive type that did not get handled properly"))
    }

    let transformer = Transformer::new(
        ty_description,
        return_type_name_on_recurse,
        (),
        type_registry,
    );
    let mut description = transformer.resolve(type_id)?;
    if format {
        description = format_type_description(&description);
    }
    Ok(description)
}

fn ty_description(
    _type_id: u32,
    ty: &Type<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let ident = ty.path.ident().unwrap_or_default();
    let prefix = match &ty.type_def {
        TypeDef::Variant(_) => "enum ",
        TypeDef::Composite(_) => "struct ",
        _ => "",
    };
    let type_def_description = type_def_type_description(&ty.type_def, transformer)?;
    Ok(format!("{prefix}{ident}{type_def_description}"))
}

fn type_def_type_description(
    type_def: &TypeDef<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    match type_def {
        TypeDef::Composite(composite) => fields_type_description(&composite.fields, transformer),

        TypeDef::Variant(variant) => variant_type_def_type_description(variant, transformer),
        TypeDef::Sequence(sequence) => sequence_type_description(sequence, transformer),
        TypeDef::Array(array) => array_type_description(array, transformer),
        TypeDef::Tuple(tuple) => tuple_type_description(tuple, transformer),
        TypeDef::Primitive(primitive) => primitive_type_description(primitive),
        TypeDef::Compact(compact) => compact_type_description(compact, transformer),
        TypeDef::BitSequence(bit_sequence) => {
            bit_sequence_type_description(bit_sequence, transformer)
        }
    }
}

fn tuple_type_description(
    tuple: &TypeDefTuple<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let mut output = "(".to_string();
    let mut iter = tuple.fields.iter().peekable();
    while let Some(ty) = iter.next() {
        let type_description = transformer.resolve(ty.id)?;
        output.push_str(&type_description);
        if iter.peek().is_some() {
            output.push(',')
        }
    }
    output.push(')');
    Ok(output)
}

fn bit_sequence_type_description(
    bit_sequence: &TypeDefBitSequence<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let bit_order_type = transformer.resolve(bit_sequence.bit_order_type.id)?;
    let bit_store_type = transformer.resolve(bit_sequence.bit_store_type.id)?;
    Ok(format!("BitSequence({bit_order_type}, {bit_store_type})"))
}

fn sequence_type_description(
    sequence: &TypeDefSequence<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let type_description = transformer.resolve(sequence.type_param.id)?;

    Ok(format!("Vec<{type_description}>"))
}

fn compact_type_description(
    compact: &TypeDefCompact<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let type_description = transformer.resolve(compact.type_param.id)?;
    Ok(format!("Compact<{type_description}>"))
}

fn array_type_description(
    array: &TypeDefArray<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let type_description = transformer.resolve(array.type_param.id)?;
    Ok(format!("[{type_description}; {}]", array.len))
}

fn primitive_type_description(primitive: &TypeDefPrimitive) -> anyhow::Result<String> {
    Ok(match &primitive {
        TypeDefPrimitive::Bool => "bool",
        TypeDefPrimitive::Char => "char",
        TypeDefPrimitive::Str => "String",
        TypeDefPrimitive::U8 => "u8",
        TypeDefPrimitive::U16 => "u16",
        TypeDefPrimitive::U32 => "u32",
        TypeDefPrimitive::U64 => "u64",
        TypeDefPrimitive::U128 => "u128",
        TypeDefPrimitive::U256 => "u256",
        TypeDefPrimitive::I8 => "i8",
        TypeDefPrimitive::I16 => "i16",
        TypeDefPrimitive::I32 => "i32",
        TypeDefPrimitive::I64 => "i64",
        TypeDefPrimitive::I128 => "i128",
        TypeDefPrimitive::I256 => "i256",
    }
    .into())
}

fn variant_type_def_type_description(
    variant_type_def: &TypeDefVariant<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let mut variants_string = String::new();
    variants_string.push('{');
    let mut iter = variant_type_def.variants.iter().peekable();
    while let Some(variant) = iter.next() {
        let variant_string = variant_type_description(variant, transformer)?;
        variants_string.push_str(&variant_string);

        if iter.peek().is_some() {
            variants_string.push(',');
        }
    }
    variants_string.push('}');
    Ok(variants_string)
}

fn variant_type_description(
    variant: &Variant<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let fields_string = fields_type_description(&variant.fields, transformer)?;
    let output = if fields_string == "()" {
        variant.name.to_string()
    } else {
        format!("{}{}", &variant.name, fields_string)
    };
    Ok(output)
}

fn fields_type_description(
    fields: &[Field<PortableForm>],
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    if fields.is_empty() {
        return Ok("()".to_string());
    }

    let all_fields_named = fields.iter().all(|f| f.name.is_some());
    let all_fields_unnamed = fields.iter().all(|f| f.name.is_none());
    let brackets = match (all_fields_named, all_fields_unnamed) {
        (true, false) => ('{', '}'),
        (false, true) => ('(', ')'),
        _ => {
            return Err(anyhow::anyhow!(
                "combination of named and unnamed fields in compound type"
            ));
        }
    };

    let mut fields_string = String::new();
    fields_string.push(brackets.0);
    let mut iter = fields.iter().peekable();
    while let Some(field) = iter.next() {
        let field_description = field_type_description(field, transformer)?;
        fields_string.push_str(&field_description);

        if iter.peek().is_some() {
            fields_string.push(',')
        }
    }
    fields_string.push(brackets.1);
    Ok(fields_string)
}

fn field_type_description(
    field: &Field<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let mut type_description = transformer.resolve(field.ty.id)?;

    let is_boxed = field
        .type_name
        .as_ref()
        .map(|e| e.contains("Box<"))
        .unwrap_or_default();
    if is_boxed {
        type_description = format!("Box<{}>", type_description);
    }

    let type_description_maybe_named = if let Some(name) = &field.name {
        format!("{}: {}", name, type_description)
    } else {
        type_description
    };
    Ok(type_description_maybe_named)
}
