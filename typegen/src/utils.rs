use scale_info::{form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeParameter};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

use crate::TypegenError;

/// Converts a [`scale_info::Type`] into a [`syn::TypePath`].
pub fn syn_type_path(ty: &Type<PortableForm>) -> Result<syn::TypePath, TypegenError> {
    let joined_path = ty.path.segments.join("::");
    let ty_path: syn::TypePath = syn::parse_str(&joined_path)?;
    Ok(ty_path)
}

/// Deduplicates type paths in the provided Registry.
pub fn ensure_unique_type_paths(types: &mut PortableRegistry) {
    let mut types_with_same_type_path = HashMap::<&[String], SmallVec<[u32; 2]>>::new();

    // Figure out which types have the same type paths by storing them in the HashMap
    for ty in types.types.iter() {
        // Ignore types without a path (i.e prelude types).
        if ty.ty.path.namespace().is_empty() {
            continue;
        };
        types_with_same_type_path
            .entry(&ty.ty.path.segments)
            .or_default()
            .push(ty.id);
    }

    let clashing_type_ids = types_with_same_type_path
        .into_iter()
        .filter_map(|(_, v)| (v.len() > 1).then_some(v));

    // submit to this buffer type ids (u32), where a number (usize) should be added to the type path to distinguish them.
    let mut renaming_commands: Vec<(u32, usize)> = vec![];

    for type_ids in clashing_type_ids {
        // Map N types in type_ids to M new type paths, where M <= N.
        // types with the same structure should map to the same type path after all.
        let mut types_with_same_structure: Vec<(&Type<PortableForm>, SmallVec<[u32; 2]>)> = vec![];
        'outer: for id in type_ids {
            let type_a = types.resolve(id).expect("type is present; qed;");
            for (type_b, ids) in types_with_same_structure.iter_mut() {
                if types_equal_extended_to_params(type_a, type_b) {
                    ids.push(id);
                    continue 'outer;
                }
            }
            types_with_same_structure.push((type_a, smallvec![id]));
        }

        // Now that the types that share a structure are grouped together, we can order commands to rename them.
        for (n, (_, group)) in types_with_same_structure.iter().enumerate() {
            for id in group {
                renaming_commands.push((*id, n + 1));
            }
        }
    }

    // execute the actual renaming. The `get_mut()` with the usize cast is a bit awkward, but there is currently not `resolve_mut()` function on the `PortableRegistry`.
    for (id, appendix) in renaming_commands {
        let ty = types
            .types
            .get_mut(id as usize)
            .expect("type is present; qed;");
        assert_eq!(ty.id, id);
        let name = ty.ty.path.segments.last_mut().expect("This is only empty for builtin types, that are filtered out with namespace().is_empty() above; qed;");
        *name = format!("{name}{appendix}"); // e.g. Header1, Header2, Header3, ...
    }
}

/// This function checks if two types that have the same type path,
/// should be considered as equal in terms of their structure and
/// their use of generics.
/// This is checked, because if they are indeed equal it is fine
/// to generate a single rust type for the two registered typed.
/// However if they are not equal, we need to deduplicate the type path.
/// This means modifying the type path of one or both clashing types.
///
/// So what types are considered equal?
/// Types are equal, if their TypeDef is exactly the same.
/// Types are also considered equal if the TypeDef has the same shape and
/// all type ids mentioned in the TypeDef are either:
/// - equal
/// - or different, but map essentially to the same generic type parameter
fn types_equal_extended_to_params(a: &Type<PortableForm>, b: &Type<PortableForm>) -> bool {
    let collect_params = |type_params: &[TypeParameter<PortableForm>]| {
        type_params
            .iter()
            .filter_map(|p| p.ty.map(|ty| (ty.id, p.name.clone())))
            .collect::<HashMap<u32, String>>()
    };

    let type_params_a = collect_params(&a.type_params);
    let type_params_b = collect_params(&a.type_params);

    // returns true if the ids are the same OR if they point to the same generic parameter.
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
