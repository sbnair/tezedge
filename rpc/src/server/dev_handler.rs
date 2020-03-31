// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use hyper::{Body, Request};
use slog::warn;

use crate::{empty, make_json_response, result_to_json_response, ServiceResult, unwrap_block_hash};
use crate::server::{HasSingleValue, Params, Query, RpcServiceEnvironment};
use crate::server::service;

pub async fn dev_blocks(_: Request<Body>, _: Params, query: Query, env: RpcServiceEnvironment) -> ServiceResult {
    let from_block_id = unwrap_block_hash(query.get_str("from_block_id"), env.state(), env.genesis_hash());
    let limit = query.get_usize("limit").unwrap_or(50);
    let every_nth_level = match query.get_str("every_nth") {
        Some("cycle") => Some(4096),
        Some("voting-period") => Some(4096 * 8),
        _ => None
    };
    result_to_json_response(service::get_blocks(every_nth_level, &from_block_id, limit, env.persistent_storage(), env.state()), env.log())
}

pub async fn dev_block_actions(_: Request<Body>, params: Params, _: Query, env: RpcServiceEnvironment) -> ServiceResult {
    let block_id = params.get_str("block_id").unwrap();
    result_to_json_response(service::get_block_actions(block_id, env.persistent_storage(), env.state()), env.log())
}

pub async fn dev_contract_actions(_: Request<Body>, params: Params, query: Query, env: RpcServiceEnvironment) -> ServiceResult {
    let contract_id = params.get_str("contract_id").unwrap();
    let from_id = query.get_u64("from_id");
    let limit = query.get_usize("limit").unwrap_or(50);
    result_to_json_response(service::get_contract_actions(contract_id, from_id, limit, env.persistent_storage()), env.log())
}

pub async fn dev_context(_: Request<Body>, params: Params, _: Query, env: RpcServiceEnvironment) -> ServiceResult {
    // TODO: Add parameter checks
    let context_level = params.get_str("id").unwrap();
    result_to_json_response(service::get_context(context_level, env.persistent_storage().context_storage()), env.log())
}

pub async fn dev_stats_memory(_: Request<Body>, _: Params, _: Query, env: RpcServiceEnvironment) -> ServiceResult {
    match service::get_stats_memory() {
        Ok(resp) => make_json_response(&resp),
        Err(e) => {
            warn!(env.log(), "GetStatsMemory: {}", e);
            empty()
        }
    }
}