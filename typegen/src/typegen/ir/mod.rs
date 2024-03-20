use quote::ToTokens;

use crate::TypeGeneratorSettings;

/// Intermediate Representation of a rust module.
pub mod module_ir;
/// Intermediate Representation of a rust type.
pub mod type_ir;

/// A trait that can translate a type into rust tokens similar to [`quote::ToTokens`], but that takes into account [`TypeGeneratorSettings`].
pub trait ToTokensWithSettingsT {
    /// Translate a type into rust tokens but takes into account [`TypeGeneratorSettings`].
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream, settings: &TypeGeneratorSettings);

    /// Translate a type into rust tokens but takes into account [`TypeGeneratorSettings`].
    fn to_token_stream(&self, settings: &TypeGeneratorSettings) -> proc_macro2::TokenStream {
        let mut tokens = proc_macro2::TokenStream::new();
        self.to_tokens(&mut tokens, settings);
        tokens
    }
}

struct ToTokensWithSettings<'a, T> {
    val: &'a T,
    settings: &'a TypeGeneratorSettings,
}
impl<'a, T> ToTokensWithSettings<'a, T> {
    fn new(val: &'a T, settings: &'a TypeGeneratorSettings) -> Self {
        Self { val, settings }
    }
}

impl<'a, T> ToTokens for ToTokensWithSettings<'a, T>
where
    T: ToTokensWithSettingsT,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.val.to_tokens(tokens, &self.settings)
    }
}
