use crate::TypeGeneratorSettings;

/// Intermediate Representation of a rust module.
pub mod module_ir;
/// Intermediate Representation of a rust type.
pub mod type_ir;

/// A trait that can translate a type into rust tokens similar to [`quote::ToTokens`], but
/// takes into account the `alloc_crate_path` from [`TypeGeneratorSettings`].
pub trait ToTokensWithSettings {
    /// Translate a type into rust tokens.
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, settings: &TypeGeneratorSettings);

    /// Translate a type into rust tokens.
    fn to_token_stream(&self, settings: &TypeGeneratorSettings) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        self.to_tokens(&mut tokens, settings);
        tokens
    }
}
