use self::{
    derives::{Derives, DerivesRegistry},
    module_ir::ModuleIR,
    settings::TypeGeneratorSettings,
    type_ir::{CompositeFieldIR, CompositeIR, CompositeIRKind, EnumIR, TypeIR, TypeIRKind},
    type_params::TypeParameters,
    type_path::{TypeParameter, TypePath, TypePathType},
    type_path_resolver::TypePathResolver,
};
use anyhow::anyhow;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use scale_info::{form::PortableForm, PortableRegistry, PortableType, Type, TypeDef};

pub mod derives;
pub mod module_ir;
pub mod settings;
pub mod substitutes;
pub mod type_ir;
pub mod type_params;
pub mod type_path;
pub mod type_path_resolver;

#[cfg(test)]
mod tests;

pub struct TypeGenerator<'a> {
    type_registry: &'a PortableRegistry,
    settings: TypeGeneratorSettings,
    root_mod_ident: Ident,
}

impl<'a> TypeGenerator<'a> {
    /// Construct a new [`TypeGenerator`].
    pub fn new(
        type_registry: &'a PortableRegistry,
        settings: TypeGeneratorSettings,
    ) -> anyhow::Result<Self> {
        let root_mod_ident: Ident = syn::parse_str(&settings.type_mod_name)?;
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
            if self.settings.substitutes.contains(path) {
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
        let mut type_params = TypeParameters::new(type_params);

        let name = ty
            .path
            .ident()
            .map(|e| syn::parse_str::<Ident>(&e))
            .ok_or_else(|| anyhow!("Structs and enums should have names"))??;

        let kind = match &ty.type_def {
            TypeDef::Composite(composite) => {
                let kind = self.create_composite_ir_kind(&composite.fields, &mut type_params)?;
                TypeIRKind::Struct(CompositeIR { name, kind })
            }
            TypeDef::Variant(variant) => {
                let variants = variant
                    .variants
                    .iter()
                    .map(|v| {
                        let name = syn::parse_str::<Ident>(&v.name)?;
                        let kind = self.create_composite_ir_kind(&v.fields, &mut type_params)?;
                        Ok((v.index, CompositeIR { kind, name }))
                    })
                    .collect::<anyhow::Result<Vec<(u8, CompositeIR)>>>()?;
                TypeIRKind::Enum(EnumIR { name, variants })
            }
            _ => unreachable!("Other variants early return before. qed."),
        };

        let derives = self.settings.type_derives(&ty)?;

        let docs = &ty.docs;
        let docs = self
            .settings
            .should_gen_docs
            .then_some(quote! { #( #[doc = #docs ] )* })
            .unwrap_or_default();

        let type_ir = TypeIR {
            kind,
            derives,
            docs,
            type_params,
        };
        Ok(Some(type_ir))
    }

    fn create_composite_ir_kind(
        &self,
        fields: &[scale_info::Field<PortableForm>],
        type_params: &mut TypeParameters,
    ) -> anyhow::Result<CompositeIRKind> {
        let type_path_resolver = self.type_path_resolver();

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

                    let path = type_path_resolver.resolve_field_type_path(
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
                    dbg!(&is_boxed, &path);
                    for param in path.parent_type_params().iter() {
                        type_params.mark_used(param);
                    }

                    Ok((ident, CompositeFieldIR::new(path, is_compact, is_boxed)))
                })
                .collect::<anyhow::Result<Vec<(Ident, CompositeFieldIR)>>>()?;
            Ok(CompositeIRKind::Named(named_fields))
        } else if all_fields_unnamed {
            let unnamed_fields = fields
                .iter()
                .map(|field| {
                    let path = type_path_resolver.resolve_field_type_path(
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
                    dbg!(&is_boxed, &path);
                    for param in path.parent_type_params().iter() {
                        type_params.mark_used(param);
                    }

                    Ok(CompositeFieldIR::new(path, is_compact, is_boxed))
                })
                .collect::<anyhow::Result<Vec<CompositeFieldIR>>>()?;
            Ok(CompositeIRKind::Unnamed(unnamed_fields))
        } else {
            unreachable!("Is either all unnamed or all named. qed.")
        }
    }

    fn type_path_resolver(&self) -> TypePathResolver<'_> {
        TypePathResolver::new(
            &self.type_registry,
            &self.settings.substitutes,
            self.settings.decoded_bits_type_path.as_ref(),
            &self.root_mod_ident,
        )
    }
}
