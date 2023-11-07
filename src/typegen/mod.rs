use self::{
    derives::{Derives, DerivesRegistry},
    module_ir::ModuleIR,
    type_def_params::TypeDefParameters,
    type_ir::{CompositeIR, CompositeIRKind, EnumIR, TypeIR, TypeIRKind},
    type_path::{TypeParameter, TypePath, TypePathType},
    unused_type_params::UnusedTypeParams,
};
use anyhow::anyhow;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use scale_info::{form::PortableForm, PortableRegistry, PortableType, Type, TypeDef};

mod derives;
pub mod module_ir;
pub mod type_def_params;
pub mod type_ir;
pub mod type_path;
pub mod unused_type_params;

pub struct Settings {
    /// The name of the module which will contain the generated types.
    type_mod_ident: String,
    // /// User defined overrides for generated types.
    // type_substitutes: TypeSubstitutes,
    should_gen_docs: bool,
    derives: DerivesRegistry,
    /// e.g. `subxt::utils::bits::DecodedBits`,
    ///
    /// If set to None, the type generation fails when `BitSequence`s are encountered.
    decoded_bits_type_path: Option<syn::Path>,
    /// e.g. `subxt::ext::codec::Compact`,
    ///
    /// If set to None, the type generation fails when `Compact`s are encountered.
    compact_type_path: Option<syn::Path>,
}

impl Settings {
    pub fn type_derives(&self, ty: &Type<PortableForm>) -> anyhow::Result<Derives> {
        let joined_path = ty.path.segments.join("::");
        let ty_path: syn::TypePath = syn::parse_str(&joined_path)?;
        Ok(self.derives.resolve(&ty_path))
    }
}

pub struct TypeGenerator<'a> {
    type_registry: &'a PortableRegistry,
    settings: Settings,
    root_mod_ident: Ident,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(type_registry: &'a PortableRegistry, settings: Settings) -> anyhow::Result<Self> {
        let root_mod_ident: Ident = syn::parse_str(&settings.type_mod_ident)?;
        Ok(Self {
            type_registry,
            settings,
            root_mod_ident,
        })
    }

    /// Generate a module containing all types defined in the supplied type registry.
    pub fn generate_types_mod(&self) -> anyhow::Result<ModuleIR> {
        let mut root_mod = ModuleIR::new(self.root_mod_ident.clone(), self.root_mod_ident.clone());

        for ty in &self.type_registry.types {
            let path = &ty.ty.path;
            // Don't generate a type if it was substituted - the target type might
            // not be in the type registry + our resolution already performs the substitution.
            if self.should_substitute_path(path) {
                continue;
            }

            let namespace = path.namespace();
            // prelude types e.g. Option/Result have no namespace, so we don't generate them
            if namespace.is_empty() {
                continue;
            }

            // if the type is not a builtin type, insert it into the respective module
            if let Some(type_ir) = self.create_type_ir(ty)? {
                // Create the module this type should go into
                let innermost_module = root_mod.get_or_insert_submodule(namespace);
                innermost_module.types.insert(path.clone(), type_ir);
            }
        }

        Ok(root_mod)
    }

    fn create_type_ir(&self, ty: &PortableType) -> anyhow::Result<Option<TypeIR>> {
        let PortableType { ty, id } = &ty;

        // if the type is some builtin, early return, we are only interested in generating structs and enums.
        if !matches!(ty.type_def, TypeDef::Composite(_) | TypeDef::Variant(_)) {
            return Ok(None);
        }

        let derives = self.settings.type_derives(&ty)?;

        let type_params = ty
            .type_params
            .iter()
            .enumerate()
            .filter_map(|(i, tp)| {
                tp.ty.as_ref().map(|ty| {
                    let tp_name = format_ident!("_{}", i);
                    TypeParameter {
                        concrete_type_id: ty.id,
                        original_name: tp.name.clone(),
                        name: tp_name,
                    }
                })
            })
            .collect::<Vec<_>>();

        let mut unused_type_params = UnusedTypeParams::new(&type_params);

        let name = ty
            .path
            .ident()
            .map(|e| syn::parse_str::<Ident>(&e))
            .ok_or_else(|| anyhow!("Structs and enums should have names"))??;

        let kind = match &ty.type_def {
            TypeDef::Composite(composite) => {
                // go over the fields and subsequently remove used type params from unused params
                let kind = self.create_composite_ir_kind(
                    &composite.fields,
                    &type_params,
                    &mut unused_type_params,
                )?;

                let mut ir = CompositeIR { name, kind };
                ir.add_phantom_data(unused_type_params);
                TypeIRKind::Struct(ir)
            }
            TypeDef::Variant(variant) => {
                let variants = variant
                    .variants
                    .iter()
                    .map(|v| {
                        let name = syn::parse_str::<Ident>(&v.name)?;
                        let kind = self.create_composite_ir_kind(
                            &v.fields,
                            &type_params,
                            &mut unused_type_params,
                        )?;
                        Ok((v.index, CompositeIR { kind, name }))
                    })
                    .collect::<anyhow::Result<Vec<(u8, CompositeIR)>>>()?;

                let mut ir = EnumIR { name, variants };
                ir.add_phantom_data(unused_type_params);
                TypeIRKind::Enum(ir)
            }
            _ => unreachable!("Other variants early return before. qed."),
        };

        let docs = &ty.docs;
        let docs = self
            .settings
            .should_gen_docs
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default();

        let type_ir = TypeIR {
            derives,
            kind,
            docs,
        };

        // Self {
        //     type_params,
        //     derives,
        //     ty_kind,
        //     ty_docs,
        // }
        Ok(Some(type_ir))
    }

    fn create_composite_ir_kind(
        &self,
        fields: &[scale_info::Field<PortableForm>],
        parent_type_params: &[TypeParameter],
        unused_type_params: &mut UnusedTypeParams,
    ) -> anyhow::Result<CompositeIRKind> {
        if fields.is_empty() {
            return Ok(CompositeIRKind::NoFields);
        }

        let all_fields_named = fields.iter().all(|f| f.name.is_some());
        let all_fields_unnamed = fields.iter().all(|f| f.name.is_none());

        if !(all_fields_named || all_fields_unnamed) {
            return Err(anyhow!("Mix of named and unnamed fields encountered"));
        }

        if all_fields_named {
            let named_fields = fields
                .iter()
                .map(|field| {
                    let field_name = field.name.as_ref().unwrap();
                    let ident = syn::parse_str::<Ident>(&field_name)?;
                    let path = self.resolve_field_type_path(
                        field.ty.id,
                        parent_type_params,
                        Some(&field_name),
                    )?;

                    for param in path.parent_type_params().iter() {
                        unused_type_params.remove(param)
                    }

                    Ok((ident, path))
                })
                .collect::<anyhow::Result<Vec<(Ident, TypePath)>>>()?;
            Ok(CompositeIRKind::Named(named_fields))
        } else if all_fields_unnamed {
            let unnamed_fields = fields
                .iter()
                .map(|field| {
                    let path =
                        self.resolve_field_type_path(field.ty.id, parent_type_params, None)?;
                    for param in path.parent_type_params().iter() {
                        unused_type_params.remove(param)
                    }
                    Ok(path)
                })
                .collect::<anyhow::Result<Vec<TypePath>>>()?;
            Ok(CompositeIRKind::UnNamed(unnamed_fields))
        } else {
            unreachable!("Is either all unnamed or all named. qed.")
        }
    }

    fn should_substitute_path(&self, path: &scale_info::Path<PortableForm>) -> bool {
        todo!()
    }

    pub fn resolve_type(&self, id: u32) -> anyhow::Result<Type<PortableForm>> {
        let ty = self
            .type_registry
            .resolve(id)
            .ok_or_else(|| anyhow!("No type with id {id} found"))?;
        Ok(ty.clone())
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
                    .settings
                    .compact_type_path
                    .as_ref()
                    .ok_or_else(|| {
                        anyhow!("Compact type path is None, cannot generate types with compact encoded fields.")
                    })?
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
                    .ok_or_else(|| anyhow!("DecodedBits type path is None, cannot generate types with bit sequences."))?
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
        // todo!("do substitutes here");

        TypePathType::from_type_def_path(path, self.root_mod_ident.clone(), params.clone())
    }
}
