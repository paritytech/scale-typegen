
# [0.9.0] - 2024-10-22

- Bump `scale-value` to 0.17.0 ([#35](https://github.com/paritytech/scale-typegen/pull/35))
- Bump `scale-decode` to 0.14.0 ([#36](https://github.com/paritytech/scale-typegen/pull/36))
- Bump `scale-encode` to 0.8 ([#37](https://github.com/paritytech/scale-typegen/pull/37))
- Bump `frame-metadata` to 17 ([#38](https://github.com/paritytech/scale-typegen/pull/38))

# [0.8.0] - 2024-05-24

This release doesn't change the public API, but does effect the generated code to make it more correct and de-duplicate types better, but also a little less forgiving in certain edge cases. As such, I've decided to do a minor bump so that opting in to this change is explicit.

- Improve type equality checking used during de-duplication of types to deduplicate more reliably/often ([#30](https://github.com/paritytech/scale-typegen/pull/30))

# [0.7.0] - 2024-05-15

- Bump `scale-value` to 0.16.0, in order to have only one `scale-decode` in the hierarchy.

# [0.6.0] - 2024-05-15

- Bump `scale-decode` to 0.13.0.

# [0.5.0] - 2024-04-29

- Bump `scale-*` dependencies to latest to bring in `scale-type-resolver` 0.2.

# [0.4.3] - 2024-04-02

This release that adds support for the builtin types `BinaryHeap`, `LinkedList` and `VecDeque`.

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
