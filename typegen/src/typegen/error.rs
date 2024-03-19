use std::collections::HashSet;

use proc_macro2::Span;
use quote::ToTokens;

/// Error for when something went wrong during type generation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypegenError {
    /// Could not parse into a syn type.
    #[error("Could not parse into a syn type: {0}")]
    SynParseError(#[from] syn::Error),
    /// Fields should either be all named or all unnamed, make sure you are providing a valid metadata.
    #[error("Fields should either be all named or all unnamed, make sure you are providing a valid metadata: {0}")]
    InvalidFields(String),
    /// A type in the metadata was invalid
    #[error("A type in the metadata was invalid: {0}")]
    InvalidType(String),
    /// Could not generate a type that contains a compact type, because the Compact type path is not set in the settings.
    #[error("Could not generate a type that contains a compact type, because the Compact type path is not set in the settings.")]
    CompactPathNone,
    /// Could not generate a type that contains a bit sequence, because the DecodedBits type path is not set in the settings.
    #[error("Could not generate a type that contains a bit sequence, because the DecodedBits type path is not set in the settings.")]
    DecodedBitsPathNone,
    /// Could not find type with ID in the type registry.
    #[error("Could not find type with ID {0} in the type registry.")]
    TypeNotFound(u32),
    /// Type substitution error.
    #[error("Type substitution error: {0}")]
    InvalidSubstitute(#[from] TypeSubstitutionError),
    /// The settings do not fit the given type registry.
    #[error("Settings do not fit the given type registry: {0}")]
    SettingsValidation(SettingsValidationError),
    /// There are two types with the the same type path but a different structure.
    /// Use [`crate::utils::ensure_unique_type_paths`] on your [`scale_info::PortableRegistry`] to deduplicate type paths.
    #[error("There are two types with the the same type path {0} but different structure. Use `scale_typegen::utils::ensure_unique_type_paths` on your `PortableRegistry` before, to avoid this error.")]
    DuplicateTypePath(String),
}

/// Error attempting to do type substitution.
#[derive(Debug, thiserror::Error)]
pub struct TypeSubstitutionError {
    /// Where in the code the error occured.
    pub span: Span,
    /// Kind of TypeSubstitutionError that happended.
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

/// Error attempting to do type substitution.
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

/// Error attempting to do type substitution.
#[derive(Debug, thiserror::Error, Default, PartialEq, Eq)]
pub struct SettingsValidationError {
    /// Cannot add derive for type that is not present in the registry.
    pub derives_for_unknown_types: Vec<(syn::Path, HashSet<syn::Path>)>,
    /// Cannot add attribute for type that is not present in the registry.
    pub attributes_for_unknown_types: Vec<(syn::Path, HashSet<syn::Attribute>)>,
    /// Invalid path to replace: the type getting substituted is not present in the registry.
    pub substitutes_for_unknown_types: Vec<(syn::Path, syn::Path)>,
}

impl std::fmt::Display for SettingsValidationError {
    fn fmt(
        &self,
        f: &mut scale_info::prelude::fmt::Formatter<'_>,
    ) -> scale_info::prelude::fmt::Result {
        fn display_one(e: &impl ToTokens) -> String {
            e.to_token_stream().to_string().replace(' ', "")
        }

        fn display_many(set: &HashSet<impl ToTokens>) -> String {
            set.iter()
                .map(|e| display_one(e))
                .collect::<Vec<_>>()
                .join(", ")
        }

        writeln!(f, "Settings validation error:")?;

        if !self.derives_for_unknown_types.is_empty() {
            writeln!(f, "  Derives for unknown types:")?;
            for (path, derives) in &self.derives_for_unknown_types {
                writeln!(
                    f,
                    "    {} (Derives: {})",
                    display_one(path),
                    display_many(derives)
                )?;
            }
        }

        if !self.attributes_for_unknown_types.is_empty() {
            writeln!(f, "  Attributes for unknown types:")?;
            for (path, attributes) in &self.attributes_for_unknown_types {
                writeln!(
                    f,
                    "    {} (Attributes: {})",
                    display_one(path),
                    display_many(attributes)
                )?;
            }
        }

        if !self.substitutes_for_unknown_types.is_empty() {
            writeln!(f, "  Substitutes for unknown types:")?;
            for (original, substitute) in &self.substitutes_for_unknown_types {
                writeln!(
                    f,
                    "    {} (Substitute: {})",
                    display_one(original),
                    display_one(substitute),
                )?;
            }
        }

        Ok(())
    }
}

impl SettingsValidationError {
    pub(crate) fn is_empty(&self) -> bool {
        self.derives_for_unknown_types.is_empty()
            && self.attributes_for_unknown_types.is_empty()
            && self.substitutes_for_unknown_types.is_empty()
    }
}
