use std::{cell::RefCell, collections::HashMap};

use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};

/// The transformer provides an abstraction for traversing a type registry
/// given a type_id as a starting point, and **transforming** it into a tree-like structure (type parameter `R`).
/// For example, `R` might be a TokenStream, a String or a Scale Value.
/// The transformer internally keeps a cache that shields users from infinite recursion.
/// It can also contain a mutable state (type parameter `S`), that can be used to store additional information.
/// This is useful for side effects, e.g. a random number generator for random type examples.
///
/// In this way, we can have easy recursion protection mechanisms for type descriptions, rust type examples and scale value type examples.
pub struct Transformer<'a, R, S = ()> {
    /// keep this private such that the cache is sealed and cannot be accessed from outside of the [`Transformer::transform`] function
    cache: RefCell<HashMap<u32, Cached<R>>>,
    /// state can be used for example for an Rng
    state: S,
    /// The `policy` defines, how to transform a type. If the type is unrepresentable, return an Err.
    policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
    /// The `recurse_policy` defines, how to handle cases, where a type has been
    /// visited before, and is visited *again*, before a representation of this type could be computed.
    /// It is up the implementation to return an error in these cases, or some other value.
    recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
    /// Describe the policy to apply when encountering a cache hit.
    /// A cache hit is, when the representation of a type has already been computed.
    ///
    /// In this case there are 2 options:
    /// - ReturnCached => return the cached value
    /// - ExecuteRecursePolicy => execute the recurse policy
    cache_hit_policy: CacheHitPolicy,
    registry: &'a PortableRegistry,
}

/// The transformer stores computed representations of types in a cache.
/// Sometimes we encounter types, where this representation is already computed.
///
/// The `CacheHitPolicy` defines, how to handle these cases.
#[derive(Debug, Clone, Copy)]
pub enum CacheHitPolicy {
    /// Returns the computed value from the cache.
    ReturnCached,
    /// Ignore the cached value and just compute the representation of the type again.
    ///
    /// Note: This is safe from a recursion standpoint.
    /// It is useful for generating multiple different type examples for one type instead of just returning the same one every time.
    ComputeAgain,
    /// Act like we were dealing with a recursive type. This will lead to the recurse policy being executed.
    /// It can for example be used to return a placeholder value, e.g. the type name, when a type is encountered for the second time.
    ExecuteRecursePolicy,
}

#[derive(Clone, Debug)]
enum Cached<Out> {
    /// not known yet, but computation has already started
    Recursive,
    /// computation was finished
    Computed(Out),
}

impl<'a, R, S> Transformer<'a, R, S>
where
    R: Clone + std::fmt::Debug,
{
    pub fn new(
        policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        cache_hit_policy: CacheHitPolicy,
        state: S,
        registry: &'a PortableRegistry,
    ) -> Self {
        Transformer {
            cache: RefCell::new(HashMap::new()),
            state,
            policy,
            recurse_policy,
            registry,
            cache_hit_policy,
        }
    }

    /// The custom user defined state of the transformer.
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn resolve(&self, type_id: u32) -> anyhow::Result<R> {
        let ty = self.registry.resolve(type_id).ok_or(anyhow::anyhow!(
            "Type with id {} not found in registry",
            type_id
        ))?;

        match self.cache.borrow().get(&type_id) {
            Some(Cached::Recursive) => {
                if !recursion_should_continue(&ty.type_def) {
                    return (self.recurse_policy)(type_id, ty, self);
                }
            }
            Some(Cached::Computed(repr)) => {
                match self.cache_hit_policy {
                    CacheHitPolicy::ReturnCached => return Ok(repr.clone()),
                    CacheHitPolicy::ExecuteRecursePolicy => {
                        if !recursion_should_continue(&ty.type_def) {
                            return (self.recurse_policy)(type_id, ty, self);
                        }
                    }
                    CacheHitPolicy::ComputeAgain => {
                        // ..continue with the computation
                    }
                };
            }
            _ => {}
        };

        self.cache.borrow_mut().insert(type_id, Cached::Recursive);
        let r = (self.policy)(type_id, ty, self)?;
        self.cache
            .borrow_mut()
            .insert(type_id, Cached::Computed(r.clone()));
        Ok(r)
    }
}

/// Returns true for types where recursion should continue, instead of being stopped when recursion in being detected.
///
/// ## Background:
///
/// There is a problem in generating recursive type descriptions:
/// Suppose we have the following setup:
/// ```rust
/// struct A {
///     bees: Vec<B>
/// }
///
/// struct B {
///     id: u8,
///     others: Vec<B>
/// }
/// ```
/// This could be described as:
/// ```txt,no_run
/// struct A {
///     bees: Vec<struct B {
///         id: u8,
///         others: Vec<B>
///     }>
/// }
/// ```
/// But the recursive resolving would get stuck in the middle, reporting recursion.
/// This is because Vec<B> needs to be mapped to different strings, so the simple cache lookup is not viable.
/// The solution to this is, to just let some container types like Vec do recursion while others can't.
///
/// # Warning
///
/// The safety of the following logic relies on the assumption that ultimately everything resolves down to a primitive or a struct/enum that is in the cache.
/// It basically just returns true for generic wrapper types.
fn recursion_should_continue(def: &TypeDef<PortableForm>) -> bool {
    match def {
        scale_info::TypeDef::Sequence(_) => true,
        scale_info::TypeDef::Array(_) => true,
        scale_info::TypeDef::Tuple(_) => true,
        scale_info::TypeDef::Compact(_) => true,
        scale_info::TypeDef::Composite(_) => false,
        scale_info::TypeDef::Primitive(_) => false,
        scale_info::TypeDef::Variant(_) => false,
        scale_info::TypeDef::BitSequence(_) => false,
    }
}
