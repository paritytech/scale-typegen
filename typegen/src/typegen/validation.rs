use scale_info::{form::PortableForm, PortableRegistry};
use syn::parse_quote;

use crate::{DerivesRegistry, TypeSubstitutes};

use super::{
    error::SettingsValidationError,
    settings::substitutes::{path_segments, PathSegments, TryIntoSynPath},
};

/// Validates that the settings given are valid for the type registry.
/// It checks for all type specific derives, attributes and substitutes if these types really exist in the type registry.
/// It is thus overly conservative.
///
/// Incrementally builds up an error that contains all the bits and pieces that make the settings invalid.
pub fn validate_substitutes_and_derives_against_registry(
    substitutes: &TypeSubstitutes,
    derives: &DerivesRegistry,
    types: &PortableRegistry,
) -> Result<(), SettingsValidationError> {
    let mut error = SettingsValidationError::default();

    for (path, derives_and_attrs) in derives.derives_on_specific_types() {
        let path_segments = path_segments(&path.path);
        if !registry_contains_type_path(types, &path_segments) {
            let attributes = derives_and_attrs.attributes();
            let derives = derives_and_attrs.derives();

            if !attributes.is_empty() {
                let already_in_err = error
                    .attributes_for_unknown_types
                    .iter_mut()
                    .find(|(e, _)| e == &path.path);

                match already_in_err {
                    Some(e) => e.1.extend(attributes.iter().cloned()),
                    None => error
                        .attributes_for_unknown_types
                        .push((path.path.clone(), attributes.clone())),
                }
            }

            if !derives.is_empty() {
                let already_in_err = error
                    .derives_for_unknown_types
                    .iter_mut()
                    .find(|(e, _)| e == &path.path);

                match already_in_err {
                    Some(e) => e.1.extend(derives.iter().cloned()),
                    None => error
                        .derives_for_unknown_types
                        .push((path.path.clone(), derives.clone())),
                }
            }
        }
    }

    for (path, sub) in substitutes.iter() {
        if !registry_contains_type_path(types, path) {
            error
                .substitutes_for_unknown_types
                .push((path_segments_to_syn_path(path), sub.path().clone()))
        }
    }

    if error.is_empty() {
        Ok(())
    } else {
        Err(error)
    }
}

/// Converts a `Vec<String>` into a [`syn::Path`]. Returns None if the Vec was empty.
///
/// # Panics
///
/// Panics if the segments are empty or contain strings that are not valid [`syn::path::PathSegment`]s.
fn path_segments_to_syn_path(segments: &PathSegments) -> syn::Path {
    if segments.is_empty() {
        panic!("Path in Substitutes should not be empty.")
    }
    let segments = segments.iter().map(|e| {
        syn::parse_str::<syn::PathSegment>(e)
            .expect("PathSegments should be syn::PathSegment compatible")
    });
    parse_quote!(#(#segments)::*)
}

/// Checks if a given type path is the type path of a type in the registry.
pub fn registry_contains_type_path(types: &PortableRegistry, path: &[String]) -> bool {
    types.types.iter().any(|t| t.ty.path.segments == *path)
}

/// Returns types in the PortableRegistry that share the identifier with the input path.
pub fn similar_type_paths_in_registry(
    types: &PortableRegistry,
    path: &syn::Path,
) -> Vec<syn::Path> {
    let scale_type_path = scale_info::Path::<PortableForm> {
        segments: path.segments.iter().map(|e| e.ident.to_string()).collect(),
    };
    if scale_type_path.ident().is_none() {
        return vec![];
    }

    // Find types that have a path with the same identifier, then convert these paths into `syn::Path`s.
    types
        .types
        .iter()
        .filter_map(|t| {
            let path = &t.ty.path;
            path.ident()
                .filter(|ident| ident == scale_type_path.ident().as_ref().expect("qed"))?;
            path.syn_path()
        })
        .collect()
}
