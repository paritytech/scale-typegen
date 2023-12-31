// Copyright 2019-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A library based on [scale-info](https://github.com/paritytech/scale-info) to transpile portable registries of types into rust type definitions.
#![deny(missing_docs)]

/// Type Generation Settings and Logic
pub mod typegen;
/// Utilities for handling Type Registries
pub mod utils;

pub use typegen::{
    error::TypegenError,
    settings::{derives::DerivesRegistry, substitutes::TypeSubstitutes, TypeGeneratorSettings},
    TypeGenerator,
};

#[cfg(test)]
mod tests;
