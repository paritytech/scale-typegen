use std::{cell::RefCell, collections::HashMap};

use scale_info::{form::PortableForm, PortableRegistry, Type};

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
    ///
    /// You can return None to sidestep recursion protection and let the transformer continue.
    recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> Option<anyhow::Result<R>>,
    /// Describe the policy to apply when encountering a cache hit.
    /// A cache hit is, when the representation of a type has already been computed.
    ///
    /// You can return None to sidestep recursion protection and let the transformer continue.
    #[allow(clippy::type_complexity)]
    cache_hit_policy: fn(u32, &Type<PortableForm>, &R, &Self) -> Option<anyhow::Result<R>>,
    registry: &'a PortableRegistry,
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
    /// Create a new transformer.
    #[allow(clippy::type_complexity)]
    pub fn new(
        policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> Option<anyhow::Result<R>>,
        cache_hit_policy: fn(u32, &Type<PortableForm>, &R, &Self) -> Option<anyhow::Result<R>>,
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

        if let Some(cache_value) = self.cache.borrow().get(&type_id) {
            let result_or_continue = match cache_value {
                Cached::Recursive => (self.recurse_policy)(type_id, ty, self),
                Cached::Computed(repr) => (self.cache_hit_policy)(type_id, ty, repr, self),
            };
            if let Some(result) = result_or_continue {
                return result;
            }
        };
        self.cache.borrow_mut().insert(type_id, Cached::Recursive);
        let r = (self.policy)(type_id, ty, self)?;
        self.cache
            .borrow_mut()
            .insert(type_id, Cached::Computed(r.clone()));
        Ok(r)
    }
}
