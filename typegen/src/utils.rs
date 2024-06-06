use self::generics_list::GenericsList;
use scale_info::{form::PortableForm, Field, PortableRegistry, Type, TypeDef};
use std::collections::{HashMap, HashSet};

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
    for (ty_idx, ty) in types.types.iter().enumerate() {
        // We use the index of the type in the types registry instead of `ty.id`. The two
        // _should_ be identical, but prior to `scale-info` 2.11.1  they sometimes weren't
        // when `registry.retain()` was used, and so to avoid older metadata files breaking
        // things, let's stick to using the index for a while:
        let ty_idx = ty_idx as u32;
        let ty = &ty.ty;

        // Ignore types without a path (i.e prelude types).
        if ty.path.namespace().is_empty() {
            continue;
        };

        // get groups that share this path already, if any.
        let groups_with_same_path = types_with_same_type_path_grouped_by_shape
            .entry(&ty.path.segments)
            .or_default();

        // Compare existing groups to check which to add our type ID to.
        let mut added_to_existing_group = false;
        for group in groups_with_same_path.iter_mut() {
            let other_ty_in_group_idx = group[0]; // all types in group are same shape; just check any one of them.
            if types_equal(ty_idx, other_ty_in_group_idx, types) {
                group.push(ty_idx);
                added_to_existing_group = true;
                break;
            }
        }

        // We didn't find a matching group, so add it to a new one.
        if !added_to_existing_group {
            groups_with_same_path.push(vec![ty_idx])
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
                    .expect("type is present (2); qed;");
                let name = ty.ty.path.segments.last_mut().expect("This is only empty for builtin types, that are filtered out with namespace().is_empty() above; qed;");
                *name = format!("{name}{n}"); // e.g. Header1, Header2, Header3, ...
            }
            n += 1;
        }
    }
}

/// This attempts to check whether two types are equal in terms of their shape.
/// In other words: should we de-duplicate these types during codegen.
///
/// The basic algorithm here is:
/// - If type IDs match, they are the same.
/// - If type IDs can be explained by the same generic parameter, they are the same.
/// - If type paths or generic names don't match, they are different.
/// - If the corresponding TypeDefs (shape of type) is different, they are different.
/// - Else, recurse through any contained type IDs and start from the top.
pub(crate) fn types_equal(a: u32, b: u32, types: &PortableRegistry) -> bool {
    let mut a_visited = HashSet::new();
    let mut b_visited = HashSet::new();
    types_equal_inner(
        a,
        &GenericsList::empty(),
        &mut a_visited,
        b,
        &GenericsList::empty(),
        &mut b_visited,
        types,
    )
}

// Panics if the given type ID is not found in the registry.
fn types_equal_inner(
    a: u32,
    a_parent_params: &GenericsList,
    a_visited: &mut HashSet<u32>,
    b: u32,
    b_parent_params: &GenericsList,
    b_visited: &mut HashSet<u32>,
    types: &PortableRegistry,
) -> bool {
    // IDs are the same; types must be identical!
    if a == b {
        return true;
    }

    // Make note of these IDs in case we recurse and see them again.
    let seen_a = !a_visited.insert(a);
    let seen_b = !b_visited.insert(b);

    // One type is recursive and the other isn't; they are different.
    // If neither type is recursive, we keep checking.
    if seen_a != seen_b {
        return false;
    }

    // Both types are recursive, and they look the same based on the above,
    // so assume all is well, since we've already checked other things in prev recursion.
    if seen_a && seen_b {
        return true;
    }

    // Make note of whether these IDs (might) correspond to any specific generic.
    let a_generic_idx = a_parent_params.index_for_type_id(a);
    let b_generic_idx = b_parent_params.index_for_type_id(b);

    let a_ty = types.resolve(a).expect("type a should exist in registry");
    let b_ty = types.resolve(b).expect("type b should exist in registry");

    // Capture a few variables to avoid some repetition later when we recurse.
    let mut types_equal_recurse =
        |a: u32, a_params: &GenericsList, b: u32, b_params: &GenericsList| -> bool {
            types_equal_inner(a, a_params, a_visited, b, b_params, b_visited, types)
        };

    // We'll lazily extend our type params only if the shapes match.
    let calc_params = || {
        let a_params = a_parent_params.extend(&a_ty.type_params);
        let b_params = b_parent_params.extend(&b_ty.type_params);
        (a_params, b_params)
    };

    // If both IDs map to same generic param, then we'll assume equal. If they don't
    // then we need to keep checking other properties (eg Vec<bool> and Vec<u8> will have
    // different type IDs but may be the same type if the bool+u8 line up to generic params).
    if let (Some(a_idx), Some(b_idx)) = (a_generic_idx, b_generic_idx) {
        if a_idx == b_idx {
            return true;
        }
    }

    // Paths differ; types won't be equal then!
    if a_ty.path.segments != b_ty.path.segments {
        return false;
    }

    #[rustfmt::skip]
    let mut compare_fields = |
        a: &Field<PortableForm>,
        a_params: &GenericsList,
        b: &Field<PortableForm>,
        b_params: &GenericsList|
     -> bool {
        if a.name != b.name {
            return false;
        }

        // The type is wrapped in another type such as `Vec<T>` or 
        // marked as skipped with `#[scale_info(skip_type_params(T))]`
        let ty_is_skipped_or_wrapped = a_params
            .index_for_type_id(a.ty.id)
            .zip(b_params.index_for_type_id(b.ty.id))
            .is_none();

        // Check that both type names are present or recurse in case of wrapped types
        a.type_name
            .as_ref()
            .zip(b.type_name.as_ref())
            .filter(|(_, _)| !ty_is_skipped_or_wrapped)
            .map_or_else(
                || types_equal_recurse(a.ty.id, a_params, b.ty.id, b_params),
                |(a_type_name, b_type_name)| {
                    // check that both type names are present in Generic Params and have the same indexes
                    a_params
                        .index_for_type_name(a_type_name.as_ref())
                        .zip(b_params.index_for_type_name(b_type_name))
                        .is_some_and(|(a, b)| a == b)
                },
            )
    };

    // Check that all of the fields of some type are equal.
    #[rustfmt::skip]
    let mut fields_equal = |
        a: &[Field<PortableForm>],
        a_params: &GenericsList,
        b: &[Field<PortableForm>],
        b_params: &GenericsList,
    | -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).all(|(a, b)| {
           compare_fields(a, a_params, b, b_params)
        })
    };

    // Check that the shape of the types and contents are equal.
    match (&a_ty.type_def, &b_ty.type_def) {
        (TypeDef::Composite(a), TypeDef::Composite(b)) => {
            let (a_params, b_params) = calc_params();
            fields_equal(&a.fields, &a_params, &b.fields, &b_params)
        }
        (TypeDef::Variant(a), TypeDef::Variant(b)) => {
            let (a_params, b_params) = calc_params();
            a.variants.len() == b.variants.len()
                && a.variants.iter().zip(b.variants.iter()).all(|(a, b)| {
                    a.name == b.name && fields_equal(&a.fields, &a_params, &b.fields, &b_params)
                })
        }
        (TypeDef::Sequence(a), TypeDef::Sequence(b)) => {
            let (a_params, b_params) = calc_params();
            types_equal_recurse(a.type_param.id, &a_params, b.type_param.id, &b_params)
        }
        (TypeDef::Array(a), TypeDef::Array(b)) => {
            let (a_params, b_params) = calc_params();
            a.len == b.len
                && types_equal_recurse(a.type_param.id, &a_params, b.type_param.id, &b_params)
        }
        (TypeDef::Tuple(a), TypeDef::Tuple(b)) => {
            let (a_params, b_params) = calc_params();
            a.fields.len() == b.fields.len()
                && a.fields
                    .iter()
                    .zip(b.fields.iter())
                    .all(|(a, b)| types_equal_recurse(a.id, &a_params, b.id, &b_params))
        }
        (TypeDef::Primitive(a), TypeDef::Primitive(b)) => a == b,
        (TypeDef::Compact(a), TypeDef::Compact(b)) => {
            let (a_params, b_params) = calc_params();
            types_equal_recurse(a.type_param.id, &a_params, b.type_param.id, &b_params)
        }
        (TypeDef::BitSequence(a), scale_info::TypeDef::BitSequence(b)) => {
            let (a_params, b_params) = calc_params();
            let order_equal = types_equal_recurse(
                a.bit_order_type.id,
                &a_params,
                b.bit_order_type.id,
                &b_params,
            );
            let store_equal = types_equal_recurse(
                a.bit_store_type.id,
                &a_params,
                b.bit_store_type.id,
                &b_params,
            );
            order_equal && store_equal
        }
        // Type defs don't match; types aren't the same!
        _ => false,
    }
}

/// Just a small helper for the [`types_equal_inner`] function, to track where generic params
/// are in order to see whether different type IDs may actually be represented by the same generics.
mod generics_list {
    use scale_info::{form::PortableForm, TypeParameter};
    use std::rc::Rc;

    /// A list of generics by type ID. For a given type ID, we'll either
    /// return the index of the first generic param we find that matches it,
    /// or None. We can extend this list with more generics as we go.
    #[derive(Clone, Debug)]
    pub struct GenericsList {
        inner: Rc<GenericsListInner>,
    }

    #[derive(Clone, Debug)]
    struct GenericsListInner {
        previous: Option<GenericsList>,
        start_idx: usize,
        generics_by_id: Vec<(u32, String)>,
    }

    impl GenericsList {
        /// Return the unique index of a generic in the list, or None if not found
        pub fn index_for_type_id(&self, type_id: u32) -> Option<usize> {
            let maybe_index = self
                .inner
                .generics_by_id
                .iter()
                .position(|(id, _)| *id == type_id)
                .map(|index| self.inner.start_idx + index);

            // if index isn't found here, go back to the previous list and try again.
            maybe_index.or_else(|| {
                self.inner
                    .previous
                    .as_ref()
                    .and_then(|prev| prev.index_for_type_id(type_id))
            })
        }
        /// Returns the unique index of a generic type name, or None if not found.
        pub fn index_for_type_name(&self, name: &str) -> Option<usize> {
            let maybe_index = self
                .inner
                .generics_by_id
                .iter()
                .position(|(_, type_name)| *type_name == name)
                .map(|index| self.inner.start_idx + index);

            // if index isn't found here, go back to the previous list and try again.
            maybe_index.or_else(|| {
                self.inner
                    .previous
                    .as_ref()
                    .and_then(|prev| prev.index_for_type_name(name))
            })
        }

        /// Create an empty list.
        pub fn empty() -> Self {
            Self::new_inner(None, &[])
        }

        /// Extend this list with more params.
        pub fn extend(&self, params: &[TypeParameter<PortableForm>]) -> Self {
            Self::new_inner(Some(self.clone()), params)
        }

        fn new_inner(
            maybe_self: Option<GenericsList>,
            params: &[TypeParameter<PortableForm>],
        ) -> Self {
            let generics_by_id = params
                .iter()
                .filter_map(|p| p.ty.map(|ty| (ty.id, p.name.clone())))
                .collect();

            let start_idx = match &maybe_self {
                Some(list) => list.inner.start_idx + list.inner.generics_by_id.len(),
                None => 0,
            };

            GenericsList {
                inner: Rc::new(GenericsListInner {
                    previous: maybe_self,
                    start_idx,
                    generics_by_id,
                }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::typegen::ir::ToTokensWithSettings;
    use pretty_assertions::assert_eq;

    use super::*;
    use quote::quote;
    use scale_info::{
        meta_type, Field, Path, PortableRegistry, TypeDef, TypeDefComposite, TypeInfo,
        TypeParameter,
    };

    #[test]
    fn associated_types_are_properly_deduplicated() {
        trait X {
            type Assoc;
        }
        struct A;
        impl X for A {
            type Assoc = u8;
        }
        struct B;
        impl X for B {
            type Assoc = u32;
        }

        #[derive(TypeInfo)]
        #[scale_info(skip_type_params(T))] // this is optional
        struct Foo<T: X> {
            _field: T::Assoc,
        }

        #[allow(unused)]
        #[derive(TypeInfo)]
        struct Bar {
            p: Foo<A>,
            q: Foo<B>,
        }

        let mut registry = scale_info::Registry::new();
        let _ = registry.register_type(&scale_info::meta_type::<Bar>()).id;
        let mut registry = scale_info::PortableRegistry::from(registry);

        ensure_unique_type_paths(&mut registry);

        let settings = crate::TypeGeneratorSettings::new();
        let generated = crate::typegen::ir::ToTokensWithSettings::to_token_stream(
            &crate::TypeGenerator::new(&registry, &settings)
                .generate_types_mod()
                .unwrap(),
            &settings,
        );

        let expected = quote!(
            pub mod types {
                use super::types;
                pub mod scale_typegen {
                    use super::types;
                    pub mod utils {
                        use super::types;
                        pub mod tests {
                            use super::types;
                            pub struct Bar {
                                pub p: types::scale_typegen::utils::tests::Foo1,
                                pub q: types::scale_typegen::utils::tests::Foo2,
                            }
                            pub struct Foo1 {
                                pub _field: ::core::primitive::u8,
                            }
                            pub struct Foo2 {
                                pub _field: ::core::primitive::u32,
                            }
                        }
                    }
                }
            }
        );

        assert_eq!(expected.to_string(), generated.to_string());
    }

    #[test]
    fn generics_unification() {
        macro_rules! nested_type {
            ($ty:ident, $generic:ty, $inner:ty) => {
                struct $ty;
                impl scale_info::TypeInfo for $ty {
                    type Identity = Self;
                    fn type_info() -> scale_info::Type {
                        scale_info::Type {
                            path: Path::new("ParamType", "my::module"),
                            type_params: vec![TypeParameter::new(
                                "T",
                                Some(meta_type::<$generic>()),
                            )],
                            type_def: TypeDef::Composite(TypeDefComposite::new([Field::new(
                                None,
                                meta_type::<$inner>(),
                                Some("T"),
                                Vec::new(),
                            )])),
                            docs: vec![],
                        }
                    }
                }
            };
        }

        struct A;
        impl scale_info::TypeInfo for A {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("NestedType", "my::module"),
                    type_params: vec![
                        TypeParameter::new("T", Some(meta_type::<u8>())),
                        TypeParameter::new("U", Some(meta_type::<u16>())),
                        TypeParameter::new("V", Some(meta_type::<u32>())),
                    ],
                    type_def: TypeDef::Composite(TypeDefComposite::new([
                        Field::new(None, meta_type::<u8>(), Some("T"), Vec::new()),
                        Field::new(None, meta_type::<u16>(), Some("U"), Vec::new()),
                        Field::new(None, meta_type::<u32>(), Some("V"), Vec::new()),
                    ])),
                    docs: vec![],
                }
            }
        }

        struct B;
        impl scale_info::TypeInfo for B {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("NestedType", "my::module"),
                    type_params: vec![
                        TypeParameter::new("T", Some(meta_type::<u8>())),
                        TypeParameter::new("U", Some(meta_type::<u16>())),
                        TypeParameter::new("V", Some(meta_type::<u32>())),
                    ],
                    type_def: TypeDef::Composite(TypeDefComposite::new([
                        Field::new(None, meta_type::<u32>(), Some("V"), Vec::new()),
                        Field::new(None, meta_type::<u16>(), Some("U"), Vec::new()),
                        Field::new(None, meta_type::<u8>(), Some("T"), Vec::new()),
                    ])),
                    docs: vec![],
                }
            }
        }
        struct BB;
        impl scale_info::TypeInfo for BB {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("NestedType", "my::module"),
                    type_params: vec![
                        TypeParameter::new("V", Some(meta_type::<u8>())),
                        TypeParameter::new("U", Some(meta_type::<u16>())),
                        TypeParameter::new("T", Some(meta_type::<u32>())),
                    ],
                    type_def: TypeDef::Composite(TypeDefComposite::new([
                        Field::new(None, meta_type::<u32>(), Some("T"), Vec::new()),
                        Field::new(None, meta_type::<u16>(), Some("U"), Vec::new()),
                        Field::new(None, meta_type::<u8>(), Some("V"), Vec::new()),
                    ])),
                    docs: vec![],
                }
            }
        }

        struct C;
        impl scale_info::TypeInfo for C {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("NestedType", "my::module"),
                    type_params: vec![
                        TypeParameter::new("A", Some(meta_type::<u8>())),
                        TypeParameter::new("D", Some(meta_type::<u16>())),
                        TypeParameter::new("B", Some(meta_type::<u32>())),
                    ],
                    type_def: TypeDef::Composite(TypeDefComposite::new([
                        Field::new(None, meta_type::<u8>(), Some("A"), Vec::new()),
                        Field::new(None, meta_type::<u16>(), Some("D"), Vec::new()),
                        Field::new(None, meta_type::<u32>(), Some("B"), Vec::new()),
                    ])),
                    docs: vec![],
                }
            }
        }

        struct D;
        impl scale_info::TypeInfo for D {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("Foo", "my::module"),
                    type_params: vec![TypeParameter::new("A", Some(meta_type::<u8>()))],
                    type_def: TypeDef::Composite(TypeDefComposite::new([Field::new(
                        None,
                        meta_type::<u8>(),
                        Some("A"),
                        Vec::new(),
                    )])),
                    docs: vec![],
                }
            }
        }
        struct E;
        impl scale_info::TypeInfo for E {
            type Identity = Self;
            fn type_info() -> scale_info::Type {
                scale_info::Type {
                    path: Path::new("Foo", "my::module"),
                    type_params: vec![TypeParameter::new("B", Some(meta_type::<u16>()))],
                    type_def: TypeDef::Composite(TypeDefComposite::new([Field::new(
                        None,
                        meta_type::<u16>(),
                        Some("B"),
                        Vec::new(),
                    )])),
                    docs: vec![],
                }
            }
        }

        let mut registry = scale_info::Registry::new();
        let id_b = registry.register_type(&meta_type::<B>()).id;
        let id_bb = registry.register_type(&meta_type::<BB>()).id;
        let id_a = registry.register_type(&meta_type::<A>()).id;
        let id_c = registry.register_type(&meta_type::<C>()).id;

        let id_d = registry.register_type(&meta_type::<D>()).id;
        let id_e = registry.register_type(&meta_type::<E>()).id;

        nested_type!(Y, A, A);
        nested_type!(W, B, B);
        nested_type!(Z, C, C);

        let id_y = registry.register_type(&meta_type::<Y>()).id;
        let id_w = registry.register_type(&meta_type::<W>()).id;
        let id_z = registry.register_type(&meta_type::<Z>()).id;

        let mut registry = PortableRegistry::from(registry);

        // A != B, different field ordering
        assert!(!types_equal(id_a, id_b, &registry));
        assert!(!types_equal(id_a, id_bb, &registry));

        // A == C, different generic param names
        assert!(types_equal(id_a, id_c, &registry));

        // D == E, different generic param names
        assert!(types_equal(id_d, id_e, &registry));

        assert!(types_equal(id_w, id_y, &registry));
        assert!(types_equal(id_z, id_y, &registry));

        ensure_unique_type_paths(&mut registry);
        let settings = crate::TypeGeneratorSettings::new();
        let output = crate::TypeGenerator::new(&registry, &settings)
            .generate_types_mod()
            .unwrap()
            .to_token_stream(&settings);

        let expected = quote! {
                pub mod types {
                use super::types;
                pub mod my {
                    use super::types;
                    pub mod module {
                        use super::types;
                        pub struct Foo<_0>(pub _0, );
                        pub struct NestedType1<_0, _1, _2>(pub _2, pub _1, pub _0, );
                        pub struct NestedType2<_0, _1, _2>(pub _0, pub _1, pub _2, );
                        pub struct ParamType < _0 > (pub _0 ,) ;
                    }
                }
            }
        };

        assert_eq!(expected.to_string(), output.to_string())
    }

    #[test]
    fn recursive_data() {
        #[derive(TypeInfo)]
        #[allow(dead_code)]
        pub enum Test<A> {
            None,
            Many { inner: Vec<Self> },
            Param(A),
        }

        #[derive(TypeInfo)]
        #[allow(dead_code)]
        pub struct TestStruct<A> {
            param: A,
            inner: Vec<Self>,
        }

        #[derive(TypeInfo)]
        #[allow(dead_code)]
        pub struct Foo<T> {
            inner: Vec<T>,
        }

        #[derive(TypeInfo)]
        #[allow(dead_code)]
        pub enum FooEnum<T> {
            None,
            Go(Vec<T>),
        }

        let mut registry = scale_info::Registry::new();
        let id_a = registry.register_type(&meta_type::<Test<u8>>()).id;
        let id_b = registry.register_type(&meta_type::<Test<u32>>()).id;

        let id_foo = registry.register_type(&meta_type::<Foo<u32>>()).id;

        let id_foo_foo = registry
            .register_type(&meta_type::<Foo<Foo<TestStruct<u32>>>>())
            .id;

        let id_foo_enum = registry.register_type(&meta_type::<FooEnum<u32>>()).id;
        let id_foo_foo_enum = registry
            .register_type(&meta_type::<FooEnum<FooEnum<TestStruct<u32>>>>())
            .id;
        let id_a_struct = registry.register_type(&meta_type::<TestStruct<u32>>()).id;
        let id_b_struct = registry
            .register_type(&meta_type::<TestStruct<TestStruct<u64>>>())
            .id;

        let registry = PortableRegistry::from(registry);

        assert!(types_equal(id_foo, id_foo_foo, &registry));
        assert!(types_equal(id_foo_enum, id_foo_foo_enum, &registry));

        assert!(types_equal(id_a, id_b, &registry));
        assert!(types_equal(id_a_struct, id_b_struct, &registry));

        let settings = crate::TypeGeneratorSettings::new();
        let output = crate::TypeGenerator::new(&registry, &settings)
            .generate_types_mod()
            .unwrap()
            .to_token_stream(&settings);
        // Why codegen for Foo expands this way?
        let expected = quote! {
            pub mod types {
                use super::types;
                pub mod scale_typegen {
                    use super::types;
                    pub mod utils {
                        use super::types;
                        pub mod tests {
                            use super::types;
                            pub struct Foo < _0 > {
                                pub inner : :: std :: vec :: Vec < _0 > ,
                            }
                            pub enum FooEnum < _0 > {
                                None ,
                                Go (:: std :: vec :: Vec < _0 > ,) ,
                            }
                            pub enum Test<_0> {
                                None,
                                Many {
                                    inner: ::std::vec::Vec<
                                        types::scale_typegen::utils::tests::Test<_0>
                                    >,
                                },
                                Param(_0, ),
                            }
                            pub struct TestStruct<_0> {
                                pub param: _0,
                                pub inner: ::std::vec::Vec<
                                    types::scale_typegen::utils::tests::TestStruct<_0>
                                >,
                            }
                        }
                    }
                }
            }
        };

        assert_eq!(expected.to_string(), output.to_string())
    }
    #[test]
    fn ensure_unique_type_paths_test() {
        macro_rules! foo {
            ($ty:ident, $prim:ident ) => {
                struct $ty;
                impl scale_info::TypeInfo for $ty {
                    type Identity = Self;
                    fn type_info() -> scale_info::Type {
                        scale_info::Type {
                            path: Path::new("Foo", "my::module"),
                            type_params: vec![],
                            type_def: scale_info::TypeDef::Primitive(
                                scale_info::TypeDefPrimitive::$prim,
                            ),
                            docs: vec![],
                        }
                    }
                }
            };
        }

        foo!(Foo1, Bool);
        foo!(Foo2, Bool);
        foo!(Foo3, U32);
        foo!(Foo4, U128);
        foo!(Foo5, U128);
        foo!(Foo6, U128);

        let mut registry = scale_info::Registry::new();
        let id_1 = registry.register_type(&meta_type::<Foo1>()).id;
        let id_2 = registry.register_type(&meta_type::<Foo2>()).id;
        let id_3 = registry.register_type(&meta_type::<Foo3>()).id;
        let id_4 = registry.register_type(&meta_type::<Foo4>()).id;
        let id_5 = registry.register_type(&meta_type::<Foo5>()).id;
        let id_6 = registry.register_type(&meta_type::<Foo6>()).id;
        let mut registry = PortableRegistry::from(registry);

        // before:
        let ident = |id: u32| registry.resolve(id).unwrap().path.ident().unwrap();
        assert_eq!(ident(id_1), "Foo");
        assert_eq!(ident(id_2), "Foo");
        assert_eq!(ident(id_3), "Foo");
        assert_eq!(ident(id_4), "Foo");
        assert_eq!(ident(id_5), "Foo");
        assert_eq!(ident(id_6), "Foo");

        // after:
        ensure_unique_type_paths(&mut registry);

        let ident = |id: u32| registry.resolve(id).unwrap().path.ident().unwrap();
        assert_eq!(ident(id_1), "Foo1");
        assert_eq!(ident(id_2), "Foo1");
        assert_eq!(ident(id_3), "Foo2");
        assert_eq!(ident(id_4), "Foo3");
        assert_eq!(ident(id_5), "Foo3");
        assert_eq!(ident(id_6), "Foo3");
    }

    #[test]
    fn types_equal_recursing_test() {
        #[derive(TypeInfo)]
        struct Foo<T> {
            _inner: T,
        }

        macro_rules! nested_type {
            ($ty:ident, $generic:ty, $inner:ty) => {
                struct $ty;
                impl scale_info::TypeInfo for $ty {
                    type Identity = Self;
                    fn type_info() -> scale_info::Type {
                        scale_info::Type {
                            path: Path::new("NestedType", "my::module"),
                            type_params: vec![TypeParameter::new(
                                "T",
                                Some(meta_type::<$generic>()),
                            )],
                            type_def: TypeDef::Composite(TypeDefComposite::new([Field::new(
                                None,
                                meta_type::<$inner>(),
                                None,
                                Vec::new(),
                            )])),
                            docs: vec![],
                        }
                    }
                }
            };
        }
        macro_rules! nested_typeB {
            ($ty:ident, $generic:ty, $inner:ty) => {
                struct $ty;
                impl scale_info::TypeInfo for $ty {
                    type Identity = Self;
                    fn type_info() -> scale_info::Type {
                        scale_info::Type {
                            path: Path::new("NestedType", "my::module"),
                            type_params: vec![TypeParameter::new(
                                "B",
                                Some(meta_type::<$generic>()),
                            )],
                            type_def: TypeDef::Composite(TypeDefComposite::new([Field::new(
                                None,
                                meta_type::<$inner>(),
                                None,
                                Vec::new(),
                            )])),
                            docs: vec![],
                        }
                    }
                }
            };
        }

        // A and B are the same because generics explain the param difference.
        //
        //NestedType<T = u32>(u32)
        //NestedType<T = bool>(bool)
        nested_type!(A, u32, u32);
        nested_type!(B, bool, bool);
        nested_typeB!(G, u64, u64);

        // As above, but another layer of nesting before generic param used.
        //
        //NestedType<T = u32>(Vec<u32>)
        //NestedType<T = bool>(Vec<bool>)
        nested_type!(C, bool, Vec<bool>);
        nested_type!(D, u32, Vec<u32>);

        // A third layer of nesting just to really check the recursion.
        //
        //NestedType<T = u32>(Vec<Foo<u32>>)
        //NestedType<T = bool>(Vec<Foo<bool>>)
        nested_type!(E, bool, Vec<Foo<bool>>);
        nested_type!(F, u32, Vec<Foo<u32>>);

        let mut registry = scale_info::Registry::new();
        let id_a = registry.register_type(&meta_type::<A>()).id;
        let id_b = registry.register_type(&meta_type::<B>()).id;
        let id_c = registry.register_type(&meta_type::<C>()).id;
        let id_d = registry.register_type(&meta_type::<D>()).id;
        let id_e = registry.register_type(&meta_type::<E>()).id;
        let id_f = registry.register_type(&meta_type::<F>()).id;
        let id_g = registry.register_type(&meta_type::<G>()).id;

        let mut registry = PortableRegistry::from(registry);

        // Despite how many layers of nesting, we identify that the generic
        // param can explain the difference, so can see them as being equal.
        assert!(types_equal(id_a, id_b, &registry));
        assert!(types_equal(id_c, id_d, &registry));
        assert!(types_equal(id_e, id_f, &registry));
        assert!(types_equal(id_a, id_g, &registry));

        // Sanity check that the pairs are not equal with each other.
        assert!(!types_equal(id_a, id_c, &registry));
        assert!(!types_equal(id_a, id_e, &registry));
        assert!(!types_equal(id_c, id_e, &registry));

        // Now, check that the generated output is sane and in line with this...

        ensure_unique_type_paths(&mut registry);
        let settings = crate::TypeGeneratorSettings::new();
        let output = crate::TypeGenerator::new(&registry, &settings)
            .generate_types_mod()
            .unwrap()
            .to_token_stream(&settings);

        // This isn't ideal, but I printed out the token stream, and it looks good (ie generates
        // 3 types after deduplicating with the correct generic param usage), so this test will
        // check that the output still looks good. To update, copy and `rustfmt` the new output
        // and then adjust the odd thing until it matches again.
        let expected = quote! {
            pub mod types {
                use super::types;
                pub mod my {
                    use super::types;
                    pub mod module {
                        use super::types;
                        pub struct NestedType1<_0>(pub _0,);
                        pub struct NestedType2<_0>(pub ::std::vec::Vec<_0>,);
                        pub struct NestedType3<_0>(
                            pub ::std::vec::Vec<types::scale_typegen::utils::tests::Foo<_0> >,
                        );
                    }
                }
                pub mod scale_typegen {
                    use super::types;
                    pub mod utils {
                        use super::types;
                        pub mod tests {
                            use super::types;
                            pub struct Foo<_0> {
                                pub _inner: _0,
                            }
                        }
                    }
                }
            }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
