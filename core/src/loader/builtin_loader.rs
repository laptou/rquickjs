use crate::{loader::Loader, module::ModuleData, Ctx, Error, Result};
use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
use std::collections::HashMap as Map;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as Map;

/// The builtin script module loader
///
/// This loader can be used as the nested backing loader in user-defined loaders.
#[derive(Debug, Default)]
pub struct BuiltinLoader {
    modules: Map<String, Vec<u8>>,
}

impl BuiltinLoader {
    /// Add builtin script module
    pub fn add_module<N: Into<String>, S: Into<Vec<u8>>>(
        &mut self,
        name: N,
        source: S,
    ) -> &mut Self {
        self.modules.insert(name.into(), source.into());
        self
    }

    /// Add builtin script module
    #[must_use]
    pub fn with_module<N: Into<String>, S: Into<Vec<u8>>>(mut self, name: N, source: S) -> Self {
        self.add_module(name, source);
        self
    }
}

impl Loader for BuiltinLoader {
    fn load<'js>(&mut self, _ctx: &Ctx<'js>, path: &str) -> Result<ModuleData> {
        match self.modules.remove(path) {
            Some(source) => Ok(ModuleData::source(path, source)),
            _ => Err(Error::new_loading(path)),
        }
    }
}
