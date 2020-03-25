use std::collections::HashMap;

// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT
use failure::Error;
use serde::{ser, Serialize};
use serde::ser::SerializeSeq;

use crypto::hash::{HashType, ProtocolHash};
use tezos_encoding::types::BigInt;

use crate::p2p::binary_message::BinaryMessage;

pub mod proto_001;
pub mod proto_002;
pub mod proto_003;
pub mod proto_004;
pub mod proto_005;
pub mod proto_005_2;
pub mod proto_006;

#[derive(Debug, Clone)]
pub enum UniversalValue {
    Number(i32),
    /// Ocaml RPC formats i64 as string
    NumberI64(i64),
    BigNumber(BigInt),
    List(Vec<Box<UniversalValue>>),
}

impl UniversalValue {
    fn num<T: Into<i32>>(val: T) -> Self {
        Self::Number(val.into())
    }

    fn i64(val: i64) -> Self {
        Self::NumberI64(val)
    }

    fn big_num(val: BigInt) -> Self {
        Self::BigNumber(val)
    }

    fn i64_list(val: Vec<i64>) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::i64(x)))
        }
        Self::List(ret)
    }

    fn big_num_list<'a, I: IntoIterator<Item=BigInt>>(val: I) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::big_num(x.clone())))
        }
        Self::List(ret)
    }
}

impl Serialize for UniversalValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        match self {
            UniversalValue::BigNumber(num) => {
                serializer.serialize_str(&format!("{}", num.0))
            }
            UniversalValue::Number(num) => {
                serializer.serialize_i32(num.clone())
            }
            UniversalValue::NumberI64(num) => {
                serializer.serialize_str(num.to_string().as_str())
            }
            UniversalValue::List(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

pub fn get_constants_for_rpc(bytes: &[u8], protocol: ProtocolHash) -> Result<Option<HashMap<&'static str, UniversalValue>>, Error> {
    let hash: &str = &HashType::ProtocolHash.bytes_to_string(&protocol);
    match hash {
        proto_001::PROTOCOL_HASH => {
            use crate::protocol::proto_001::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_002::PROTOCOL_HASH => {
            use crate::protocol::proto_002::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_003::PROTOCOL_HASH => {
            use crate::protocol::proto_003::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_004::PROTOCOL_HASH => {
            use crate::protocol::proto_004::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_005::PROTOCOL_HASH => {
            use crate::protocol::proto_005::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_005_2::PROTOCOL_HASH => {
            use crate::protocol::proto_005_2::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        proto_006::PROTOCOL_HASH => {
            use crate::protocol::proto_006::constants::{ParametricConstants, FIXED};
            let mut param = ParametricConstants::from_bytes(bytes.to_vec())?.as_map();
            param.extend(FIXED.clone().as_map());
            Ok(Some(param))
        }
        _ => Ok(None)
    }
}