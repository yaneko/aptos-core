// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_scratchpad::SparseMerkleTree;
use aptos_types::state_store::state_key::StateKey;
use aptos_types::state_store::state_value::StateValue;
use crate::state_delta::{StateDelta, StateUpdate};

/// note: only a single field for now, more to be introduced later.
#[derive(Clone, Debug)]
pub struct StateAuthenticator {
    pub global_state: SparseMerkleTree<StateValue>,
}

impl StateAuthenticator {
    pub fn new(global_state: SparseMerkleTree<StateValue>) -> Self {
        Self { global_state }
    }

    pub fn update(
        &self,
        _persisted_auth: &StateAuthenticator,
        _state_delta: &StateDelta,
    ) -> Self {
        /// FIXME(aldenhu)
        todo!()
    }

}
