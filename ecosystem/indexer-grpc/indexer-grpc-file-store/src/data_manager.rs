// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_protos::transaction::v1::Transaction;

struct FullNodeClient {}

impl FullNodeClient {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

struct HistoricalDataFetcher {}

impl HistoricalDataFetcher {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

struct LatestDataFetcher {}

impl LatestDataFetcher {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

pub(crate) struct DataManager {
    historical_data_fetcher: HistoricalDataFetcher,
    latest_data_fetcher: LatestDataFetcher,
}

impl DataManager {
    pub(crate) fn new() -> Self {
        let historical_data_fetcher = HistoricalDataFetcher::new();
        let latest_data_fetcher = LatestDataFetcher::new();
        Self {
            historical_data_fetcher,
            latest_data_fetcher,
        }
    }

    pub(crate) async fn get_transactions(&self, start_version: u64) -> Vec<Transaction> {
        vec![]
    }
}
