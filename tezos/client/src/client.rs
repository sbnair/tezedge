// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use crypto::hash::{ChainId, ContextHash, ProtocolHash};
use tezos_api::ffi::{
    ApplyBlockError, ApplyBlockRequest, ApplyBlockRequestBuilder, ApplyBlockResult,
    CommitGenesisResult,
    ContextDataError, GenesisChain, GetDataError, InitProtocolContextResult, ProtocolOverrides, TezosGenerateIdentityError,
    TezosRuntimeConfiguration, TezosRuntimeConfigurationError, TezosStorageInitError,
};
use tezos_api::identity::Identity;
use tezos_interop::ffi;
use tezos_messages::p2p::encoding::prelude::*;

/// Override runtime configuration for OCaml runtime
pub fn change_runtime_configuration(settings: TezosRuntimeConfiguration) -> Result<(), TezosRuntimeConfigurationError> {
    match ffi::change_runtime_configuration(settings) {
        Ok(result) => Ok(result?),
        Err(e) => {
            Err(TezosRuntimeConfigurationError::ChangeConfigurationError {
                message: format!("FFI 'change_runtime_configuration' failed! Reason: {:?}", e)
            })
        }
    }
}

/// Initializes context for Tezos ocaml protocol
pub fn init_protocol_context(
    storage_data_dir: String,
    genesis: GenesisChain,
    protocol_overrides: ProtocolOverrides,
    commit_genesis: bool,
    enable_testchain: bool) -> Result<InitProtocolContextResult, TezosStorageInitError> {
    match ffi::init_protocol_context(storage_data_dir, genesis, protocol_overrides, commit_genesis, enable_testchain) {
        Ok(result) => Ok(result?),
        Err(e) => {
            Err(TezosStorageInitError::InitializeError {
                message: format!("FFI 'init_protocol_context' failed! Initialization of Tezos context failed, this storage is required, we can do nothing without that! Reason: {:?}", e)
            })
        }
    }
}

/// Gets data for genesis
pub fn genesis_result_data(
    context_hash: &ContextHash,
    chain_id: &ChainId,
    protocol_hash: &ProtocolHash,
    genesis_max_operations_ttl: u16,
) -> Result<CommitGenesisResult, GetDataError> {
    match ffi::genesis_result_data(context_hash.clone(), chain_id.clone(), protocol_hash.clone(), genesis_max_operations_ttl) {
        Ok(result) => Ok(result?),
        Err(e) => {
            Err(GetDataError::ReadError {
                message: format!("FFI 'genesis_result_data' failed! Reason: {:?}", e)
            })
        }
    }
}

/// Applies new block to Tezos ocaml storage, means:
/// - block and operations are decoded by the protocol
/// - block and operations data are correctly stored in Tezos chain/storage
/// - new current head is evaluated
/// - returns validation_result.message
pub fn apply_block(
    chain_id: &ChainId,
    block_header: &BlockHeader,
    predecessor_block_header: &BlockHeader,
    operations: &Vec<Option<OperationsForBlocksMessage>>,
    max_operations_ttl: u16) -> Result<ApplyBlockResult, ApplyBlockError> {

    // check operations count by validation_pass
    if (block_header.validation_pass() as usize) != operations.len() {
        return Err(ApplyBlockError::IncompleteOperations {
            expected: block_header.validation_pass() as usize,
            actual: operations.len(),
        });
    }

    // request
    let request: ApplyBlockRequest = ApplyBlockRequestBuilder::default()
        .chain_id(chain_id.clone())
        .block_header(block_header.clone())
        .pred_header(predecessor_block_header.clone())
        .max_operations_ttl(max_operations_ttl as i32)
        .operations(ApplyBlockRequest::convert_operations(operations))
        .build().unwrap();

    match ffi::apply_block(request) {
        Ok(result) => result,
        Err(e) => {
            Err(ApplyBlockError::FailedToApplyBlock {
                message: format!("Unknown OcamlError: {:?}", e)
            })
        }
    }
}

/// Generate tezos identity
pub fn generate_identity(expected_pow: f64) -> Result<Identity, TezosGenerateIdentityError> {
    match ffi::generate_identity(expected_pow) {
        Ok(result) => Ok(result?),
        Err(e) => {
            Err(TezosGenerateIdentityError::GenerationError {
                message: format!("FFI 'generate_identity' failed! Reason: {:?}", e)
            })
        }
    }
}

/// Decode protocoled context data
pub fn decode_context_data(protocol_hash: ProtocolHash, key: Vec<String>, data: Vec<u8>) -> Result<Option<String>, ContextDataError> {
    match ffi::decode_context_data(protocol_hash, key, data) {
        Ok(result) => Ok(result?),
        Err(e) => {
            Err(ContextDataError::DecodeError {
                message: format!("FFI 'decode_context_data' failed! Reason: {:?}", e)
            })
        }
    }
}