# scale-typegen

A library based on [scale-info](https://github.com/paritytech/scale-info) to transpile portable registries of types into rust type definitions.
This library exposes a `TypeGenerator` struct which wants to be given two things:

- a `scale_info::PortableRegistry` containing the type information for the types that will be transpiled.
- a `TypeGeneratorSettings` defining how the code generation happens: What derives and attributes that are applied, substitutes for types, if docs should be generated as well, (..).

It exposes a `generate_types_mod()` function that creates an intermediate representation of a module (`ModuleIR`) that contains types and other modules. It can be directly converted to rust code via `to_token_stream()`.
