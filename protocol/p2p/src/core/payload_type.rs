use crate::pb::pyrind_message::Payload as PyrindMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum PyrindMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&PyrindMessagePayload> for PyrindMessagePayloadType {
    fn from(payload: &PyrindMessagePayload) -> Self {
        match payload {
            PyrindMessagePayload::Addresses(_) => PyrindMessagePayloadType::Addresses,
            PyrindMessagePayload::Block(_) => PyrindMessagePayloadType::Block,
            PyrindMessagePayload::Transaction(_) => PyrindMessagePayloadType::Transaction,
            PyrindMessagePayload::BlockLocator(_) => PyrindMessagePayloadType::BlockLocator,
            PyrindMessagePayload::RequestAddresses(_) => PyrindMessagePayloadType::RequestAddresses,
            PyrindMessagePayload::RequestRelayBlocks(_) => PyrindMessagePayloadType::RequestRelayBlocks,
            PyrindMessagePayload::RequestTransactions(_) => PyrindMessagePayloadType::RequestTransactions,
            PyrindMessagePayload::IbdBlock(_) => PyrindMessagePayloadType::IbdBlock,
            PyrindMessagePayload::InvRelayBlock(_) => PyrindMessagePayloadType::InvRelayBlock,
            PyrindMessagePayload::InvTransactions(_) => PyrindMessagePayloadType::InvTransactions,
            PyrindMessagePayload::Ping(_) => PyrindMessagePayloadType::Ping,
            PyrindMessagePayload::Pong(_) => PyrindMessagePayloadType::Pong,
            PyrindMessagePayload::Verack(_) => PyrindMessagePayloadType::Verack,
            PyrindMessagePayload::Version(_) => PyrindMessagePayloadType::Version,
            PyrindMessagePayload::TransactionNotFound(_) => PyrindMessagePayloadType::TransactionNotFound,
            PyrindMessagePayload::Reject(_) => PyrindMessagePayloadType::Reject,
            PyrindMessagePayload::PruningPointUtxoSetChunk(_) => PyrindMessagePayloadType::PruningPointUtxoSetChunk,
            PyrindMessagePayload::RequestIbdBlocks(_) => PyrindMessagePayloadType::RequestIbdBlocks,
            PyrindMessagePayload::UnexpectedPruningPoint(_) => PyrindMessagePayloadType::UnexpectedPruningPoint,
            PyrindMessagePayload::IbdBlockLocator(_) => PyrindMessagePayloadType::IbdBlockLocator,
            PyrindMessagePayload::IbdBlockLocatorHighestHash(_) => PyrindMessagePayloadType::IbdBlockLocatorHighestHash,
            PyrindMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                PyrindMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            PyrindMessagePayload::DonePruningPointUtxoSetChunks(_) => PyrindMessagePayloadType::DonePruningPointUtxoSetChunks,
            PyrindMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                PyrindMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            PyrindMessagePayload::BlockWithTrustedData(_) => PyrindMessagePayloadType::BlockWithTrustedData,
            PyrindMessagePayload::DoneBlocksWithTrustedData(_) => PyrindMessagePayloadType::DoneBlocksWithTrustedData,
            PyrindMessagePayload::RequestPruningPointAndItsAnticone(_) => PyrindMessagePayloadType::RequestPruningPointAndItsAnticone,
            PyrindMessagePayload::BlockHeaders(_) => PyrindMessagePayloadType::BlockHeaders,
            PyrindMessagePayload::RequestNextHeaders(_) => PyrindMessagePayloadType::RequestNextHeaders,
            PyrindMessagePayload::DoneHeaders(_) => PyrindMessagePayloadType::DoneHeaders,
            PyrindMessagePayload::RequestPruningPointUtxoSet(_) => PyrindMessagePayloadType::RequestPruningPointUtxoSet,
            PyrindMessagePayload::RequestHeaders(_) => PyrindMessagePayloadType::RequestHeaders,
            PyrindMessagePayload::RequestBlockLocator(_) => PyrindMessagePayloadType::RequestBlockLocator,
            PyrindMessagePayload::PruningPoints(_) => PyrindMessagePayloadType::PruningPoints,
            PyrindMessagePayload::RequestPruningPointProof(_) => PyrindMessagePayloadType::RequestPruningPointProof,
            PyrindMessagePayload::PruningPointProof(_) => PyrindMessagePayloadType::PruningPointProof,
            PyrindMessagePayload::Ready(_) => PyrindMessagePayloadType::Ready,
            PyrindMessagePayload::BlockWithTrustedDataV4(_) => PyrindMessagePayloadType::BlockWithTrustedDataV4,
            PyrindMessagePayload::TrustedData(_) => PyrindMessagePayloadType::TrustedData,
            PyrindMessagePayload::RequestIbdChainBlockLocator(_) => PyrindMessagePayloadType::RequestIbdChainBlockLocator,
            PyrindMessagePayload::IbdChainBlockLocator(_) => PyrindMessagePayloadType::IbdChainBlockLocator,
            PyrindMessagePayload::RequestAntipast(_) => PyrindMessagePayloadType::RequestAntipast,
            PyrindMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                PyrindMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
