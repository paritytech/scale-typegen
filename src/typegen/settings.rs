use parity_scale_codec::Compact;
use scale_info::{form::PortableForm, Type};

use super::derives::{Derives, DerivesRegistry};

pub struct TypeGeneratorSettings {
    /// The name of the module which will contain the generated types.
    pub type_mod_ident: String,
    // /// User defined overrides for generated types.
    // type_substitutes: TypeSubstitutes,
    pub should_gen_docs: bool,
    pub derives: DerivesRegistry,
    /// e.g. `subxt::utils::DecodedBits`. Two generic parameters are expected on this type.
    pub decoded_bits_type_path: Option<syn::Path>,
}

impl Default for TypeGeneratorSettings {
    fn default() -> Self {
        Self {
            type_mod_ident: "types".into(),
            should_gen_docs: true,
            derives: DerivesRegistry::new(),
            decoded_bits_type_path: None,
        }
    }
}

impl TypeGeneratorSettings {
    pub fn type_derives(&self, ty: &Type<PortableForm>) -> anyhow::Result<Derives> {
        let joined_path = ty.path.segments.join("::");
        let ty_path: syn::TypePath = syn::parse_str(&joined_path)?;
        Ok(self.derives.resolve(&ty_path))
    }
}
