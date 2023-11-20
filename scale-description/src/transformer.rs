use std::{cell::RefCell, collections::HashMap};

use scale_info::{form::PortableForm, PortableRegistry, Type};

/// The transformer provides an abstraction for traversing a type registry
/// given a type_id as a starting point, and **transforming** it into a tree-like structure.
/// It provides a cache that shields users from infinite recursion.
///
/// In this way, we can have easy recursion protection mechanisms for type descirptions, rust type examples and scale value type examples.
pub struct Transformer<'a, R, S = ()> {
    /// keep this private such that the cache is sealed and connot be accessed from outside of the Transformer::transform function
    cache: RefCell<HashMap<u32, Cached<R>>>,
    /// state can be used for example for an Rng
    pub state: S,
    policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
    recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
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
    R: Clone,
{
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn new(
        policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        resurse_policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        state: S,
        registry: &'a PortableRegistry,
    ) -> Self {
        Transformer {
            cache: RefCell::new(HashMap::new()),
            state,
            policy,
            recurse_policy: resurse_policy,
            registry,
        }
    }

    pub fn resolve(&self, type_id: u32) -> anyhow::Result<R> {
        let ty = self.registry.resolve(type_id).ok_or(anyhow::anyhow!(
            "Type with id {} not found in registry",
            type_id
        ))?;

        match self.cache.borrow().get(&type_id) {
            Some(Cached::Recursive) => {
                return (self.recurse_policy)(type_id, ty, self);
            }
            Some(Cached::Computed(r)) => return Ok(r.clone()),
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
