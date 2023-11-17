pub mod rust_type;
pub mod scale_value;

pub use rust_type::example_rust_type;
pub use scale_value::example_scale_value;

#[cfg(test)]
mod tests {
    use scale_info::{PortableRegistry, TypeInfo};

    fn make_type<T: TypeInfo + 'static>() -> (u32, PortableRegistry) {
        let mut registry = scale_info::Registry::new();
        let m = scale_info::MetaType::new::<T>();
        let ty = registry.register_type(&m);
        (ty.id, registry.into())
    }

    fn test_example() {

        struct 


        make_type


    }
}
