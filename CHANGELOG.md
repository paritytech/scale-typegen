# [0.3.1] - 2024-03-21

- Fix logic bug in `scale_typegen::utils::ensure_unique_type_paths`.

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
