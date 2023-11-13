use anyhow::anyhow;
use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};
use syn::Ident;

use super::{
    settings::substitutes::TypeSubstitutes,
    type_path::{TypeParameter, TypePath, TypePathType},
};

pub struct TypePathResolver<'a> {
    registry: &'a PortableRegistry,
    substitutes: &'a TypeSubstitutes,
    decoded_bits_type_path: Option<&'a syn::Path>,
    compact_type_path: Option<&'a syn::Path>,
    root_mod_ident: &'a Ident,
}

impl<'a> TypePathResolver<'a> {
    pub fn new(
        registry: &'a PortableRegistry,
        substitutes: &'a TypeSubstitutes,
        decoded_bits_type_path: Option<&'a syn::Path>,
        compact_type_path: Option<&'a syn::Path>,
        root_mod_ident: &'a Ident,
    ) -> Self {
        Self {
            registry,
            substitutes,
            decoded_bits_type_path,
            root_mod_ident,
            compact_type_path,
        }
    }

    /// Get the type path for a field of a struct or an enum variant, providing any generic
    /// type parameters from the containing type. This is for identifying where a generic type
    /// parameter is used in a field type e.g.
    ///
    /// ```rust
    /// struct S<T> {
    ///     a: T, // `T` is the "parent" type param from the containing type.
    ///     b: Vec<Option<T>>, // nested use of generic type param `T`.
    /// }
    /// ```
    ///
    /// This allows generating the correct generic field type paths.
    ///
    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_field_type_path(
        &self,
        id: u32,
        parent_type_params: &[TypeParameter],
        original_name: Option<&str>,
    ) -> anyhow::Result<TypePath> {
        self.resolve_type_path_recurse(id, true, parent_type_params, original_name)
    }

    /// Get the type path for the given type identifier.
    ///
    /// # Panics
    ///
    /// If no type with the given id found in the type registry.
    pub fn resolve_type_path(&self, id: u32) -> anyhow::Result<TypePath> {
        self.resolve_type_path_recurse(id, false, &[], None)
    }

    /// Visit each node in a possibly nested type definition to produce a type path.
    ///
    /// e.g `Result<GenericStruct<NestedGenericStruct<T>>, String>`
    ///
    /// if `original_name` is `Some(original_name)`, the resolved type needs to have the same `original_name`.
    fn resolve_type_path_recurse(
        &self,
        id: u32,
        is_field: bool,
        parent_type_params: &[TypeParameter],
        original_name: Option<&str>,
    ) -> anyhow::Result<TypePath> {
        if let Some(parent_type_param) = parent_type_params.iter().find(|tp| {
            tp.concrete_type_id == id
                && original_name.map_or(true, |original_name| tp.original_name == original_name)
        }) {
            let type_path = TypePath::from_parameter(parent_type_param.clone());
            return Ok(type_path);
        }

        let mut ty = self.resolve_type(id)?;

        if ty.path.ident() == Some("Cow".to_string()) {
            let inner_ty_id = ty.type_params[0]
                .ty
                .ok_or_else(|| anyhow!("type parameters to Cow are not expected to be skipped"))?
                .id;
            ty = self.resolve_type(inner_ty_id)?
        }

        let params: Vec<TypePath> = ty
            .type_params
            .iter()
            .filter_map(|f| {
                f.ty.map(|f| self.resolve_type_path_recurse(f.id, false, parent_type_params, None))
            })
            .collect::<anyhow::Result<Vec<TypePath>>>()?;

        let ty = match &ty.type_def {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {
                self.type_path_maybe_with_substitutes(&ty.path, &params)
            }
            TypeDef::Primitive(primitive) => TypePathType::Primitive {
                def: primitive.clone(),
            },
            TypeDef::Array(arr) => {
                let inner_type = self.resolve_type_path_recurse(
                    arr.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )?;
                TypePathType::Array {
                    len: arr.len as usize,
                    of: Box::new(inner_type),
                }
            }
            TypeDef::Sequence(seq) => {
                let inner_type = self.resolve_type_path_recurse(
                    seq.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )?;
                TypePathType::Vec {
                    of: Box::new(inner_type),
                }
            }
            TypeDef::Tuple(tuple) => {
                let elements = tuple
                    .fields
                    .iter()
                    .map(|f| self.resolve_type_path_recurse(f.id, false, parent_type_params, None))
                    .collect::<anyhow::Result<Vec<TypePath>>>()?;

                TypePathType::Tuple { elements }
            }
            TypeDef::Compact(compact) => {
                let inner_type = self.resolve_type_path_recurse(
                    compact.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )?;

                let compact_type_path = self
                    .compact_type_path
                    .ok_or_else(|| {
                        anyhow!(
                            "compact_type_path is None, cannot generate code for compact types."
                        )
                    })?
                    .clone();

                TypePathType::Compact {
                    inner: Box::new(inner_type),
                    is_field,
                    compact_type_path,
                }
            }
            TypeDef::BitSequence(bitseq) => {
                let decoded_bits_type_path = self.decoded_bits_type_path
                    .ok_or_else(|| anyhow!("decoded_bits_type_path is None, cannot generate code with bit sequences."))?
                    .clone();

                let bit_order_type = self.resolve_type_path_recurse(
                    bitseq.bit_order_type.id,
                    false,
                    parent_type_params,
                    None,
                )?;

                let bit_store_type = self.resolve_type_path_recurse(
                    bitseq.bit_store_type.id,
                    false,
                    parent_type_params,
                    None,
                )?;

                TypePathType::BitVec {
                    bit_order_type: Box::new(bit_order_type),
                    bit_store_type: Box::new(bit_store_type),
                    decoded_bits_type_path,
                }
            }
        };
        Ok(TypePath::from_type(ty))
    }

    pub fn type_path_maybe_with_substitutes(
        &self,
        path: &scale_info::Path<PortableForm>,
        params: &Vec<TypePath>,
    ) -> TypePathType {
        if let Some(substitute) = self.substitutes.for_path_with_params(path, params) {
            substitute
        } else {
            TypePathType::from_type_def_path(path, self.root_mod_ident.clone(), params.clone())
        }
    }

    pub fn resolve_type(&self, id: u32) -> anyhow::Result<Type<PortableForm>> {
        let ty = self
            .registry
            .resolve(id)
            .ok_or_else(|| anyhow!("No type with id {id} found"))?;
        Ok(ty.clone())
    }
}
