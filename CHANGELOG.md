# [0.4.2] - 2024-03-26

- Revert some code simplification in 0.4.1 so that older metadata (built with scale-info versions prior to 2.11.1 and using the `retain()` method) is more likely to work.

# [0.4.1] - 2024-03-22

- Bump `scale-info` to 2.11.1 to fix an issue in `scale_typegen::utils::ensure_unique_type_paths` for cases where a type's index and id do not line up, and simplify code a touch.

# [0.4.0] - 2024-03-21

- Fix logic bug in `scale_typegen::utils::ensure_unique_type_paths`.

- You can now specify a custom path to the `alloc` crate in the settings. It is used for type paths that are typically in the rust prelude or in the `std` library, like `Vec`, `Box` and `String`. This gives you the option to use the generated code in a `no_std` environment.

# [0.3.0] - 2024-03-19

- When generating rust code with `TypeGenerator::gerate_types_mod` we now validate that no type
is overwritten by another type that has an identical type path but a different structure. In case this happens,
we return an error and encourage users to use `scale_typegen::utils::ensure_unique_type_paths` on
the type registry before. Doing so, should let the type generation succeed.

# [0.2.0] - 2024-03-01

- bumped dependencies of `scale-*` crates.

# [0.1.2] - 2024-02-29 [YANKED]

- dependency updates broke usage with subxt -> released 0.2.0 instead.

# [0.1.1] - 2024-01-05

### Fixed

- A bug regarding type substitutes with generic parameters.

# [0.1.0] - 2023-12-08

This is the first release of `scale-typegen` and `scale-typegen-description`.
