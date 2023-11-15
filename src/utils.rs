use std::collections::HashMap;

use scale_info::{form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeParameter};

/// Checks if the two types are equal, but with a bit of a relaxed condition:
/// If there is any type id involved that is not equal:
///     If it is from the same generic parameter for a and b:
///         types still considered equal.
///
/// Should be transitive.
fn types_equal_extended_to_params(a: &Type<PortableForm>, b: &Type<PortableForm>) -> bool {
    let collect_params = |type_params: &[TypeParameter<PortableForm>]| {
        type_params
            .iter()
            .filter_map(|p| p.ty.map(|ty| (ty.id, p.name.clone())))
            .collect::<HashMap<u32, String>>()
    };

    let type_params_a = collect_params(&a.type_params);
    let type_params_b = collect_params(&a.type_params);

    // returns true if the ids are the same or if they point to the same generic parameter.
    let ids_equal = |a: u32, b: u32| -> bool {
        a == b
            || matches!((type_params_a.get(&a), type_params_b.get(&b)), (Some(a_name), Some(b_name)) if a_name == b_name)
    };

    let fields_equal = |a: &[Field<PortableForm>], b: &[Field<PortableForm>]| -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(a, b)| {
            a.name == b.name && a.type_name == b.type_name && ids_equal(a.ty.id, b.ty.id)
        })
    };

    match (&a.type_def, &b.type_def) {
        (TypeDef::Composite(a), TypeDef::Composite(b)) => fields_equal(&a.fields, &b.fields),
        (TypeDef::Variant(a), TypeDef::Variant(b)) => {
            a.variants.len() == b.variants.len()
                && a.variants
                    .iter()
                    .zip(b.variants.iter())
                    .all(|(a, b)| a.name == b.name && fields_equal(&a.fields, &b.fields))
        }
        (TypeDef::Sequence(a), TypeDef::Sequence(b)) => ids_equal(a.type_param.id, b.type_param.id),
        (TypeDef::Array(a), TypeDef::Array(b)) => {
            a.len == b.len && ids_equal(a.type_param.id, b.type_param.id)
        }
        (TypeDef::Tuple(a), TypeDef::Tuple(b)) => {
            a.fields.len() == b.fields.len()
                && a.fields
                    .iter()
                    .zip(b.fields.iter())
                    .all(|(a, b)| ids_equal(a.id, b.id))
        }
        (TypeDef::Primitive(a), TypeDef::Primitive(b)) => a == b,
        (TypeDef::Compact(a), TypeDef::Compact(b)) => ids_equal(a.type_param.id, b.type_param.id),
        (TypeDef::BitSequence(a), scale_info::TypeDef::BitSequence(b)) => {
            ids_equal(a.bit_order_type.id, b.bit_order_type.id)
                && ids_equal(a.bit_store_type.id, b.bit_store_type.id)
        }
        _ => false,
    }
}

pub fn ensure_unique_type_paths(types: &mut PortableRegistry) {
    // Filled with clones of types in type registry, for fast lookup. Drawback: Clone :(
    let mut type_path_lookup = HashMap::<Vec<String>, Type<PortableForm>>::new();

    for ty in &mut types.types {
        // Ignore types without a path (i.e prelude types).
        if ty.ty.path.namespace().is_empty() {
            continue;
        };

        let mut try_counter = 0;
        let name_unincremented = ty.ty.path.segments.last().unwrap().clone();
        loop {
            let Some(entry) = type_path_lookup.get(&ty.ty.path.segments) else {
                // Note: clone here not too great, but unavoidable.
                type_path_lookup.insert(ty.ty.path.segments.clone(), ty.ty.clone());
                break;
            };
            if types_equal_extended_to_params(entry, &ty.ty) {
                // type is basically a 1:1 copy of the type already present.
                break;
            }

            try_counter += 1;

            let name = ty
                .ty
                .path
                .segments
                .last_mut()
                .expect("Types have names; qed;");

            *name = format!("{name_unincremented}{}", try_counter + 1);
        }
    }
}
