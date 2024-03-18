use scale_info::{form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeParameter};
use smallvec::{smallvec, SmallVec};
use std::collections::{hash_map::Entry, HashMap};

use crate::TypegenError;

/// Converts a [`scale_info::Type`] into a [`syn::TypePath`].
pub fn syn_type_path(ty: &Type<PortableForm>) -> Result<syn::TypePath, TypegenError> {
    let joined_path = ty.path.segments.join("::");
    let ty_path: syn::TypePath = syn::parse_str(&joined_path)?;
    Ok(ty_path)
}

/// Deduplicates type paths in the provided Registry.
pub fn ensure_unique_type_paths(types: &mut PortableRegistry) {
    let mut types_with_same_type_path_grouped_by_shape = HashMap::<&[String], Vec<Vec<u32>>>::new();

    // First, group types if they are similar (same path, same shape).

    for (ty_id, ty) in types.types.iter().enumerate() {
        // Ignore types without a path (i.e prelude types).
        if ty.ty.path.namespace().is_empty() {
            continue;
        };

        // get groups that share this path already, if any.
        let groups_with_same_path = types_with_same_type_path_grouped_by_shape
            .entry(&ty.ty.path.segments)
            .or_default();

        // Compare existing groups to check which to add our type ID to.
        let mut added_to_existing_group = false;
        for group in groups_with_same_path.iter_mut() {
            let ty_id_b = group[0]; // all types in group are same shape; just check any one of them.
            let ty_b = types.resolve(ty_id_b).expect("ty exists");
            if types_equal_extended_to_params(&ty.ty, ty_b) {
                group.push(ty_id_b);
                added_to_existing_group = true;
                break;
            }
        }

        // We didn't find a matching group, so add it to a new one.
        if !added_to_existing_group {
            groups_with_same_path.push(vec![ty_id as u32])
        }
    }

    // Now, rename types as needed based on these groups.
    let groups_that_need_renaming = types_with_same_type_path_grouped_by_shape
        .into_values()
        .filter(|g| g.len() > 1)
        .collect::<Vec<_>>(); // Collect necessary because otherwise types is borrowed immutably and cannot be modified.

    for groups_with_same_path in groups_that_need_renaming {
        let mut n = 1;
        for group_with_same_shape in groups_with_same_path {
            for ty_id in group_with_same_shape {
                let ty = types
                    .types
                    .get_mut(ty_id as usize)
                    .expect("type is present; qed;");
                let name = ty.ty.path.segments.last_mut().expect("This is only empty for builtin types, that are filtered out with namespace().is_empty() above; qed;");
                *name = format!("{name}{n}"); // e.g. Header1, Header2, Header3, ...
            }
            n += 1;
        }
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
pub(crate) fn types_equal_extended_to_params(
    a: &Type<PortableForm>,
    b: &Type<PortableForm>,
) -> bool {
    // We map each type ID to all type params if could refer to. Type IDs can refer to multiple parameters:
    // E.g. Foo<A,B> can be parameterized as Foo<u8,u8>, so if 42 is the type id of u8, a filed with id=42 could refer to eiher A or B.
    fn collect_params(
        type_params: &[TypeParameter<PortableForm>],
    ) -> HashMap<u32, SmallVec<[&str; 2]>> {
        let mut hm: HashMap<u32, SmallVec<[&str; 2]>> = HashMap::new();
        for p in type_params {
            if let Some(ty) = &p.ty {
                match hm.entry(ty.id) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(p.name.as_str());
                    }
                    Entry::Vacant(e) => {
                        e.insert(smallvec![p.name.as_str()]);
                    }
                }
            }
        }
        hm
    }

    let type_params_a = collect_params(&a.type_params);
    let type_params_b = collect_params(&b.type_params);

    if a.type_params.len() != b.type_params.len() {
        return false;
    }
    // Returns true if the ids are the same OR if they point to the same generic parameter.
    let ids_equal = |a: u32, b: u32| -> bool {
        if a == b {
            return true;
        }
        let Some(a_param_names) = type_params_a.get(&a) else {
            return false;
        };
        let Some(b_param_names) = type_params_b.get(&b) else {
            return false;
        };
        // Check if there is any intersection, meaning that both IDs map to the same generic type param:
        a_param_names.iter().any(|a_p| b_param_names.contains(a_p))
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
