use parity_scale_codec::Compact;
use scale_info::{form::PortableForm, Type};

use super::{
    derives::{Derives, DerivesRegistry},
    substitutes::TypeSubstitutes,
};

pub struct TypeGeneratorSettings {
    /// The name of the module which will contain the generated types.
    pub type_mod_name: String,
    /// If false, no docs are generated for the types.
    pub should_gen_docs: bool,
    /// Derive traits on generated types.
    pub derives: DerivesRegistry,
    /// User defined overrides for generated types.
    pub substitutes: TypeSubstitutes,
    /// Two generic parameters are expected on this type:
    /// - Store (e.g. `u8`/`u16`/`u32`/`u64`)
    /// - Order (e.g. LSB, MSB)
    ///
    /// subxt provides a `subxt::utils::DecodedBits` that can be used here.
    pub decoded_bits_type_path: Option<syn::Path>,
}

impl Default for TypeGeneratorSettings {
    fn default() -> Self {
        Self {
            type_mod_name: "types".into(),
            should_gen_docs: true,
            substitutes: TypeSubstitutes::new(),
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
