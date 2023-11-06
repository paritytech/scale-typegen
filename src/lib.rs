use anyhow::anyhow;
use codegen::{
    derives::{Derives, DerivesRegistry},
    module_ir::ModuleIR,
    type_ir::{CompositeIR, CompositeIRKind, TypeIR, TypeIRKind},
    type_path::TypeParameter,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use scale_info::{form::PortableForm, Path, PortableRegistry, Type, TypeDef, TypeDefComposite};
use syn::parse_quote;

use crate::codegen::type_path::TypePath;

pub mod codegen;

pub struct Settings {
    /// The name of the module which will contain the generated types.
    mod_ident: Ident,
    // /// User defined overrides for generated types.
    // type_substitutes: TypeSubstitutes,
    should_gen_docs: bool,
    derives: DerivesRegistry,
    /// e.g. `subxt::utils::bits::DecodedBits`,
    decoded_bits_type_path: syn::Path,
    /// e.g. `subxt::ext::codec::Compact`,
    compact_type_path: syn::Path,
}

pub struct Generator<'a> {
    registry: &'a PortableRegistry,
    settings: Settings,
}

impl<'a> Generator<'a> {
    pub fn create_ir(&self) -> anyhow::Result<ModuleIR> {
        let root_mod_ident = self.settings.mod_ident.clone();
        let mut root_mod = ModuleIR::new(root_mod_ident.clone(), root_mod_ident);

        for ty in &self.registry.types {
            // todo!();
            // // Don't generate a type if it was substituted - the target type might
            // // not be in the type registry + our resolution already performs the substitution.
            // if self.type_substitutes.contains(path) {
            //     continue;
            // }

            let namespace = ty.ty.path.namespace();
            // prelude types e.g. Option/Result have no namespace, so we don't generate them
            if namespace.is_empty() {
                continue;
            }

            // Get or create submodules for the namespace path.
            let module_for_type = root_mod.get_or_insert_submodule(namespace);

            // type_ir is None for builtin types
            let ty_gen = TypeGenerator {
                generator: &self,
                ty: &ty.ty,
                ty_id: ty.id,
            };
            if let Some(type_ir) = ty_gen.create_ir()? {
                // insert type there.
                module_for_type.types.insert(ty.ty.path.clone(), type_ir);
            }
        }

        Ok(root_mod)
    }

    pub fn resolve_type(&self, id: u32) -> anyhow::Result<Type<PortableForm>> {
        let ty = self
            .registry
            .resolve(id)
            .ok_or_else(|| anyhow!("type {id} not found"))?;
        Ok(ty.clone())
    }

    pub fn resolve_type_path(&self, id: u32) -> anyhow::Result<TypePath> {
        self.resolve_type_path_recurse(id, &[], None)
    }

    fn maybe_substitute(&self, type_path: &scale_info::Path<PortableForm>, params: &[TypePath]) -> TypePath{
        // todo!() check substiutes here. Right now, just return the bare type path
        TypePath::Path { path: type_path.to, generics: () }
    }

    fn resolve_type_path_recurse(
        &self,
        id: u32,
        parent_type_params: &[TypeParameter],
        type_needs_to_have_name: Option<&str>,
    ) -> anyhow::Result<TypePath> {
        //

        if let Some(param) = parent_type_params.iter().find(|tp| {
            tp.concrete_type_id == id
                && type_needs_to_have_name.map_or(true, |name| tp.original_name == name)
        }) {
            return Ok(TypePath::Parameter(param.clone()));
        }

        let mut ty: Type<PortableForm> = self.resolve_type(id)?;

        if ty.path.ident() == Some("Cow".to_string()) {
            let pointee_id = ty.type_params[0]
                .ty
                .ok_or_else(|| anyhow!("type parameters to Cow are not expected to be skipped"))?
                .id;
            ty = self.resolve_type(pointee_id)?;
        }

        let params: Vec<TypePath> = ty
            .type_params
            .iter()
            .filter_map(|f| {
                f.ty.map(|f| self.resolve_type_path_recurse(f.id, parent_type_params, None))
            })
            .collect::<anyhow::Result<Vec<TypePath>>>()?;

        let ty = match &ty.type_def {
            TypeDef::Composite(_) | TypeDef::Variant(_) => {

                self.

                if let Some(ty) = self
                    .type_substitutes
                    .for_path_with_params(&ty.path, &params)
                {
                    ty
                } else {
                    TypePathType::from_type_def_path(
                        &ty.path,
                        self.settings.mod_ident.clone(),
                        params,
                    )
                }
            }
            TypeDef::Primitive(primitive) => TypePathType::Primitive {
                def: primitive.clone(),
            },
            TypeDef::Array(arr) => TypePathType::Array {
                len: arr.len as usize,
                of: Box::new(self.resolve_type_path_recurse(
                    arr.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )),
            },
            TypeDef::Sequence(seq) => TypePathType::Vec {
                of: Box::new(self.resolve_type_path_recurse(
                    seq.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )),
            },
            TypeDef::Tuple(tuple) => TypePathType::Tuple {
                elements: tuple
                    .fields
                    .iter()
                    .map(|f| self.resolve_type_path_recurse(f.id, false, parent_type_params, None))
                    .collect(),
            },
            TypeDef::Compact(compact) => TypePathType::Compact {
                inner: Box::new(self.resolve_type_path_recurse(
                    compact.type_param.id,
                    false,
                    parent_type_params,
                    None,
                )),
                is_field,
                crate_path: self.crate_path.clone(),
            },
            TypeDef::BitSequence(bitseq) => TypePathType::BitVec {
                bit_order_type: Box::new(self.resolve_type_path_recurse(
                    bitseq.bit_order_type.id,
                    false,
                    parent_type_params,
                    None,
                )),
                bit_store_type: Box::new(self.resolve_type_path_recurse(
                    bitseq.bit_store_type.id,
                    false,
                    parent_type_params,
                    None,
                )),
                crate_path: self.crate_path.clone(),
            },
        };

        todo!()
        // TypePath::from_type(ty)
    }
}

pub struct TypeGenerator<'a> {
    generator: &'a Generator<'a>,
    ty_id: u32,
    ty: &'a Type<PortableForm>,
}

impl<'a> TypeGenerator<'a> {
    fn should_gen_docs(&self) -> bool {
        self.generator.settings.should_gen_docs
    }

    fn derives(&self) -> anyhow::Result<Derives> {
        let joined_path = self.ty.path.segments.join("::");
        let ty_path: syn::TypePath = syn::parse_str(&joined_path)?;
        let derives = self.generator.settings.derives.resolve(&ty_path);
        Ok(derives)
    }

    fn create_ir(&self) -> anyhow::Result<Option<TypeIR>> {
        // early return Ok(None) if this type is not a composite or variant:
        if !matches!(
            self.ty.type_def,
            TypeDef::Composite(_) | TypeDef::Variant(_)
        ) {
            return Ok(None);
        }

        // extract docs if the should_gen_docs flag is set.
        let docs = &self.ty.docs;
        let docs = self
            .should_gen_docs()
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default();

        // parse the type name as an ident
        let type_name: Ident = self
            .ty
            .path
            .ident()
            .map(|s| syn::parse_str::<Ident>(&s))
            .ok_or_else(|| anyhow!("path is empty, cannot get ident."))??;

        let kind = match &self.ty.type_def {
            TypeDef::Composite(type_def) => {
                let kind = Self::create_composite_ir_kind(type_def, &self.ty.path)?;
                let composite_ir = CompositeIR {
                    name: type_name,
                    kind,
                };
                TypeIRKind::Struct(composite_ir)
            }
            TypeDef::Variant(_) => {
                let variants = todo!();

                TypeIRKind::Enum(type_name, variants)
            }
            _ => unreachable!(),
        };

        let derives = self.derives()?;
        let ir = TypeIR {
            derives,
            kind,
            docs,
        };
        Ok(Some(ir))
    }

    fn create_composite_ir_kind(
        type_def: &TypeDefComposite<PortableForm>,
        path: &Path<PortableForm>,
    ) -> anyhow::Result<CompositeIRKind> {
        if type_def.fields.is_empty() {
            return Ok(CompositeIRKind::NoFields);
        }

        let fields_are_named = type_def.fields.iter().any(|f| f.name.is_some());
        let fields_are_unnamed = type_def.fields.iter().any(|f| f.name.is_none());

        if fields_are_named && fields_are_unnamed {
            return Err(anyhow!(
                "Found named and unnamed fields in a single Composite"
            ));
        }

        if fields_are_named {
            let mut fields: Vec<(Ident, TypePath)> = vec![];
            for f in &type_def.fields {
                let field_name = f.name.as_ref().unwrap();
                let field_ident: Ident = syn::parse_str(&field_name)?;
                let type_path = f.ty.id;
            }
        }

        todo!()
    }
}
