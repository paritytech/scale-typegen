use std::{cell::RefCell, collections::HashMap};

use scale_info::{form::PortableForm, PortableRegistry, Type};

#[derive(Clone, Debug)]
enum Cached<Out> {
    /// not known yet, but computation has already started
    Recursive,
    /// computation was finished
    Computed(Out),
}

pub struct Transformer<'a, R, S = ()> {
    /// keep this private such that the cache is sealed and connot be accessed from outside of the Transformer::transform function
    cache: RefCell<HashMap<u32, Cached<R>>>,
    /// state can be used for example for an Rng
    state: S,
    policy: fn(&Type<PortableForm>, &Self) -> anyhow::Result<R>,
    resurse_policy: fn(&Type<PortableForm>) -> anyhow::Result<R>,
    registry: &'a PortableRegistry,
}

impl<'a, R, S> Transformer<'a, R, S>
where
    R: Clone,
{
    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn new(
        policy: fn(&Type<PortableForm>, &Self) -> anyhow::Result<R>,
        resurse_policy: fn(&Type<PortableForm>) -> anyhow::Result<R>,
        state: S,
        registry: &'a PortableRegistry,
    ) -> Self {
        Transformer {
            cache: RefCell::new(HashMap::new()),
            state,
            policy,
            resurse_policy,
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
                return (self.resurse_policy)(ty);
            }
            Some(Cached::Computed(r)) => return Ok(r.clone()),
            _ => {}
        };

        self.cache.borrow_mut().insert(type_id, Cached::Recursive);
        let r = (self.policy)(ty, self)?;
        self.cache
            .borrow_mut()
            .insert(type_id, Cached::Computed(r.clone()));
        Ok(r)
    }
}
