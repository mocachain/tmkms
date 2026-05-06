//! Moca chain (Greenfield-derived) protobuf extensions
//!
//! moca-cometbft (forked from bnb-chain/greenfield-cometbft) adds a
//! `SignReveal` privval RPC for Ethereum-2-style randao_reveal commitments
//! per block proposal. The reveal is an ed25519 signature over
//! `sha256(chain_id + "/") || u64-be(height)` produced by the validator's
//! consensus key — same key that signs Votes/Proposals.
//!
//! This breaks wire compat with vanilla CometBFT and Celestia: moca uses
//! oneof tag 7/8 for SignReveal, shifting Ping to 9/10.

use super::v1beta1::RemoteSignerError;

/// Reveal represents a randao reveal from the block proposer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reveal {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub signature: Vec<u8>,
}

/// SignRevealRequest asks the KMS to sign a randao reveal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignRevealRequest {
    #[prost(message, optional, tag = "1")]
    pub reveal: Option<Reveal>,
    #[prost(string, tag = "2")]
    pub chain_id: String,
}

/// SignedRevealResponse returns the signed reveal (or an error).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedRevealResponse {
    #[prost(message, optional, tag = "1")]
    pub reveal: Option<Reveal>,
    #[prost(message, optional, tag = "2")]
    pub error: Option<RemoteSignerError>,
}
