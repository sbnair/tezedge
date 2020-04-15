// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

//! This module exposes protocol rpc services.
//!
//! Rule 1:
//!     - if service has different implementation and uses different structs for various protocols, it have to be placed and implemented in correct subpackage proto_XYZ
//!     - and here will be just redirector to correct subpackage by protocol_hash
//! Rule 2:
//!     - if service has the same implementation for various protocol, can be place directly here
//!     - if in new version of protocol is changed behavior, we have to splitted it here aslo by protocol_hash

use std::convert::TryInto;
use std::collections::HashMap;

use failure::bail;
use getset::Getters;
use itertools::Itertools;
use serde::Serialize;

use crypto::hash::HashType;
use storage::{num_from_slice, BlockStorage};
use storage::persistent::{ContextList, ContextMap, PersistentStorage};
use storage::skip_list::Bucket;
use storage::context::{TezedgeContext, ContextIndex, ContextApi};
use tezos_messages::base::signature_public_key_hash::SignaturePublicKeyHash;
use tezos_messages::protocol::{
    proto_001 as proto_001_constants,
    proto_002 as proto_002_constants,
    proto_003 as proto_003_constants,
    proto_004 as proto_004_constants,
    proto_005 as proto_005_constants,
    proto_005_2 as proto_005_2_constants,
    proto_006 as proto_006_constants,
    RpcJsonMap,
    UniversalValue,
};

use crate::helpers::{get_context, get_context_protocol_params, get_level_by_block_id};
use crate::rpc_actor::RpcCollectedStateRef;

mod proto_005_2;

/// Return generated baking rights.
///
/// # Arguments
///
/// * `chain_id` - Url path parameter 'chain_id'.
/// * `block_id` - Url path parameter 'block_id', it contains string "head", block level or block hash.
/// * `level` - Url query parameter 'level'.
/// * `delegate` - Url query parameter 'delegate'.
/// * `cycle` - Url query parameter 'cycle'.
/// * `max_priority` - Url query parameter 'max_priority'.
/// * `has_all` - Url query parameter 'all'.
/// * `list` - Context list handler.
/// * `persistent_storage` - Persistent storage handler.
/// * `state` - Current RPC collected state (head).
///
/// Prepare all data to generate baking rights and then use Tezos PRNG to generate them.
pub(crate) fn check_and_get_baking_rights(
    chain_id: &str,
    block_id: &str,
    level: Option<&str>,
    delegate: Option<&str>,
    cycle: Option<&str>,
    max_priority: Option<&str>,
    has_all: bool,
    list: ContextList,
    persistent_storage: &PersistentStorage,
    state: &RpcCollectedStateRef) -> Result<Option<Vec<RpcJsonMap>>, failure::Error> {

    // get protocol and constants
    let context_proto_params = get_context_protocol_params(
        block_id,
        None,
        list.clone(),
        persistent_storage,
        state,
    )?;

    // split impl by protocol
    let hash: &str = &HashType::ProtocolHash.bytes_to_string(&context_proto_params.protocol_hash);
    match hash {
        proto_001_constants::PROTOCOL_HASH
        | proto_002_constants::PROTOCOL_HASH
        | proto_003_constants::PROTOCOL_HASH
        | proto_004_constants::PROTOCOL_HASH
        | proto_005_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        proto_005_2_constants::PROTOCOL_HASH => {
            proto_005_2::rights_service::check_and_get_baking_rights(
                context_proto_params,
                chain_id,
                level,
                delegate,
                cycle,
                max_priority,
                has_all,
                list,
                persistent_storage,
            )
        }
        proto_006_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        _ => panic!("Missing baking rights implemetation for protocol: {}, protocol is not yet supported!", hash)
    }
}

/// Return generated endorsing rights.
///
/// # Arguments
///
/// * `chain_id` - Url path parameter 'chain_id'.
/// * `block_id` - Url path parameter 'block_id', it contains string "head", block level or block hash.
/// * `level` - Url query parameter 'level'.
/// * `delegate` - Url query parameter 'delegate'.
/// * `cycle` - Url query parameter 'cycle'.
/// * `has_all` - Url query parameter 'all'.
/// * `list` - Context list handler.
/// * `persistent_storage` - Persistent storage handler.
/// * `state` - Current RPC collected state (head).
///
/// Prepare all data to generate endorsing rights and then use Tezos PRNG to generate them.
pub(crate) fn check_and_get_endorsing_rights(
    chain_id: &str,
    block_id: &str,
    level: Option<&str>,
    delegate: Option<&str>,
    cycle: Option<&str>,
    has_all: bool,
    list: ContextList,
    persistent_storage: &PersistentStorage,
    state: &RpcCollectedStateRef) -> Result<Option<Vec<RpcJsonMap>>, failure::Error> {

    // get protocol and constants
    let context_proto_params = get_context_protocol_params(
        block_id,
        None,
        list.clone(),
        persistent_storage,
        state,
    )?;

    // split impl by protocol
    let hash: &str = &HashType::ProtocolHash.bytes_to_string(&context_proto_params.protocol_hash);
    match hash {
        proto_001_constants::PROTOCOL_HASH
        | proto_002_constants::PROTOCOL_HASH
        | proto_003_constants::PROTOCOL_HASH
        | proto_004_constants::PROTOCOL_HASH
        | proto_005_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        proto_005_2_constants::PROTOCOL_HASH => {
            proto_005_2::rights_service::check_and_get_endorsing_rights(
                context_proto_params,
                chain_id,
                level,
                delegate,
                cycle,
                has_all,
                list,
                persistent_storage,
            )
        }
        proto_006_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        _ => panic!("Missing endorsing rights implemetation for protocol: {}, protocol is not yet supported!", hash)
    }
}

pub(crate) fn get_votes_listings(_chain_id: &str, block_id: &str, persistent_storage: &PersistentStorage, context_list: ContextList, state: &RpcCollectedStateRef) -> Result<Option<Vec<VoteListings>>, failure::Error> {
    let mut listings = Vec::<VoteListings>::new();

    // get block level first
    let block_level: i64 = match get_level_by_block_id(block_id, persistent_storage, state)? {
        Some(val) => val.try_into()?,
        None => bail!("Block level not found")
    };

    // get the whole context
    let ctxt = get_context(&block_level.to_string(), context_list)?;

    // filter out the listings data
    let listings_data: ContextMap = ctxt.unwrap().into_iter()
        .filter(|(k, _)| k.starts_with(&"data/votes/listings/"))
        .collect();

    // convert the raw context data to VoteListings
    for (key, value) in listings_data.into_iter() {
        if let Bucket::Exists(data) = value {
            // get the address an the curve tag from the key (e.g. data/votes/listings/ed25519/2c/ca/28/ab/01/9ae2d8c26f4ce4924cad67a2dc6618)
            let address = key.split("/").skip(4).take(6).join("");
            let curve = key.split("/").skip(3).take(1).join("");

            let address_decoded = SignaturePublicKeyHash::from_hex_hash_and_curve(&address, &curve)?.to_string();
            listings.push(VoteListings::new(address_decoded, num_from_slice!(data, 0, i32)));
        }
    }

    // sort the vector in reverse ordering (as in ocaml node)
    listings.sort();
    listings.reverse();
    Ok(Some(listings))
}
//ed25519/a8/4d/01/3b/61b4c2cafe3fb89463329d7295a377
/// Struct for the delegates and they voting power (in rolls)
#[derive(Serialize, Debug, Clone, Getters, Eq, Ord, PartialEq, PartialOrd)]
pub struct VoteListings {
    /// Public key hash (address, e.g tz1...)
    #[get = "pub(crate)"]
    pkh: String,

    /// Number of rolls the pkh owns
    #[get = "pub(crate)"]
    rolls: i32,
}

impl VoteListings {
    /// Simple constructor to construct VoteListings
    pub fn new(pkh: String, rolls: i32) -> Self {
        Self {
            pkh,
            rolls,
        }
    }
}

// pub(crate) fn get_votes_current_quorum

pub(crate) fn get_votes_current_quorum(
    chain_id: &str,
    block_id: &str,
    persistent_storage: &PersistentStorage,
    context_list: ContextList,
    state: &RpcCollectedStateRef) -> Result<Option<UniversalValue>, failure::Error> {

    // get protocol and constants
    let context_proto_params = get_context_protocol_params(
        block_id,
        None,
        context_list.clone(),
        persistent_storage,
        state,
    )?;

    let context = TezedgeContext::new(BlockStorage::new(&persistent_storage), context_list);

    // split impl by protocol
    let hash: &str = &HashType::ProtocolHash.bytes_to_string(&context_proto_params.protocol_hash);
    match hash {
        proto_001_constants::PROTOCOL_HASH
        | proto_002_constants::PROTOCOL_HASH
        | proto_003_constants::PROTOCOL_HASH
        | proto_004_constants::PROTOCOL_HASH
        | proto_005_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        proto_005_2_constants::PROTOCOL_HASH => {
            proto_005_2::votes_services::get_current_quorum(context_proto_params, chain_id, context)
        }
        proto_006_constants::PROTOCOL_HASH => panic!("not yet implemented!"),
        _ => panic!("Missing endorsing rights implemetation for protocol: {}, protocol is not yet supported!", hash)
    }
}

pub(crate) fn get_votes_current_proposal(_chain_id: &str, block_id: &str, persistent_storage: &PersistentStorage, context_list: ContextList, state: &RpcCollectedStateRef) -> Result<Option<UniversalValue>, failure::Error> {
    // get level by block_id
    let level: usize = if let Some(l) = get_level_by_block_id(block_id, persistent_storage, state)? {
        l
    } else {
        bail!("Level not found for block_id {}", block_id)
    };

    let context = TezedgeContext::new(BlockStorage::new(&persistent_storage), context_list);
    let context_index = ContextIndex::new(Some(level), None);

    let current_proposal = if let Some(Bucket::Exists(data)) = context.get_key(&context_index, &vec!["data".to_string(), "votes".to_string(), "current_proposal".to_string()])? {
        // data
        // TODO: perform validation before using bytes_to_string
        HashType::ProtocolHash.bytes_to_string(&data)
    } else {
        // set current_proposal to an empty string if there is no current_proposal
        "".to_string()
    };
    
    Ok(Some(UniversalValue::String(current_proposal)))
}

pub(crate) fn get_votes_proposals(chain_id: &str, block_id: &str, persistent_storage: &PersistentStorage, context_list: ContextList, state: &RpcCollectedStateRef) -> Result<Option<Vec<Vec<UniversalValue>>>, failure::Error> {
    // get level by block_id
    let level: usize = if let Some(l) = get_level_by_block_id(block_id, persistent_storage, state)? {
        l
    } else {
        bail!("Level not found for block_id {}", block_id)
    };

    let context = TezedgeContext::new(BlockStorage::new(&persistent_storage), context_list);
    let context_index = ContextIndex::new(Some(level), None);

    let proposals_key = vec!["data/votes/proposals/".to_string()];
    let listings_key = vec!["data/votes/listings/".to_string()];

    let proposals: Vec<UniversalValue>;

    let proposals_data = if let Some(data) = context.get_by_key_prefix(&context_index, &proposals_key)? {
        data
    } else {
        bail!("No proposals found");
    };

    let listings_data = if let Some(data) = context.get_by_key_prefix(&context_index, &listings_key)? {
        data
    } else {
        bail!("No proposals found");
    };

    let mut listings_map: HashMap<String, i32> = Default::default();
    // get listings as a hasmap
    for (key, value) in listings_data.into_iter() {
        if let Bucket::Exists(data) = value {
            // get the address an the curve tag from the key (e.g. data/votes/listings/ed25519/2c/ca/28/ab/01/9ae2d8c26f4ce4924cad67a2dc6618)
            let address = key.split("/").skip(4).take(6).join("");
            let curve = key.split("/").skip(3).take(1).join("");
            println!("Add: {} Curve: {}", address, curve);
            let address_decoded = SignaturePublicKeyHash::from_hex_hash_and_curve(&address, &curve)?.to_string();
            
            let roll_count = num_from_slice!(data, 0, i32);
            listings_map.entry(address_decoded)
                .and_modify(|val| *val = *val + roll_count)
                .or_insert(roll_count);
        }
    }
    println!("Phase2");
    let mut proposal_map: HashMap<String, i32> = Default::default();
    let ret: Vec<Vec<UniversalValue>>;
    for (key, value) in proposals_data.into_iter() {
        if let Bucket::Exists(_) = value  {
            let protocol_hash = HashType::ProtocolHash.bytes_to_string(&hex::decode(key.split("/").skip(3).take(6).join(""))?);
            let address = key.split("/").skip(10).take(6).join("");
            let curve = key.split("/").skip(9).take(1).join("");
            let address_decoded = SignaturePublicKeyHash::from_hex_hash_and_curve(&address, &curve)?.to_string();

            if let Some(data) = listings_map.get(&address_decoded) {
                proposal_map.entry(protocol_hash)
                .and_modify(|val| *val = *val + data)
                .or_insert(data.clone());
            }
        }
    }

    ret = proposal_map.into_iter()
        .map(|(k, v)| vec![UniversalValue::String(k), UniversalValue::Number(v)])
        .collect();
        
    
    Ok(Some(ret))
}