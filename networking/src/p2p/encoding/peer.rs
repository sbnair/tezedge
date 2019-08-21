use std::mem::size_of;

use serde::{Deserialize, Serialize};

use tezos_encoding::encoding::{Encoding, Field, HasEncoding, Tag, TagMap};

use crate::p2p::encoding::block_header::{BlockHeaderMessage, GetBlockHeadersMessage};
use crate::p2p::encoding::current_branch::{CurrentBranchMessage, GetCurrentBranchMessage};
use crate::p2p::encoding::current_head::{CurrentHeadMessage, GetCurrentHeadMessage};
use crate::p2p::encoding::operation::{GetOperationsMessage, OperationMessage};
use crate::p2p::encoding::protocol::{GetProtocolsMessage, ProtocolMessage};
use crate::p2p::encoding::operations_for_blocks::{GetOperationsForBlocksMessage, OperationsForBlocksMessage};

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerMessage {
    Disconnect,
//    Advertise,      // TODO
//    SwapRequest,    // TODO
//    SwapAck,        // TODO
    Bootstrap,
    GetCurrentBranch(GetCurrentBranchMessage),
    CurrentBranch(CurrentBranchMessage),
//    Deactivate,     // TODO
    GetCurrentHead(GetCurrentHeadMessage),
    CurrentHead(CurrentHeadMessage),
    GetBlockHeaders(GetBlockHeadersMessage),
    BlockHeader(BlockHeaderMessage),
    GetOperations(GetOperationsMessage),
    Operation(OperationMessage),
    GetProtocols(GetProtocolsMessage),
    Protocol(ProtocolMessage),
//    GetOperationHashesForBlocks,    // TODO
//    OperationHashesForBlock,        // TODO
    GetOperationsForBlocks(GetOperationsForBlocksMessage),
    OperationsForBlocks(OperationsForBlocksMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerMessageResponse {
    messages: Vec<PeerMessage>,
}

impl PeerMessageResponse {
    pub fn get_messages(&self) -> &Vec<PeerMessage> {
        &self.messages
    }
}

impl HasEncoding for PeerMessageResponse {
    fn encoding() -> Encoding {
        Encoding::Obj(vec![
            Field::new("messages", Encoding::dynamic(Encoding::list(
                Encoding::Tags(
                    size_of::<u16>(),
                    TagMap::new(&[
                        Tag::new(0x01, "Disconnect", Encoding::Unit),
                        Tag::new(0x02, "Bootstrap", Encoding::Unit),
                        Tag::new(0x10, "GetCurrentBranch", GetCurrentBranchMessage::encoding()),
                        Tag::new(0x11, "CurrentBranch", CurrentBranchMessage::encoding()),
                        Tag::new(0x13, "GetCurrentHead", GetCurrentHeadMessage::encoding()),
                        Tag::new(0x14, "CurrentHead", CurrentHeadMessage::encoding()),
                        Tag::new(0x20, "GetBlockHeaders", GetBlockHeadersMessage::encoding()),
                        Tag::new(0x21, "BlockHeader", BlockHeaderMessage::encoding()),
                        Tag::new(0x30, "GetOperations", GetOperationsMessage::encoding()),
                        Tag::new(0x31, "Operation", OperationMessage::encoding()),
                        Tag::new(0x40, "GetProtocols", GetProtocolsMessage::encoding()),
                        Tag::new(0x41, "Protocol", ProtocolMessage::encoding()),
                        Tag::new(0x60, "GetOperationsForBlocks", GetOperationsForBlocksMessage::encoding()),
                        Tag::new(0x61, "OperationsForBlocks", OperationsForBlocksMessage::encoding()),
                    ])
                )
            )))
        ])
    }
}

impl From<PeerMessage> for PeerMessageResponse {
    fn from(peer_message: PeerMessage) -> Self {
        PeerMessageResponse { messages: vec![peer_message] }
    }
}