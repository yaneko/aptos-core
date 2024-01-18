// Copyright © Aptos Foundation

use crate::{
    move_any, move_any::AsMoveAny, on_chain_config::OnChainConfig,
    validator_verifier::ValidatorConsensusInfo,
};
use anyhow::{bail, ensure, Result};
use aptos_crypto::bls12381;
use aptos_crypto_derive::{BCSCryptoHash, CryptoHasher};
use move_any::Any as MoveAny;
use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::IdentStr, move_resource::MoveStructType,
};
use rand::CryptoRng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, CryptoHasher, BCSCryptoHash)]
pub struct DKGTranscriptMetadata {
    pub epoch: u64,
    pub author: AccountAddress,
}

/// Reflection of Move type `0x1::dkg::DKGStartEvent`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DKGStartEvent {
    pub session_metadata: DKGSessionMetadata,
    pub start_time_us: u64,
}

impl MoveStructType for DKGStartEvent {
    const MODULE_NAME: &'static IdentStr = ident_str!("dkg");
    const STRUCT_NAME: &'static IdentStr = ident_str!("DKGStartEvent");
}

/// Reflection of Move type `0x1::dkg::DKGConfig`.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DKGConfig {
    variant: MoveAny,
}

impl Default for DKGConfig {
    fn default() -> Self {
        Self {
            variant: DKGConfigV0::default().as_move_any(),
        }
    }
}

impl OnChainConfig for DKGConfig {
    const MODULE_IDENTIFIER: &'static str = "dkg";
    const TYPE_IDENTIFIER: &'static str = "DKGConfig";
}

/// Reflection of Move type `0x1::dkg::DKGConfigV0`.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DKGConfigV0 {
    // When this equals to `x`, `x / u64::MAX` is the DKG transcript aggregation threshold.
    aggregation_threshold: u64,
}

impl Default for DKGConfigV0 {
    fn default() -> Self {
        Self {
            aggregation_threshold: u64::MAX / 2,
        }
    }
}

impl AsMoveAny for DKGConfigV0 {
    const MOVE_TYPE_NAME: &'static str = "0x1::dkg::DKGConfigV0";
}

impl DKGConfig {
    pub fn aggregation_threshold(&self) -> Result<u64> {
        match self.variant.type_name.as_str() {
            DKGConfigV0::MOVE_TYPE_NAME => {
                let threshold = MoveAny::unpack::<DKGConfigV0>(
                    DKGConfigV0::MOVE_TYPE_NAME,
                    self.variant.clone(),
                )?
                .aggregation_threshold;
                Ok(threshold)
            },
            _ => {
                bail!("getting aggregation_threshold failed with unknown variant type")
            },
        }
    }
}

/// DKG transcript and its metadata.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DKGNode {
    pub metadata: DKGTranscriptMetadata,
    pub transcript_bytes: Vec<u8>,
}

impl DKGNode {
    pub fn new(epoch: u64, author: AccountAddress, transcript_bytes: Vec<u8>) -> Self {
        Self {
            metadata: DKGTranscriptMetadata { epoch, author },
            transcript_bytes,
        }
    }
}

// The input of DKG.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DKGSessionMetadata {
    pub config: DKGConfig,
    pub dealer_epoch: u64,
    pub dealer_validator_set: Vec<ValidatorConsensusInfo>,
    pub target_validator_set: Vec<ValidatorConsensusInfo>,
}

// The input and the run state of DKG.
/// Reflection of Move type `0x1::dkg::DKGSessionState`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DKGSessionState {
    pub metadata: DKGSessionMetadata,
    pub start_time_us: u64,
    pub result: Vec<u8>,
    pub deadline_microseconds: u64,
}

/// Reflection of Move type `0x1::dkg::DKGState`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DKGState {
    pub last_complete: Option<DKGSessionState>,
    pub in_progress: Option<DKGSessionState>,
}

impl OnChainConfig for DKGState {
    const MODULE_IDENTIFIER: &'static str = "dkg";
    const TYPE_IDENTIFIER: &'static str = "DKGState";
}

pub trait DKGTrait {
    type PrivateParams;
    type PublicParams: Clone + Send + Sync;
    type Transcript: Clone + Default + Send + Sync + for<'a> Deserialize<'a>;

    fn new_public_params(dkg_session_metadata: &DKGSessionMetadata) -> Self::PublicParams;

    fn generate_transcript<R: CryptoRng>(
        rng: &mut R,
        params: &Self::PublicParams,
        my_index: usize,
        sk: &Self::PrivateParams,
    ) -> Self::Transcript;

    fn verify_transcript(params: &Self::PublicParams, trx: &Self::Transcript) -> Result<()>;

    fn aggregate_transcripts(
        params: &Self::PublicParams,
        base: &mut Self::Transcript,
        extra: &Self::Transcript,
    );

    fn serialize_transcript(trx: &Self::Transcript) -> Vec<u8>;
    fn deserialize_transcript(bytes: &[u8]) -> Result<Self::Transcript>;
}

pub trait DKGPrivateParamsProvider<DKG: DKGTrait>: Send + Sync {
    fn dkg_private_params(&self) -> &DKG::PrivateParams;
}

pub struct DummyDKG {}

impl DKGTrait for DummyDKG {
    type PrivateParams = bls12381::PrivateKey;
    type PublicParams = DKGSessionMetadata;
    type Transcript = DummyDKGTranscript;

    fn new_public_params(dkg_session_metadata: &DKGSessionMetadata) -> Self::PublicParams {
        dkg_session_metadata.clone()
    }

    fn generate_transcript<R: CryptoRng>(
        _rng: &mut R,
        _params: &Self::PublicParams,
        _my_index: usize,
        _sk: &Self::PrivateParams,
    ) -> Self::Transcript {
        DummyDKGTranscript::default()
    }

    fn verify_transcript(_params: &Self::PublicParams, trx: &Self::Transcript) -> Result<()> {
        ensure!(
            !trx.data.is_empty(),
            "DummyDKG::verify_transcript failed with bad trx len"
        );
        Ok(())
    }

    fn aggregate_transcripts(
        _params: &Self::PublicParams,
        base: &mut Self::Transcript,
        extra: &Self::Transcript,
    ) {
        base.data.extend(extra.data.to_vec())
    }

    fn serialize_transcript(trx: &Self::Transcript) -> Vec<u8> {
        trx.data.clone()
    }

    fn deserialize_transcript(bytes: &[u8]) -> Result<Self::Transcript> {
        ensure!(
            !bytes.is_empty(),
            "DummyDKG::deserialize_transcript failed with invalid byte string length"
        );
        Ok(DummyDKGTranscript {
            data: bytes.to_vec(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DummyDKGTranscript {
    data: Vec<u8>,
}

impl Default for DummyDKGTranscript {
    fn default() -> Self {
        Self {
            data: b"data".to_vec(),
        }
    }
}

impl DKGPrivateParamsProvider<DummyDKG> for bls12381::PrivateKey {
    fn dkg_private_params(&self) -> &<DummyDKG as DKGTrait>::PrivateParams {
        self
    }
}
