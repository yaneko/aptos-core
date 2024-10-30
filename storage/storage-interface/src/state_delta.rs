// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::HashValue;
use aptos_experimental_layered_map::{LayeredMap, MapLayer};
use aptos_types::{
    state_store::{
        state_key::StateKey, state_storage_usage::StateStorageUsage, state_value::StateValue,
    },
    transaction::Version,
    write_set::{TransactionWrite, WriteSet},
};
use derive_more::Deref;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct StateUpdate {
    pub version: Version,
    pub value: Option<StateValue>,
}

impl StateUpdate {
    pub fn new(version: Version, value: Option<StateValue>) -> Self {
        Self { version, value }
    }
}

#[derive(Clone, Debug)]
pub struct InMemState {
    pub next_version: Version,
    pub updates: MapLayer<StateKey, StateUpdate>,
    pub usage: StateStorageUsage,
}

impl InMemState {
    pub fn new_empty() -> Self {
        // FIXME(aldenh): check call site and implement
        todo!()
    }

    pub fn next_version(&self) -> Version {
        self.next_version
    }

    pub fn into_delta(self, base: InMemState) -> StateDelta {
        StateDelta::new(
            base,
            self,
        )
    }
}

/// Represents all state updates in the (base, current] range
#[derive(Clone, Debug, Deref)]
pub struct StateDelta {
    // exclusive
    pub base: InMemState,
    pub current: InMemState,
    #[deref]
    pub updates: LayeredMap<StateKey, StateUpdate>,
}

impl StateDelta {
    pub fn new(base: InMemState, current: InMemState) -> Self {
        let updates = current.updates.view_layers_after(&base.updates);
        Self {
            base,
            current,
            updates,
        }
    }

    pub fn new_empty() -> Self {
        /* FIXME(aldenhu):
        let smt = SparseMerkleTree::new_empty();
        Self::new(smt.clone(), None, smt, None, HashMap::new())
         */
        todo!()
    }

    pub fn new_at_checkpoint(
        _root_hash: HashValue,
        _usage: StateStorageUsage,
        _checkpoint_version: Option<Version>,
    ) -> Self {
        /* FIXME(aldenhu):
        let smt = SparseMerkleTree::new(root_hash, usage);
        Self::new(
            smt.clone(),
            checkpoint_version,
            smt,
            checkpoint_version,
            HashMap::new(),
        )

         */
        todo!()
    }

    pub fn merge(&mut self, _other: StateDelta) {
        /* FIXME(aldenhu):
        assert!(other.follow(self));
        self.updates_since_base
            .extend(other.updates_since_base.deref_mut().drain());

        self.current = other.current;
        self.current_version = other.current_version;
         */
        todo!()
    }

    pub fn has_same_current_state(&self, _other: &StateDelta) -> bool {
        /* FIXME(aldenhu):
        self.current_version == other.current_version
            && self.current.has_same_root_hash(&other.current)
         */
        todo!()
    }

    pub fn next_version(&self) -> Version {
        self.current.next_version()
    }

    pub fn base_version(&self) -> Option<Version> {
        self.base.next_version.checked_sub(1)
    }

    pub fn replace_with(&mut self, mut _rhs: Self) -> Self {
        /* FIXME(aldenhu):
        std::mem::swap(self, &mut rhs);
        rhs
         */
        todo!()
    }

    pub fn update<'a>(&self, write_sets: impl IntoIterator<Item = &'a WriteSet>) -> InMemState {
        let mut next_version = self.next_version();
        let kvs = write_sets
            .into_iter()
            .flat_map(|write_set| {
                write_set.iter().map(move |(state_key, write_op)| {
                    let version = next_version;
                    next_version += 1;
                    (
                        state_key.clone(),
                        StateUpdate::new(version, write_op.as_state_value()),
                    )
                })
            })
            .collect_vec();
        let updates = self.updates.new_layer(&kvs);
        let usage = Self::caculate_usage(self.current.usage, &kvs);

        InMemState {
            next_version,
            updates,
            usage,
        }
    }

    fn caculate_usage(
        _base_usage: StateStorageUsage,
        _updates: &[(StateKey, StateUpdate)],
    ) -> StateStorageUsage {
        // FIXME(aldenhu)
        todo!()
    }
}

impl Default for StateDelta {
    fn default() -> Self {
        Self::new_empty()
    }
}
