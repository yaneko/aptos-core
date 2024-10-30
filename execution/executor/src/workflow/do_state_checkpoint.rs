// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use anyhow::{ensure, Result};
use aptos_crypto::HashValue;
use aptos_executor_types::{
    execution_output::ExecutionOutput, state_checkpoint_output::StateCheckpointOutput,
};
use aptos_storage_interface::state_authenticator::StateAuthenticator;
use crate::types::in_memory_state_calculator_v2::InMemoryStateCalculatorV2;

pub struct DoStateCheckpoint;

impl DoStateCheckpoint {
    pub fn run(
        execution_output: &ExecutionOutput,
        parent_auth: &StateAuthenticator,
        persisted_auth: &StateAuthenticator,
        known_state_checkpoints: Option<impl IntoIterator<Item = Option<HashValue>>>,
    ) -> Result<StateCheckpointOutput> {
        let last_checkpoint_auth: Option<StateAuthenticator>;
        let state_auth: StateAuthenticator;

        if let Some(last_checkpoint_state) = execution_output.last_checkpoint_state.as_ref() {
            let before_checkpoint = last_checkpoint_state.clone().into_delta(
                execution_output.parent_state.clone(),
            );
            let after_checkpoint = execution_output.result_state.clone().into_delta(
                last_checkpoint_state.clone(),
            );

            last_checkpoint_auth = Some(parent_auth.update(
                persisted_auth,
                &before_checkpoint,
            ));
            state_auth = last_checkpoint_auth.as_ref().unwrap().update(
                persisted_auth,
                &after_checkpoint,
            );
        } else {
            last_checkpoint_auth = None;
            let updates = execution_output.result_state.clone().into_delta(
                execution_output.parent_state.clone(),
            );
            state_auth = parent_auth.update(
                persisted_auth,
                &updates,
            );
        };

        let mut state_checkpoint_hashes = known_state_checkpoints
            .map_or_else(|| vec![None; num_txns], |v| v.into_iter().collect());
        ensure!(
            state_checkpoint_hashes.len() == execution_output.to_commit.len(),
            "Bad number of known hashes."
        );
        if let Some(last_checkpoint_state) = &execution_output.last_checkpoint_state {
            if let Some(h) = state_checkpoint_hashes[index] {
                ensure!(
                    h == latest_checkpoint.root_hash(),
                    "Last checkpoint not expected."
                );
            } else {
                state_checkpoint_hashes[index] = Some(latest_checkpoint.root_hash());
            }
            latest_checkpoint_version = Some(first_version + index as u64);
        }

        StateCheckpointOutput::new(
            parent_auth.clone(),
            last_checkpoint_auth,
            state_auth,
            state_checkpoint_hashes,
        )
    }
}
