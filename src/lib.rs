pub mod typegen;

pub use typegen::{
    settings::{
        derives::{Derives, DerivesRegistry},
        substitutes::TypeSubstitutes,
        TypeGeneratorSettings,
    },
    type_path_resolver::TypePathResolver,
    TypeGenerationError, TypeGenerator,
};

#[cfg(test)]
mod tests;
