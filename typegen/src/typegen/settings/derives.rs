use std::collections::{HashMap, HashSet};

use quote::ToTokens;
use scale_info::{form::PortableForm, PortableRegistry, Type};

use crate::{utils::syn_type_path, TypegenError};

/// A struct containing the derives that we'll be applying to types;
/// a combination of some common derives for all types, plus type
/// specific derives.
#[derive(Debug, Clone, Default)]
pub struct DerivesRegistry {
    default_derives: Derives,
    specific_type_derives: HashMap<syn::TypePath, Derives>,
    recursive_type_derives: HashMap<syn::TypePath, Derives>,
}

impl DerivesRegistry {
    /// Creates a new `DerivesRegistry` with no default derives.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert derives to be applied to all generated types.
    pub fn extend_for_all(
        &mut self,
        derives: impl IntoIterator<Item = syn::Path>,
        attributes: impl IntoIterator<Item = syn::Attribute>,
    ) {
        self.default_derives.derives.extend(derives);
        self.default_derives.attributes.extend(attributes);
    }

    /// Insert derives to be applied to a specific generated type.
    ///
    /// The `recursive` flag can be set if child types should also receive the given derives/attributes.
    /// Child types are all types that are mentioned as fields or type parameters of the type.
    pub fn extend_for_type(
        &mut self,
        ty: syn::TypePath,
        derives: impl IntoIterator<Item = syn::Path>,
        attributes: impl IntoIterator<Item = syn::Attribute>,
        recursive: bool,
    ) {
        let type_derives = if recursive {
            self.recursive_type_derives.entry(ty).or_default()
        } else {
            self.specific_type_derives.entry(ty).or_default()
        };
        type_derives.derives.extend(derives);
        type_derives.attributes.extend(attributes);
    }

    /// Returns the derives to be applied to all generated types.
    pub fn default_derives(&self) -> &Derives {
        &self.default_derives
    }

    /// Flattens out the recursive derives into specific derives.
    /// For this it needs to have a PortableRegistry that it can traverse recursively.
    pub fn flatten_recursive_derives(
        self,
        types: &PortableRegistry,
    ) -> Result<FlatDerivesRegistry, TypegenError> {
        let DerivesRegistry {
            default_derives,
            mut specific_type_derives,
            mut recursive_type_derives,
        } = self;

        if recursive_type_derives.is_empty() {
            return Ok(FlatDerivesRegistry {
                default_derives,
                specific_type_derives,
            });
        }

        // Build a mapping of type ids to syn paths for all types in the registry:
        let mut syn_path_for_id: HashMap<u32, syn::TypePath> = types
            .types
            .iter()
            .filter_map(|t| {
                if t.ty.path.is_empty() {
                    None
                } else {
                    match syn_type_path(&t.ty) {
                        Ok(path) => Some(Ok((t.id, path))),
                        Err(err) => Some(Err(err)),
                    }
                }
            })
            .collect::<Result<_, TypegenError>>()?;

        // Create an empty map of derives that we are about to fill:
        let mut add_derives_for_id: HashMap<u32, Derives> = HashMap::new();

        // Check for each type in the registry if it is the top level of
        for ty in types.types.iter() {
            let Some(path) = syn_path_for_id.get(&ty.id) else {
                // this is only the case for types with empty path (i.e. builtin types).
                continue;
            };
            let Some(recursive_derives) = recursive_type_derives.remove(path) else {
                continue;
            };
            // The collected_type_ids contain the id of the type itself and all ids of its fields:
            let mut collected_type_ids: HashSet<u32> = HashSet::new();
            collect_type_ids(ty.id, types, &mut collected_type_ids);

            // We collect the derives for each type id in the add_derives_for_id HashMap.
            for id in collected_type_ids {
                add_derives_for_id
                    .entry(id)
                    .or_default()
                    .extend_from(recursive_derives.clone());
            }
        }

        // Merge all the recursively obtained derives with the existing derives for the types.
        for (id, derived_to_add) in add_derives_for_id {
            if let Some(path) = syn_path_for_id.remove(&id) {
                specific_type_derives
                    .entry(path)
                    .or_default()
                    .extend_from(derived_to_add);
            }
        }

        Ok(FlatDerivesRegistry {
            default_derives,
            specific_type_derives,
        })
    }

    /// An iterator over (syn::TypePath, Derives) pairs for specific types (also recursive ones).
    pub fn derives_on_specific_types(&self) -> impl Iterator<Item = (&syn::TypePath, &Derives)> {
        self.specific_type_derives
            .iter()
            .chain(self.recursive_type_derives.iter())
    }
}

/// A struct storing the set of derives and derive attributes that we'll apply
/// to generated types.
#[derive(Debug, Clone, Default)]
pub struct Derives {
    derives: HashSet<syn::Path>,
    attributes: HashSet<syn::Attribute>,
}

impl FromIterator<syn::Path> for Derives {
    fn from_iter<T: IntoIterator<Item = syn::Path>>(iter: T) -> Self {
        let derives = iter.into_iter().collect();
        Self {
            derives,
            attributes: HashSet::new(),
        }
    }
}

impl Derives {
    /// Creates an empty instance of `Derives` (with no default derives).
    pub fn new() -> Self {
        Self {
            derives: HashSet::new(),
            attributes: HashSet::new(),
        }
    }

    /// Extend this set of `Derives` from another.
    pub fn extend_from(&mut self, other: Derives) {
        self.derives.extend(other.derives);
        self.attributes.extend(other.attributes);
    }

    /// Extend the set of derives by providing an iterator of paths to derive macros.
    pub fn extend(&mut self, derives: impl Iterator<Item = syn::Path>) {
        for derive in derives {
            self.insert_derive(derive)
        }
    }

    /// Insert a single derive.
    pub fn insert_derive(&mut self, derive: syn::Path) {
        self.derives.insert(derive);
    }

    /// Insert a single attribute to be applied to types.
    pub fn insert_attribute(&mut self, attribute: syn::Attribute) {
        self.attributes.insert(attribute);
    }

    /// Getter for the derived traits
    pub fn derives(&self) -> &HashSet<syn::Path> {
        &self.derives
    }

    /// Getter for the added attributes
    pub fn attributes(&self) -> &HashSet<syn::Attribute> {
        &self.attributes
    }
}

impl ToTokens for Derives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.derives.is_empty() {
            let mut sorted = self.derives.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #[derive(#( #sorted ),*)]
            })
        }
        if !self.attributes.is_empty() {
            let mut sorted = self.attributes.iter().cloned().collect::<Vec<_>>();
            sorted.sort_by(|a, b| {
                quote::quote!(#a)
                    .to_string()
                    .cmp(&quote::quote!(#b).to_string())
            });

            tokens.extend(quote::quote! {
                #( #sorted )*
            })
        }
    }
}

/// This is like a DerivesRegistry, but the recursive type derives have been flattened out into specific_type_derives.
///
/// Can be constructed properly using a DerivesRegistry and a PortableRegistry with `DerivesRegistry::flatten_recursive_derives()`.
#[derive(Debug, Clone, Default)]
pub struct FlatDerivesRegistry {
    default_derives: Derives,
    specific_type_derives: HashMap<syn::TypePath, Derives>,
}

impl FlatDerivesRegistry {
    /// Resolve the derives for a specific type path.
    pub fn resolve(&self, ty: &syn::TypePath) -> Derives {
        let mut resolved_derives = self.default_derives.clone();
        if let Some(specific) = self.specific_type_derives.get(ty) {
            resolved_derives.extend_from(specific.clone());
        }
        resolved_derives
    }

    /// Resolve the derives for a specific type.
    pub fn resolve_derives_for_type(
        &self,
        ty: &Type<PortableForm>,
    ) -> Result<Derives, TypegenError> {
        Ok(self.resolve(&syn_type_path(ty)?))
    }
}

fn collect_type_ids(id: u32, types: &PortableRegistry, collected_types: &mut HashSet<u32>) {
    // Recursion protection:
    if collected_types.contains(&id) {
        return;
    }

    // Add the type id itself as well:
    collected_types.insert(id);
    let ty = types
        .resolve(id)
        .expect("Should contain this id, if Registry not corrupted");

    // Collect the types that are passed as type params (Question/Note: Is this necessary? Maybe not...)
    for param in ty.type_params.iter() {
        if let Some(id) = param.ty.map(|e| e.id) {
            collect_type_ids(id, types, collected_types);
        }
    }

    // Collect ids depending on the types structure:
    match &ty.type_def {
        scale_info::TypeDef::Composite(def) => {
            for f in def.fields.iter() {
                collect_type_ids(f.ty.id, types, collected_types);
            }
        }
        scale_info::TypeDef::Variant(def) => {
            for v in def.variants.iter() {
                for f in v.fields.iter() {
                    collect_type_ids(f.ty.id, types, collected_types);
                }
            }
        }
        scale_info::TypeDef::Sequence(def) => {
            collect_type_ids(def.type_param.id, types, collected_types);
        }
        scale_info::TypeDef::Array(def) => {
            collect_type_ids(def.type_param.id, types, collected_types);
        }
        scale_info::TypeDef::Tuple(def) => {
            for f in def.fields.iter() {
                collect_type_ids(f.id, types, collected_types);
            }
        }
        scale_info::TypeDef::Primitive(_) => {}
        scale_info::TypeDef::Compact(def) => {
            collect_type_ids(def.type_param.id, types, collected_types);
        }
        scale_info::TypeDef::BitSequence(_) => {}
    }
}
