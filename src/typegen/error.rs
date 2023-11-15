#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypegenError {
    #[error("Could not parse into a syn type: {0}")]
    SynParseError(#[from] syn::Error),
    #[error("Fields should either be all named or all unnamed, make sure you are providing a valid metadata: {0}")]
    InvalidFields(String),
    #[error("A type in the metadata was invalid: {0}")]
    InvalidType(String),
    #[error("Could not generate a type that contains a compact type, because the Compact type path is not set in the settings.")]
    CompactPathNone,
    #[error("Could not generate a type that contains a bit sequence, because the DecodedBits type path is not set in the settings.")]
    DecodedBitsPathNone,
    #[error("Could not find type with ID {0} in the type registry.")]
    TypeNotFound(u32),
}
