use std::collections::btree_map::Entry;

use crate::{
    utils::{sanity_pass, types_equal},
    TypegenError,
};

use self::{
    ir::module_ir::ModuleIR,
    ir::type_ir::{CompositeFieldIR, CompositeIR, CompositeIRKind, EnumIR, TypeIR, TypeIRKind},
    settings::{
        derives::{Derives, FlatDerivesRegistry},
        TypeGeneratorSettings,
    },
    type_params::TypeParameters,
    type_path::{TypeParameter, TypePath, TypePathType},
};

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};
use syn::parse_quote;

/// Custom error types.
pub mod error;
/// Intermediate representation of types and modules.
pub mod ir;
/// Settings passed into the `TypeGenerator`.
pub mod settings;
/// Logic for dealing with used and unused generic type parameters.
pub mod type_params;
/// Type path definition and conversion into tokens.
pub mod type_path;
/// Utility functions to validate that type paths in the user defined
/// derives and substitutes exist in a type registry.
pub mod validation;

/// An interface for generating a types module.
#[derive(Debug, Clone, Copy)]
pub struct TypeGenerator<'a> {
    type_registry: &'a PortableRegistry,
    settings: &'a TypeGeneratorSettings,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(type_registry: &'a PortableRegistry, settings: &'a TypeGeneratorSettings) -> Self {
        Self {
            type_registry,
            settings,
        }
    }

    /// The name of the generated module which will contain the generated types.
    pub fn types_mod_ident(&self) -> &Ident {
        &self.settings.types_mod_ident
    }

    /// The settings used by this type generator.
    pub fn settings(&self) -> &TypeGeneratorSettings {
        self.settings
    }

    /// The type registry backing this type generator.
    pub fn types(&self) -> &PortableRegistry {
        self.type_registry
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(&self) -> Result<ModuleIR, TypegenError> {
        sanity_pass(self.type_registry)?;

        let flat_derives_registry = self
            .settings
            .derives
            .clone()
            .flatten_recursive_derives(self.type_registry)?;

        let mut root_mod = ModuleIR::new(
            self.settings.types_mod_ident.clone(),
            self.settings.types_mod_ident.clone(),
        );

        for ty in &self.type_registry.types {
            let path = &ty.ty.path;
            // Don't generate a type if it was substituted - the target type might
            // not be in the type registry + our resolution already performs the substitution.
            if self.settings.substitutes.contains(&path.segments) {
                continue;
            }

            let namespace = path.namespace();
            // prelude types e.g. Option/Result have no namespace, so we don't generate them
            if namespace.is_empty() {
                continue;
            }

            // if the type is not a builtin type, insert it into the respective module
            let ty_id = ty.id;
            if let Some(type_ir) = self.create_type_ir(&ty.ty, &flat_derives_registry)? {
                // Create the module this type should go into
                let innermost_module = root_mod.get_or_insert_submodule(namespace);
                match innermost_module.types.entry(path.clone()) {
                    Entry::Vacant(e) => {
                        e.insert((ty_id, type_ir));
                    }
                    Entry::Occupied(e) => {
                        // There is already a type with the same type path present.
                        // We do not just want to override it, so we check if the two types are semantically similar (structure + generics).
                        // If not, return an error, if yes, just keep the first one.
                        let other_ty_id = e.get().0;
                        if !types_equal(ty_id, other_ty_id, self.type_registry) {
                            return Err(TypegenError::DuplicateTypePath(ty.ty.path.to_string()));
                        }
                    }
                };
            }
        }

        Ok(root_mod)
    }

    /// Creates an intermediate representation of a type that can later be converted into rust tokens.
    pub fn create_type_ir(
        &self,
        ty: &Type<PortableForm>,
        flat_derives_registry: &FlatDerivesRegistry,
    ) -> Result<Option<TypeIR>, TypegenError> {
        // if the type is some builtin, early return, we are only interested in generating structs and enums.
        if !matches!(ty.type_def, TypeDef::Composite(_) | TypeDef::Variant(_)) {
            return Ok(None);
        }

        let mut type_params = TypeParameters::from_scale_info(&ty.type_params);

        let name = ty
            .path
            .ident()
            .map(|e| syn::parse_str::<Ident>(&e))
            .expect(
            "Structs and enums should have a name. Checked with namespace.is_empty() above. qed;",
        )?;

        let docs = self.docs_from_scale_info(&ty.docs);

        let mut could_derive_as_compact: bool = false;
        let kind = match &ty.type_def {
            TypeDef::Composite(composite) => {
                let kind = self.create_composite_ir_kind(&composite.fields, &mut type_params)?;

                if kind.could_derive_as_compact() {
                    could_derive_as_compact = true;
                }

                TypeIRKind::Struct(CompositeIR { name, kind, docs })
            }
            TypeDef::Variant(variant) => {
                let variants = variant
                    .variants
                    .iter()
                    .map(|v| {
                        let name = syn::parse_str::<Ident>(&v.name)?;
                        let kind = self.create_composite_ir_kind(&v.fields, &mut type_params)?;
                        let docs = self.docs_from_scale_info(&v.docs);
                        Ok((v.index, CompositeIR { kind, name, docs }))
                    })
                    .collect::<Result<Vec<(u8, CompositeIR)>, TypegenError>>()?;
                TypeIRKind::Enum(EnumIR {
                    name,
                    variants,
                    docs,
                })
            }
            _ => unreachable!("Other variants early return before. qed."),
        };

        let mut derives = flat_derives_registry.resolve_derives_for_type(ty)?;
        if could_derive_as_compact {
            self.add_as_compact_derive(&mut derives);
        }

        let type_ir = TypeIR {
            kind,
            derives,
            type_params,
            insert_codec_attributes: self.settings.insert_codec_attributes,
        };
        Ok(Some(type_ir))
    }

    /// takes into account the settings value for `should_gen_docs`
    pub fn docs_from_scale_info(&self, docs: &[String]) -> TokenStream {
        self.settings
            .should_gen_docs
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default()
    }

    /// Creates an intermediate representation of a composite.
    pub fn create_composite_ir_kind(
        &self,
        fields: &[scale_info::Field<PortableForm>],
        type_params: &mut TypeParameters,
    ) -> Result<CompositeIRKind, TypegenError> {
        if fields.is_empty() {
            return Ok(CompositeIRKind::NoFields);
        }

        let all_fields_named = fields.iter().all(|f| f.name.is_some());
        let all_fields_unnamed = fields.iter().all(|f| f.name.is_none());

        if !(all_fields_named || all_fields_unnamed) {
            return Err(TypegenError::InvalidFields(format!("{:?}", fields)));
        }

        if all_fields_named {
            let named_fields = fields
                .iter()
                .map(|field| {
                    let field_name = field.name.as_ref().unwrap();
                    let ident = syn::parse_str::<Ident>(field_name)?;

                    let path = self.resolve_field_type_path(
                        field.ty.id,
                        type_params.params(),
                        field.type_name.as_deref(),
                    )?;
                    let is_compact = path.is_compact();
                    let is_boxed = field
                        .type_name
                        .as_ref()
                        .map(|e| e.contains("Box<"))
                        .unwrap_or_default();

                    for param in path.parent_type_params().iter() {
                        type_params.mark_used(param);
                    }

                    Ok((ident, CompositeFieldIR::new(path, is_compact, is_boxed)))
                })
                .collect::<Result<Vec<(Ident, CompositeFieldIR)>, TypegenError>>()?;
            Ok(CompositeIRKind::Named(named_fields))
        } else if all_fields_unnamed {
            let unnamed_fields = fields
                .iter()
                .map(|field| {
                    let path = self.resolve_field_type_path(
                        field.ty.id,
                        type_params.params(),
                        field.type_name.as_deref(),
                    )?;

                    let is_compact = path.is_compact();
                    let is_boxed = field
                        .type_name
                        .as_ref()
                        .map(|e| e.contains("Box<"))
                        .unwrap_or_default();

                    for param in path.parent_type_params().iter() {
                        type_params.mark_used(param);
                    }

                    Ok(CompositeFieldIR::new(path, is_compact, is_boxed))
                })
                .collect::<Result<Vec<CompositeFieldIR>, TypegenError>>()?;
            Ok(CompositeIRKind::Unnamed(unnamed_fields))
        } else {
            unreachable!("Is either all unnamed or all named. qed.")
        }
    }

    /// Creates the intermediate representation of a type from just a composite definition.
    /// This uses just the default derives and type params are left empty.
    pub fn upcast_composite(&self, composite: &CompositeIR) -> TypeIR {
        // just use Default Derives + AsCompact. No access to type specific derives here. (Mainly used in subxt to create structs from enum variants...)
        let mut derives = self.settings.derives.default_derives().clone();
        if composite.kind.could_derive_as_compact() {
            self.add_as_compact_derive(&mut derives)
        }
        TypeIR {
            type_params: TypeParameters::from_scale_info(&[]),
            derives,
            insert_codec_attributes: self.settings.insert_codec_attributes,
            kind: TypeIRKind::Struct(composite.clone()),
        }
    }

    /// Adds a AsCompact derive, if a path to AsCompact trait/derive macro set in settings.
    fn add_as_compact_derive(&self, derives: &mut Derives) {
        if let Some(compact_as_type_path) = &self.settings.compact_as_type_path {
            derives.insert_derive(parse_quote!(#compact_as_type_path));
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
    pub fn resolve_field_type_path(
        &self,
        id: u32,
        parent_type_params: &[TypeParameter],
        original_name: Option<&str>,
    ) -> Result<TypePath, TypegenError> {
        self.resolve_type_path_recurse(id, true, parent_type_params, original_name)
    }

    /// Get the type path for the given type identifier.
    pub fn resolve_type_path(&self, id: u32) -> Result<TypePath, TypegenError> {
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
    ) -> Result<TypePath, TypegenError> {
        if let Some(parent_type_param) = parent_type_params.iter().find(|tp| {
            tp.concrete_type_id == id
                && original_name.is_none_or(|original_name| tp.original_name == original_name)
        }) {
            let type_path = TypePath::from_parameter(parent_type_param.clone());
            return Ok(type_path);
        }

        let mut ty = self.resolve_type(id)?;

        if ty.path.ident() == Some("Cow".to_string()) {
            let inner_ty_id = ty.type_params[0]
                .ty
                .ok_or_else(|| {
                    TypegenError::InvalidType(
                        "type parameters to Cow are not expected to be skipped".into(),
                    )
                })?
                .id;
            ty = self.resolve_type(inner_ty_id)?
        }

        let params: Vec<TypePath> = ty
            .type_params
            .iter()
            .filter_map(|f| {
                f.ty.map(|f| self.resolve_type_path_recurse(f.id, false, parent_type_params, None))
            })
            .collect::<Result<Vec<TypePath>, TypegenError>>()?;

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
                    .collect::<Result<Vec<TypePath>, TypegenError>>()?;

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
                    .settings
                    .compact_type_path
                    .as_ref()
                    .ok_or(TypegenError::CompactPathNone)?
                    .clone();

                TypePathType::Compact {
                    inner: Box::new(inner_type),
                    is_field,
                    compact_type_path,
                }
            }
            TypeDef::BitSequence(bitseq) => {
                let decoded_bits_type_path = self
                    .settings
                    .decoded_bits_type_path
                    .as_ref()
                    .ok_or(TypegenError::DecodedBitsPathNone)?
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

    /// Converts a [`scale_info::Path`] into a [`TypePathType`], replacing all types that should be substituted.
    pub fn type_path_maybe_with_substitutes(
        &self,
        path: &scale_info::Path<PortableForm>,
        params: &[TypePath],
    ) -> TypePathType {
        if let Some(substitute) =
            self.settings
                .substitutes
                .for_path_with_params(&path.segments, params, self.settings)
        {
            substitute
        } else {
            TypePathType::from_type_def_path(
                path,
                self.settings.types_mod_ident.clone(),
                params.to_vec(),
                &self.settings.alloc_crate_path,
            )
        }
    }

    /// Resolves a type, given some type id.
    pub fn resolve_type(&self, id: u32) -> Result<&Type<PortableForm>, TypegenError> {
        let ty = self
            .type_registry
            .resolve(id)
            .ok_or(TypegenError::TypeNotFound(id))?;
        Ok(ty)
    }
}
