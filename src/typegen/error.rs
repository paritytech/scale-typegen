use proc_macro2::Span;

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
    #[error("Type substitution error: {0}")]
    InvalidSubstitute(#[from] TypeSubstitutionError),
}

/// Error attempting to do type substitution.
#[derive(Debug, thiserror::Error)]
pub struct TypeSubstitutionError {
    pub span: Span,
    pub kind: TypeSubstitutionErrorKind,
}

impl std::fmt::Display for TypeSubstitutionError {
    fn fmt(
        &self,
        f: &mut scale_info::prelude::fmt::Formatter<'_>,
    ) -> scale_info::prelude::fmt::Result {
        f.write_fmt(format_args!("{}", self.kind))
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypeSubstitutionErrorKind {
    /// Substitute "to" type must be an absolute path.
    #[error("`substitute_type(with = <path>)` must be a path prefixed with 'crate::' or '::'")]
    ExpectedAbsolutePath,
    /// Substitute types must have a valid path.
    #[error("Substitute types must have a valid path.")]
    EmptySubstitutePath,
    /// From/To substitution types should use angle bracket generics.
    #[error("Expected the from/to type generics to have the form 'Foo<A,B,C..>'.")]
    ExpectedAngleBracketGenerics,
    /// Source substitute type must be an ident.
    #[error("Expected an ident like 'Foo' or 'A' to mark a type to be substituted.")]
    InvalidFromType,
    /// Target type is invalid.
    #[error("Expected an ident like 'Foo' or an absolute concrete path like '::path::to::Bar' for the substitute type.")]
    InvalidToType,
    /// Target ident doesn't correspond to any source type.
    #[error("Cannot find matching param on 'from' type.")]
    NoMatchingFromType,
}
