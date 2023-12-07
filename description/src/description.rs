use scale_info::{
    form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeDefPrimitive, TypeDefTuple,
    TypeDefVariant, Variant,
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
    fn return_type_name(
        _type_id: u32,
        ty: &Type<PortableForm>,
        transformer: &Transformer<String>,
    ) -> Option<anyhow::Result<String>> {
        if ty.path.ident().is_some() {
            return Some(Ok(type_name_with_type_params(ty, transformer.types())));
        }
        None
    }

    fn return_type_name_on_cache_hit(
        _type_id: u32,
        ty: &Type<PortableForm>,
        cached: &String,
        transformer: &Transformer<String>,
    ) -> Option<anyhow::Result<String>> {
        if ty.path.ident().is_some() {
            return Some(Ok(type_name_with_type_params(ty, transformer.types())));
        }
        Some(Ok(cached.to_owned()))
    }
    let transformer = Transformer::new(
        ty_description,
        return_type_name,
        return_type_name_on_cache_hit,
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
    let name_and_params = if ty.path.ident().is_some() {
        type_name_with_type_params(ty, transformer.types())
    } else {
        String::new()
    };

    let prefix = match &ty.type_def {
        TypeDef::Variant(_) => "enum ",
        TypeDef::Composite(_) => "struct ",
        _ => "",
    };
    let type_def_description = type_def_type_description(&ty.type_def, transformer)?;
    Ok(format!("{prefix}{name_and_params}{type_def_description}"))
}

/// Can be None for types that have an empty path
fn type_name_with_type_params(ty: &Type<PortableForm>, types: &PortableRegistry) -> String {
    match &ty.type_def {
        TypeDef::Sequence(s) => {
            let inner = type_name_with_type_params(types.resolve(s.type_param.id).unwrap(), types);
            return format!("Vec<{inner}>",);
        }
        TypeDef::Array(a) => {
            let inner = type_name_with_type_params(types.resolve(a.type_param.id).unwrap(), types);
            let len = a.len;
            return format!("[{inner};{len}]",);
        }
        TypeDef::Tuple(t) => {
            let mut output = "(".to_string();
            let mut iter = t.fields.iter().peekable();
            while let Some(ty) = iter.next() {
                let type_name = type_name_with_type_params(types.resolve(ty.id).unwrap(), types);
                output.push_str(&type_name);
                if iter.peek().is_some() || t.fields.len() == 1 {
                    output.push(',')
                }
            }
            output.push(')');
            return output;
        }
        TypeDef::Primitive(p) => return primitive_type_description(p).into(),
        TypeDef::Compact(c) => {
            let inner = type_name_with_type_params(types.resolve(c.type_param.id).unwrap(), types);
            return format!("Compact<{inner}>",);
        }
        TypeDef::BitSequence(_) => return "BitSequence".into(),
        TypeDef::Composite(_) => {}
        TypeDef::Variant(_) => {}
    }

    let Some(ident) = ty.path.ident() else {
        return "_".to_string(); // this should happen rarely
    };

    let params = ty
        .type_params
        .iter()
        .map(|e| {
            let Some(ty) = e.ty.as_ref() else {
                return "_".to_string();
            };

            let ty = types.resolve(ty.id).unwrap();
            type_name_with_type_params(ty, types)
        })
        .collect::<Vec<_>>()
        .join(",");

    if params.is_empty() {
        ident.to_string()
    } else {
        format!("{ident}<{}>", params)
    }
}

fn type_def_type_description(
    type_def: &TypeDef<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    match type_def {
        TypeDef::Composite(composite) => fields_type_description(&composite.fields, transformer),
        TypeDef::Variant(variant) => variant_type_def_type_description(variant, transformer),
        TypeDef::Sequence(sequence) => Ok(format!(
            "Vec<{}>",
            transformer.resolve(sequence.type_param.id)?
        )),
        TypeDef::Array(array) => Ok(format!(
            "[{}; {}]",
            transformer.resolve(array.type_param.id)?,
            array.len
        )),
        TypeDef::Tuple(tuple) => tuple_type_description(tuple, transformer),
        TypeDef::Primitive(primitive) => Ok(primitive_type_description(primitive).into()),
        TypeDef::Compact(compact) => Ok(format!(
            "Compact<{}>",
            transformer.resolve(compact.type_param.id)?
        )),
        TypeDef::BitSequence(bit_sequence) => {
            let bit_order_type = transformer.resolve(bit_sequence.bit_order_type.id)?;
            let bit_store_type = transformer.resolve(bit_sequence.bit_store_type.id)?;
            Ok(format!("BitSequence({bit_order_type}, {bit_store_type})"))
        }
    }
}

fn tuple_type_description(
    tuple: &TypeDefTuple<PortableForm>,
    transformer: &Transformer<String>,
) -> anyhow::Result<String> {
    let mut output = "(".to_string();
    let mut iter = tuple.fields.iter().peekable();
    let field_count = tuple.fields.len();
    while let Some(ty) = iter.next() {
        let type_description = transformer.resolve(ty.id)?;
        output.push_str(&type_description);
        if iter.peek().is_some() || field_count == 1 {
            output.push(',')
        }
    }
    output.push(')');
    Ok(output)
}

fn primitive_type_description(primitive: &TypeDefPrimitive) -> &'static str {
    match &primitive {
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
