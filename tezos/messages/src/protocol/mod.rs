// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use failure::Error;
use serde::{ser, Serialize};
use serde::ser::{SerializeSeq, SerializeMap};


use crypto::hash::{HashType, ProtocolHash};
use tezos_encoding::types::BigInt;

use crate::p2p::binary_message::BinaryMessage;
use crate::ts_to_rfc3339;

pub mod proto_001;
pub mod proto_002;
pub mod proto_003;
pub mod proto_004;
pub mod proto_005;
pub mod proto_005_2;
pub mod proto_006;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UniversalValue {
    Number(i32),
    /// Ocaml RPC formats i64 as string
    NumberI64(i64),
    BigNumber(BigInt),
    List(Vec<Box<UniversalValue>>),
    Map(HashMap<&'static str, UniversalValue>),
    String(String),
    TimestampRfc3339(i64),
    Bool(bool),
}

impl UniversalValue {
    fn num<T: Into<i32>>(val: T) -> Self {
        Self::Number(val.into())
    }

    fn string(val: String) -> Self {
        Self::String(val)
    }

    fn timestamp_rfc3339(val: i64) -> Self {
        Self::TimestampRfc3339(val)
    }

    fn i64(val: i64) -> Self {
        Self::NumberI64(val)
    }

    fn big_num(val: BigInt) -> Self {
        Self::BigNumber(val)
    }

    fn bool(val: bool) -> Self {
        Self::Bool(val)
    }

    fn map(val: HashMap<&'static str, UniversalValue>) -> Self {
        let mut ret: HashMap<&'static str, UniversalValue> = Default::default();
        for (k, v) in val.into_iter() {
            ret.insert(k, v);
        }
        Self::Map(ret)
    }

    fn i64_list(val: Vec<i64>) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::i64(x)))
        }
        Self::List(ret)
    }

    fn num_list<'a, T: 'a + Into<i32> + Clone, I: IntoIterator<Item=&'a T>>(val: I) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::num(x.clone())))
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

    fn string_list<'a, I: IntoIterator<Item=String>>(val: I) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::string(x.clone())))
        }
        Self::List(ret)
    }
    
    fn map_list<'a, I: IntoIterator<Item=HashMap<&'static str, UniversalValue>>>(val: I) -> Self {
        let mut ret: Vec<Box<UniversalValue>> = Default::default();
        for x in val {
            ret.push(Box::new(Self::map(x.clone())))
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
            UniversalValue::Bool(val) => {
                serializer.serialize_bool(val.clone())
            }
            UniversalValue::BigNumber(num) => {
                serializer.serialize_str(&format!("{}", num.0))
            }
            UniversalValue::Number(num) => {
                serializer.serialize_i32(num.clone())
            }
            UniversalValue::NumberI64(num) => {
                serializer.serialize_str(num.to_string().as_str())
            }
            UniversalValue::String(val) => {
                serializer.serialize_str(val.as_str())
            }
            UniversalValue::TimestampRfc3339(val) => {
                let timestamp = ts_to_rfc3339(val.clone());
                serializer.serialize_str(timestamp.as_str())
            }
            UniversalValue::List(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for value in values { 
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            // UniversalValue::MapList(values) => {
            //     let mut seq = serializer.serialize_seq(Some(values.len()))?;
            //     for value in values {
            //         seq.serialize_element(value)?;
            //     }
            //     seq.end()
            // }
            UniversalValue::Map(values) => {
                let mut map = serializer.serialize_map(Some(values.len()))?;
                for (k, v) in values {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

pub type RpcJsonMap = HashMap<&'static str, UniversalValue>;

pub trait ToRpcJsonList {
    fn as_list(&self) -> UniversalValue;
}

pub trait ToRpcResultJsonList {
    fn as_result_list(&self) -> Result<UniversalValue, failure::Error>;
}

/// A trait for converting a protocol data for RPC json purposes.
pub trait ToRpcJsonMap {
    /// Converts a value of `self` to a HashMap, which can be serialized as json for rpc
    fn as_map(&self) -> RpcJsonMap;
}

/// A trait for converting a protocol data for RPC json purposes.
pub trait ToRpcResultJsonMap {
    /// Converts a value of `self` to a Result<HashMap, failure::Error>, which can be serialized as json for rpc
    fn as_result_map(&self) -> Result<RpcJsonMap, failure::Error>;
}

pub fn get_constants_for_rpc(bytes: &[u8], protocol: ProtocolHash) -> Result<Option<RpcJsonMap>, Error> {
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
        _ => panic!("Missing constants encoding for protocol: {}, protocol is not yet supported!", hash)
    }
}