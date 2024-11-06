// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use crate::transactions_with_output::TransactionsWithOutput;
use aptos_crypto::HashValue;
use aptos_drop_helper::DropHelper;
use aptos_storage_interface::state_delta::StateDelta;
use aptos_types::{
    state_store::{state_key::StateKey, state_value::StateValue},
    transaction::TransactionStatus,
};
use derive_more::Deref;
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct TransactionsByStatus {
    // Statuses of the input transactions, in the same order as the input transactions.
    // Contains BlockMetadata/Validator transactions,
    // but doesn't contain StateCheckpoint/BlockEpilogue, as those get added during execution
    statuses_for_input_txns: Vec<TransactionStatus>,
    // List of all transactions to be committed, including StateCheckpoint/BlockEpilogue if needed.
    to_commit: TransactionsWithOutput,
    to_discard: TransactionsWithOutput,
    to_retry: TransactionsWithOutput,
}

impl TransactionsByStatus {
    pub fn new(
        statuses_for_input_txns: Vec<TransactionStatus>,
        to_commit: TransactionsWithOutput,
        to_discard: TransactionsWithOutput,
        to_retry: TransactionsWithOutput,
    ) -> Self {
        Self {
            statuses_for_input_txns,
            to_commit,
            to_discard,
            to_retry,
        }
    }

    pub fn input_txns_len(&self) -> usize {
        self.statuses_for_input_txns.len()
    }

    pub fn into_inner(
        self,
    ) -> (
        Vec<TransactionStatus>,
        TransactionsWithOutput,
        TransactionsWithOutput,
        TransactionsWithOutput,
    ) {
        (
            self.statuses_for_input_txns,
            self.to_commit,
            self.to_discard,
            self.to_retry,
        )
    }
}

#[derive(Clone, Debug, Default, Deref)]
pub struct StateCheckpointOutput {
    #[deref]
    inner: Arc<DropHelper<Inner>>,
}

impl StateCheckpointOutput {
    pub fn new(
        parent_state: Arc<StateDelta>,
        result_state: Arc<StateDelta>,
        state_updates_before_last_checkpoint: Option<HashMap<StateKey, Option<StateValue>>>,
        state_checkpoint_hashes: Vec<Option<HashValue>>,
    ) -> Self {
        Self::new_impl(Inner {
            parent_state,
            result_state,
            state_updates_before_last_checkpoint,
            state_checkpoint_hashes,
        })
    }

    pub fn new_empty(state: Arc<StateDelta>) -> Self {
        Self::new_impl(Inner {
            parent_state: state.clone(),
            result_state: state,
            state_updates_before_last_checkpoint: None,
            state_checkpoint_hashes: vec![],
        })
    }

    pub fn new_dummy() -> Self {
        Self::new_empty(Arc::new(StateDelta::new_empty()))
    }

    fn new_impl(inner: Inner) -> Self {
        Self {
            inner: Arc::new(DropHelper::new(inner)),
        }
    }

    pub fn reconfig_suffix(&self) -> Self {
        Self::new_empty(self.result_state.clone())
    }
}

#[derive(Debug, Default)]
pub struct Inner {
    pub parent_state: Arc<StateDelta>,
    pub result_state: Arc<StateDelta>,
    pub state_updates_before_last_checkpoint: Option<HashMap<StateKey, Option<StateValue>>>,
    pub state_checkpoint_hashes: Vec<Option<HashValue>>,
}

impl Inner {}
