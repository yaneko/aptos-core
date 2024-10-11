// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_types::vm::{modules::ModuleCacheEntry, scripts::ScriptCacheEntry};
use hashbrown::HashMap;
use move_binary_format::errors::VMResult;
use move_core_types::language_storage::ModuleId;
use std::{cell::RefCell, sync::Arc};

/// A per-block code cache to be used for sequential transaction execution. Modules and scripts
/// can be cached and retrieved. It is responsibility of the caller to cache the base (i.e.,
/// storage version) modules.
pub struct UnsyncCodeCache {
    /// Script cache, indexed by script hashes.
    script_cache: RefCell<HashMap<[u8; 32], ScriptCacheEntry>>,
    /// Module cache, indexed by module address-name pair.
    module_cache: RefCell<HashMap<ModuleId, Arc<ModuleCacheEntry>>>,
}

impl UnsyncCodeCache {
    /// Returns an empty code cache.
    pub(crate) fn empty() -> Self {
        Self {
            script_cache: RefCell::new(HashMap::new()),
            module_cache: RefCell::new(HashMap::new()),
        }
    }

    /// Returns the number of modules cached in the code cache.
    pub(crate) fn num_modules(&self) -> usize {
        self.module_cache.borrow().len()
    }

    /// Stores the module to the code cache.
    pub fn store_module(&self, module_id: ModuleId, entry: ModuleCacheEntry) {
        self.module_cache
            .borrow_mut()
            .insert(module_id, Arc::new(entry));
    }

    /// Fetches the module from the code cache, if it exists there. If not, uses the provided
    /// initialization function to initialize and cache it.
    pub fn fetch_or_initialize_module<F>(
        &self,
        module_id: &ModuleId,
        init_func: &F,
    ) -> VMResult<Option<Arc<ModuleCacheEntry>>>
    where
        F: Fn(&ModuleId) -> VMResult<Option<ModuleCacheEntry>>,
    {
        if let Some(e) = self.module_cache.borrow().get(module_id) {
            return Ok(Some(e.clone()));
        }

        Ok(match init_func(module_id)? {
            Some(v) => {
                let e = Arc::new(v);
                self.module_cache
                    .borrow_mut()
                    .insert(module_id.clone(), e.clone());
                Some(e)
            },
            None => None,
        })
    }

    /// Stores the script to the code cache.
    pub fn store_script(&self, hash: [u8; 32], entry: ScriptCacheEntry) {
        self.script_cache.borrow_mut().insert(hash, entry);
    }

    /// Returns the script if it has been cached before, or [None] otherwise.
    pub fn fetch_script(&self, hash: &[u8; 32]) -> Option<ScriptCacheEntry> {
        self.script_cache.borrow().get(hash).cloned()
    }

    /// Collects the verified modules that were published and loaded during this block. Should only
    /// be called at the block end.
    pub fn collect_verified_entries_into<F, V>(&self, collector: &mut HashMap<ModuleId, V>, f: F)
    where
        F: Fn(&ModuleCacheEntry) -> V,
    {
        for (id, entry) in self
            .module_cache
            .borrow()
            .iter()
            .filter(|(_, e)| e.is_verified())
        {
            collector.insert(id.clone(), f(entry));
        }
    }
}
