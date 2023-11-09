use proc_macro2::Span;
use std::{borrow::Borrow, collections::HashMap};
use syn::spanned::Spanned as _;

use crate::typegen::type_path::{TypePath, TypePathType};

/// A map of type substitutes. We match on the paths to generated types in order
/// to figure out when to swap said type with some provided substitute.
#[derive(Debug)]
pub struct TypeSubstitutes {
    substitutes: HashMap<PathSegments, Substitute>,
}

#[derive(Debug)]
struct Substitute {
    path: syn::Path,
    param_mapping: TypeParamMapping,
}

/// Error attempting to do type substitution.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TypeSubstitutionError {
    /// Substitute "to" type must be an absolute path.
    #[error("`substitute_type(with = <path>)` must be a path prefixed with 'crate::' or '::'")]
    ExpectedAbsolutePath(Span),
    /// Substitute types must have a valid path.
    #[error("Substitute types must have a valid path.")]
    EmptySubstitutePath(Span),
    /// From/To substitution types should use angle bracket generics.
    #[error("Expected the from/to type generics to have the form 'Foo<A,B,C..>'.")]
    ExpectedAngleBracketGenerics(Span),
    /// Source substitute type must be an ident.
    #[error("Expected an ident like 'Foo' or 'A' to mark a type to be substituted.")]
    InvalidFromType(Span),
    /// Target type is invalid.
    #[error("Expected an ident like 'Foo' or an absolute concrete path like '::path::to::Bar' for the substitute type.")]
    InvalidToType(Span),
    /// Target ident doesn't correspond to any source type.
    #[error("Cannot find matching param on 'from' type.")]
    NoMatchingFromType(Span),
}

#[derive(Debug)]
enum TypeParamMapping {
    // Pass any generics from source to target type
    PassThrough,
    // Replace any ident seen in the path with the input generic type at this index
    Specified(Vec<(syn::Ident, usize)>),
}

impl Default for TypeSubstitutes {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeSubstitutes {
    /// Creates a new `TypeSubstitutes` with no default derives.
    pub fn new() -> Self {
        Self {
            substitutes: HashMap::new(),
        }
    }

    /// Insert the given substitution, overwriting any other with the same path.
    pub fn insert(
        &mut self,
        source: syn::Path,
        target: AbsolutePath,
    ) -> Result<(), TypeSubstitutionError> {
        let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
        self.substitutes.insert(key, val);
        Ok(())
    }

    /// Only insert the given substitution if a substitution at that path doesn't
    /// already exist.
    pub fn insert_if_not_exists(
        &mut self,
        source: syn::Path,
        target: AbsolutePath,
    ) -> Result<(), TypeSubstitutionError> {
        let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
        self.substitutes.entry(key).or_insert(val);
        Ok(())
    }

    /// Add a bunch of source to target type substitutions.
    pub fn extend(
        &mut self,
        elems: impl IntoIterator<Item = (syn::Path, AbsolutePath)>,
    ) -> Result<(), TypeSubstitutionError> {
        for (source, target) in elems.into_iter() {
            let (key, val) = TypeSubstitutes::parse_path_substitution(source, target.0)?;
            self.substitutes.insert(key, val);
        }
        Ok(())
    }

    /// Given a source and target path, parse the type params to work out the mapping from
    /// source to target, and output the source => substitution mapping that we work out from this.
    fn parse_path_substitution(
        src_path: syn::Path,
        target_path: syn::Path,
    ) -> Result<(PathSegments, Substitute), TypeSubstitutionError> {
        let param_mapping = Self::parse_path_param_mapping(&src_path, &target_path)?;

        Ok((
            PathSegments::from(&src_path),
            Substitute {
                // Note; at this point, target_path might have some generics still. These
                // might be hardcoded types that we want to keep, so leave them here for now.
                path: target_path,
                param_mapping,
            },
        ))
    }

    /// Given a source and target path, parse the type params to work out the mapping from
    /// source to target, and return it.
    fn parse_path_param_mapping(
        src_path: &syn::Path,
        target_path: &syn::Path,
    ) -> Result<TypeParamMapping, TypeSubstitutionError> {
        let Some(syn::PathSegment {
            arguments: src_path_args,
            ..
        }) = src_path.segments.last()
        else {
            return Err(TypeSubstitutionError::EmptySubstitutePath(src_path.span()));
        };
        let Some(syn::PathSegment {
            arguments: target_path_args,
            ..
        }) = target_path.segments.last()
        else {
            return Err(TypeSubstitutionError::EmptySubstitutePath(
                target_path.span(),
            ));
        };

        // Get hold of the generic args for the "from" type, erroring if they aren't valid.
        let source_args = match src_path_args {
            syn::PathArguments::None => {
                // No generics defined on the source type:
                Vec::new()
            }
            syn::PathArguments::AngleBracketed(args) => {
                // We have generics like <A,B> defined on the source type (error for any non-ident type):
                args.args
                    .iter()
                    .map(|arg| match get_valid_from_substitution_type(arg) {
                        Some(ident) => Ok(ident),
                        None => Err(TypeSubstitutionError::InvalidFromType(arg.span())),
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }
            syn::PathArguments::Parenthesized(args) => {
                // Generics like (A,B) -> defined; not allowed:
                return Err(TypeSubstitutionError::ExpectedAngleBracketGenerics(
                    args.span(),
                ));
            }
        };

        // Get hold of the generic args for the "to" type, erroring if they aren't valid.
        let target_args = match target_path_args {
            syn::PathArguments::None => {
                // No generics on target.
                Vec::new()
            }
            syn::PathArguments::AngleBracketed(args) => {
                // We have generics like <A,B> defined on the target type.
                args.args
                    .iter()
                    .map(|arg| match get_valid_to_substitution_type(arg) {
                        Some(arg) => Ok(arg),
                        None => Err(TypeSubstitutionError::InvalidToType(arg.span())),
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }
            syn::PathArguments::Parenthesized(args) => {
                // Generics like (A,B) -> defined; not allowed:
                return Err(TypeSubstitutionError::ExpectedAngleBracketGenerics(
                    args.span(),
                ));
            }
        };

        // If no generics defined on source or target, we just apply any concrete generics
        // to the substitute type.
        if source_args.is_empty() && target_args.is_empty() {
            return Ok(TypeParamMapping::PassThrough);
        }

        // Make a note of the idents in the source args and their indexes.
        let mapping = source_args
            .into_iter()
            .enumerate()
            .map(|(idx, ident)| (ident.clone(), idx))
            .collect();

        Ok(TypeParamMapping::Specified(mapping))
    }

    /// Given a source type path, return whether a substitute exists for it.
    pub fn contains(&self, path: impl Into<PathSegments>) -> bool {
        let path_segments: PathSegments = path.into();
        self.substitutes.contains_key(&path_segments)
    }

    /// Given a source type path and the resolved, supplied type parameters,
    /// return a new path and optionally overwritten type parameters.
    pub fn for_path_with_params(
        &self,
        path: impl Into<PathSegments>,
        params: &[TypePath],
    ) -> Option<TypePathType> {
        // If we find a substitute type, we'll take the substitute path, and
        // swap any idents with their new concrete types.
        fn replace_params(
            mut substitute_path: syn::Path,
            params: &[TypePath],
            mapping: &TypeParamMapping,
        ) -> TypePathType {
            match mapping {
                // We need to map the input params to the output params somehow:
                TypeParamMapping::Specified(mapping) => {
                    // A map from ident name to replacement path.
                    let replacement_map: Vec<(&syn::Ident, &TypePath)> = mapping
                        .iter()
                        .filter_map(|(ident, idx)| params.get(*idx).map(|param| (ident, param)))
                        .collect();

                    // Replace params in our substitute path with the incoming ones as needed.
                    // No need if no replacements given.
                    if !replacement_map.is_empty() {
                        replace_path_params_recursively(&mut substitute_path, &replacement_map);
                    }

                    TypePathType::Path {
                        path: substitute_path,
                        params: Vec::new(),
                    }
                }
                // No mapping; just hand back the substitute path and input params.
                TypeParamMapping::PassThrough => TypePathType::Path {
                    path: substitute_path,
                    params: params.to_vec(),
                },
            }
        }

        let path = path.into();

        self.substitutes
            .get(&path)
            .map(|sub| replace_params(sub.path.clone(), params, &sub.param_mapping))
    }
}

#[macro_export]
macro_rules! path_segments {
    ($($ident: ident)::*) => {
        PathSegments(
            [$(stringify!($ident)),*].into_iter().map(String::from).collect::<Vec<_>>()
        )
    }
}

/// Identifiers joined by the `::` separator.
///
/// We use this as a common denominator, since we need a consistent keys for both
/// `syn::TypePath` and `scale_info::ty::path::Path` types.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PathSegments(pub Vec<String>);

impl From<&syn::Path> for PathSegments {
    fn from(path: &syn::Path) -> Self {
        PathSegments(path.segments.iter().map(|x| x.ident.to_string()).collect())
    }
}

impl<'a> From<Vec<&'a str>> for PathSegments {
    fn from(path: Vec<&'a str>) -> Self {
        PathSegments(path.iter().map(|x| x.to_string()).collect())
    }
}

impl<T: scale_info::form::Form> From<&scale_info::Path<T>> for PathSegments {
    fn from(path: &scale_info::Path<T>) -> Self {
        PathSegments(
            path.segments
                .iter()
                .map(|x| x.as_ref().to_owned())
                .collect(),
        )
    }
}

/// Dig through a `syn::TypePath` (this is provided by the user in a type substitution definition as the "to" type) and
/// swap out any type params which match the idents given in the "from" type with the corresponding concrete types.
///
/// eg if we have:
///
/// ```text
/// from = sp_runtime::MultiAddress<A, B>,
/// to = ::subxt::utils::Static<::sp_runtime::MultiAddress<A, B>>
/// ```
///
/// And we encounter a `sp_runtime::MultiAddress<Foo, Bar>`, then we will pass the `::sp_runtime::MultiAddress<A, B>`
/// type param value into this call to turn it into `::sp_runtime::MultiAddress<Foo, Bar>`.
fn replace_path_params_recursively<I: Borrow<syn::Ident>, P: Borrow<TypePath>>(
    path: &mut syn::Path,
    params: &Vec<(I, P)>,
) {
    for segment in &mut path.segments {
        let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments else {
            continue;
        };
        for arg in &mut args.args {
            let syn::GenericArgument::Type(ty) = arg else {
                continue;
            };
            let syn::Type::Path(path) = ty else {
                continue;
            };
            if let Some(ident) = get_ident_from_type_path(path) {
                if let Some((_, replacement)) = params.iter().find(|(i, _)| ident == i.borrow()) {
                    *ty = replacement.borrow().to_syn_type();
                    continue;
                }
            }
            replace_path_params_recursively(&mut path.path, params);
        }
    }
}

/// Given a "to" type in a type substitution, return the TypePath inside or None if
/// it's not a valid "to" type.
fn get_valid_to_substitution_type(arg: &syn::GenericArgument) -> Option<&syn::TypePath> {
    let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg else {
        // We are looking for a type, not a lifetime or anything else
        return None;
    };
    Some(type_path)
}

/// Given a "from" type in a type substitution, return the Ident inside or None if
/// it's not a valid "from" type.
fn get_valid_from_substitution_type(arg: &syn::GenericArgument) -> Option<&syn::Ident> {
    let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg else {
        // We are looking for a type, not a lifetime or anything else
        return None;
    };
    get_ident_from_type_path(type_path)
}

/// Given a type path, return the single ident representing it if that's all it is.
fn get_ident_from_type_path(type_path: &syn::TypePath) -> Option<&syn::Ident> {
    if type_path.qself.is_some() {
        // No "<Foo as Bar>" type thing
        return None;
    }
    if type_path.path.leading_colon.is_some() {
        // No leading "::"
        return None;
    }
    if type_path.path.segments.len() > 1 {
        // The path should just be a single ident, not multiple
        return None;
    }
    let Some(segment) = type_path.path.segments.last() else {
        // Get the single ident (should be infallible)
        return None;
    };
    if !segment.arguments.is_empty() {
        // The ident shouldn't have any of it's own generic args like A<B, C>
        return None;
    }
    Some(&segment.ident)
}

/// Whether a path is absolute - starts with `::` or `crate`.
fn is_absolute(path: &syn::Path) -> bool {
    path.leading_colon.is_some()
        || path
            .segments
            .first()
            .map_or(false, |segment| segment.ident == "crate")
}

pub fn absolute_path(path: syn::Path) -> Result<AbsolutePath, TypeSubstitutionError> {
    path.try_into()
}

pub struct AbsolutePath(syn::Path);

impl TryFrom<syn::Path> for AbsolutePath {
    type Error = TypeSubstitutionError;
    fn try_from(value: syn::Path) -> Result<Self, Self::Error> {
        if is_absolute(&value) {
            Ok(AbsolutePath(value))
        } else {
            Err(TypeSubstitutionError::ExpectedAbsolutePath(value.span()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! syn_path {
        ($path:path) => {{
            let path: syn::Path = syn::parse_quote!($path);
            path
        }};
    }

    macro_rules! type_path {
        ($path:path) => {{
            let path: syn::Path = syn::parse_quote!($path);
            TypePath::from_syn_path(path)
        }};
    }

    fn ident(name: &'static str) -> syn::Ident {
        syn::Ident::new(name, proc_macro2::Span::call_site())
    }

    #[test]
    #[rustfmt::skip]
    fn replacing_nested_type_params_works() {
        // Original path, replacement ident->paths, expected output path
        let paths = [
            // Works ok if nothing to replace
            (
                syn_path!(::some::path::Foo<::other::Path<A, B>>),
                vec![],
                syn_path!(::some::path::Foo<::other::Path<A, B>>),
            ),
            // Simple top level replacing
            (
                syn_path!(::some::path::Foo<A>),
                vec![(ident("A"), type_path!(::new::Value))],
                syn_path!(::some::path::Foo<::new::Value>),
            ),
            // More deeply nested replacing works too
            (
                syn_path!(::some::path::Foo<::other::Path<A, B>>),
                vec![(ident("A"), type_path!(::new::Value))],
                syn_path!(::some::path::Foo<::other::Path<::new::Value, B>>),
            ),
            (
                syn_path!(::some::path::Foo<::other::Path<A, B>>),
                vec![
                    (ident("A"), type_path!(::new::A)),
                    (ident("B"), type_path!(::new::B)),
                ],
                syn_path!(::some::path::Foo<::other::Path<::new::A, ::new::B>>),
            ),
            (
                syn_path!(::some::path::Foo<::other::Path<A, ::more::path::to<::something::Argh<B>>>, C>),
                vec![
                    (ident("A"), type_path!(::new::A)),
                    (ident("B"), type_path!(::new::B)),
                ],
                syn_path!(::some::path::Foo<::other::Path<::new::A, ::more::path::to<::something::Argh<::new::B>>>,C>),
            ),
            // The same ident will be replaced as many times as needed:
            (
                syn_path!(::some::path::Foo<::other::Path<A, ::foo::Argh<A, B>, A>>),
                vec![(ident("A"), type_path!(::new::Value))],
                syn_path!(::some::path::Foo<::other::Path<::new::Value, ::foo::Argh<::new::Value, B>, ::new::Value>>),
            ),
        ];

        for (mut path, replacements, expected) in paths {
            replace_path_params_recursively(&mut path, &replacements);
            assert_eq!(path, expected);
        }
    }
}
