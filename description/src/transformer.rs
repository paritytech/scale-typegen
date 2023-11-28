use std::{cell::RefCell, collections::HashMap};

use scale_info::{form::PortableForm, PortableRegistry, Type, TypeDef};

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
    /// The `policy` defines, how to transform a type. If the type is unrepresentable, return an Err.
    policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
    /// The `recurse_policy` defines, how to handle cases,
    /// where a type has been visited before, and is visited again BEFORE a representation of this type could be computed.
    /// If it returns Some(..) we avoid infinite recursion by returning a concrete value.
    /// If it returns None, nothing is done in the case of detected recursion and the type falls through.
    /// It is transformed with the `policy` in this case. This can be dangerous, but may be necessary for some container types.
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
    R: Clone + std::fmt::Debug,
{
    pub fn state(&self) -> &S {
        &self.state
    }
    pub fn new(
        policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        recurse_policy: fn(u32, &Type<PortableForm>, &Self) -> anyhow::Result<R>,
        state: S,
        registry: &'a PortableRegistry,
    ) -> Self {
        Transformer {
            cache: RefCell::new(HashMap::new()),
            state,
            policy,
            recurse_policy,
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
                dbg!(self.cache.borrow());
                if !recursion_should_fall_through(&ty.type_def) {
                    return (self.recurse_policy)(type_id, ty, self);
                }
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
/// The safety of the following logic relies on the assumption that ultimately everything resolves down to a primitive or a struct/enum that is in the cache.
/// It basically just returns true o generic wrapper types.
fn recursion_should_fall_through(def: &TypeDef<PortableForm>) -> bool {
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
