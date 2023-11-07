use std::collections::BTreeSet;

use super::{type_ir::CompositeIRKind, type_path::TypeParameter};

pub struct UnusedTypeParams {
    unused: BTreeSet<TypeParameter>,
}

impl UnusedTypeParams {
    pub fn into_iter(self) -> impl Iterator<Item = TypeParameter> {
        self.unused.into_iter()
    }

    pub fn new(all_params: &[TypeParameter]) -> Self {
        let unused = all_params.iter().cloned().collect();
        Self { unused }
    }

    pub fn remove(&mut self, param: &TypeParameter) {
        self.unused.remove(param);
    }
}
