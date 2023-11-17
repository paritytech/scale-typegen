use std::collections::HashMap;

use scale_info::{form::PortableForm, PortableRegistry, Type};

#[derive(Clone, Debug)]
enum Cached<Out> {
    /// not known yet, but computation has already started
    Recursive,
    /// computation was finished
    Computed(Out),
}

struct Transformer<R> {
    /// keep this private such that the cache is sealed and connot be accessed from outside of the Transformer::transform function
    __cache: HashMap<u32, Cached<R>>,
    policy: fn(&Type<PortableForm>, &PortableRegistry, &mut Self) -> anyhow::Result<R>,
    resurse_policy: fn(&Type<PortableForm>) -> anyhow::Result<R>,
}

impl<R> Transformer<R>
where
    R: Clone,
{
    fn new(
        policy: fn(&Type<PortableForm>, &PortableRegistry, &mut Self) -> anyhow::Result<R>,
        resurse_policy: fn(&Type<PortableForm>) -> anyhow::Result<R>,
    ) -> Self {
        Transformer {
            __cache: HashMap::new(),
            policy,
            resurse_policy,
        }
    }

    fn transform<P, RP>(&mut self, type_id: u32, registry: &PortableRegistry) -> anyhow::Result<R> {
        let ty = registry.resolve(type_id).ok_or(anyhow::anyhow!(
            "Type with id {} not found in registry",
            type_id
        ))?;

        match self.__cache.get(&type_id) {
            Some(Cached::Recursive) => {
                return (self.resurse_policy)(ty);
            }
            Some(Cached::Computed(r)) => return Ok(r.clone()),
            _ => {}
        };

        self.__cache.insert(type_id, Cached::Recursive);

        let r = (self.policy)(ty, registry, self);
        r
    }
}
