// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

struct FullNodeInfo {
    address: String,
    latest_version: u64,
    oldest_version: u64,
}

struct LiveDataServiceInfo {
    address: String,
    known_latest_version: u64,
    oldest_version: u64,
}

struct HistoricalDataServiceInfo {
    address: String,
    known_latest_version: u64,
    known_filestore_latest_version: u64,
}

struct GrpcManagerInfo {
    address: String,
    is_master: bool,
}

pub(crate) struct MetadataManager {
    grpc_managers: Vec<GrpcManagerInfo>,
    fullnodes: Vec<FullNodeInfo>,
    live_data_services: Vec<LiveDataServiceInfo>,
    historical_data_services: Vec<HistoricalDataServiceInfo>,
}

impl MetadataManager {
    pub(crate) fn new() -> Self {
        Self {
            grpc_managers: vec![],
            fullnodes: vec![],
            live_data_services: vec![],
            historical_data_services: vec![],
        }
    }

    pub(crate) fn start(&self) -> Result<()> {
        loop {
            for grpc_manager in &self.grpc_managers {}

            for fullnode in &self.fullnodes {}

            for live_data_service in &self.live_data_services {}

            for historical_data_service in &self.historical_data_services {}
        }
    }
}
