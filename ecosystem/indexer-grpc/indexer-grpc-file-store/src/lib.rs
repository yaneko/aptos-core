// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

pub mod data_manager;
pub mod file_store_uploader;
pub mod metadata_manager;
pub mod metrics;
pub mod service;

use crate::{
    data_manager::DataManager, metadata_manager::MetadataManager, service::GrpcManagerService,
};
use anyhow::Result;
use aptos_indexer_grpc_server_framework::RunnableConfig;
use aptos_indexer_grpc_utils::config::IndexerGrpcFileStoreConfig;
use aptos_protos::indexer::v1::grpc_manager_server::GrpcManagerServer;
use file_store_uploader::FileStoreUploader;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tonic::{codec::CompressionEncoding, transport::Server};

const HTTP2_PING_INTERVAL_DURATION: Duration = Duration::from_secs(60);
const HTTP2_PING_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ServiceConfig {
    listen_address: SocketAddr,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexerGrpcManagerConfig {
    pub chain_id: u64,
    pub service_config: ServiceConfig,
    pub file_store_config: IndexerGrpcFileStoreConfig,
}

#[async_trait::async_trait]
impl RunnableConfig for IndexerGrpcManagerConfig {
    async fn run(&self) -> Result<()> {
        let grpc_manager = GrpcManager::new(self);
        grpc_manager
            .start(&self.service_config.listen_address)
            .await?;
        Ok(())
    }

    fn get_server_name(&self) -> String {
        "grpc_manager".to_string()
    }
}

struct GrpcManager {
    chain_id: u64,
    filestore_uploader: FileStoreUploader,
    metadata_manager: Arc<MetadataManager>,
    data_manager: Arc<DataManager>,
}

impl GrpcManager {
    pub(crate) fn new(config: &IndexerGrpcManagerConfig) -> Self {
        let chain_id = config.chain_id;
        let filestore_uploader =
            block_on(FileStoreUploader::new(&config.file_store_config, chain_id)).expect(&format!(
                "Failed to create filestore uploader, config: {:?}.",
                config.file_store_config
            ));
        let data_manager = Arc::new(DataManager::new());
        let metadata_manager = Arc::new(MetadataManager::new());
        Self {
            chain_id,
            filestore_uploader,
            metadata_manager,
            data_manager,
        }
    }

    pub(crate) async fn start(&self, listen_address: &SocketAddr) -> Result<()> {
        self.metadata_manager.start()?;
        let service = GrpcManagerServer::new(GrpcManagerService::new(
            self.chain_id,
            self.metadata_manager.clone(),
            self.data_manager.clone(),
        ))
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);
        let server = Server::builder()
            .http2_keepalive_interval(Some(HTTP2_PING_INTERVAL_DURATION))
            .http2_keepalive_timeout(Some(HTTP2_PING_TIMEOUT_DURATION))
            .add_service(service);
        server
            .serve(*listen_address)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}
