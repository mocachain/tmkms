//! moca-cometbft privval message oneof.
//!
//! Tag 7/8 carry the moca/Greenfield-specific `SignReveal` RPC; Ping is
//! shifted to 9/10. This is wire-incompatible with both vanilla CometBFT
//! and Celestia (originally vendored from tendermint-rs), but matches
//! greenfield-cometbft / moca-cometbft.

use super::{moca, v1beta1};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
pub enum Sum {
    #[prost(message, tag = "1")]
    PubKeyRequest(v1beta1::PubKeyRequest),
    #[prost(message, tag = "2")]
    PubKeyResponse(v1beta1::PubKeyResponse),
    #[prost(message, tag = "3")]
    SignVoteRequest(v1beta1::SignVoteRequest),
    #[prost(message, tag = "4")]
    SignedVoteResponse(v1beta1::SignedVoteResponse),
    #[prost(message, tag = "5")]
    SignProposalRequest(v1beta1::SignProposalRequest),
    #[prost(message, tag = "6")]
    SignedProposalResponse(v1beta1::SignedProposalResponse),

    // Moca/Greenfield extensions.
    #[prost(message, tag = "7")]
    SignRevealRequest(moca::SignRevealRequest),
    #[prost(message, tag = "8")]
    SignedRevealResponse(moca::SignedRevealResponse),

    // Ping shifted from 7/8 -> 9/10 in moca-cometbft.
    #[prost(message, tag = "9")]
    PingRequest(v1beta1::PingRequest),
    #[prost(message, tag = "10")]
    PingResponse(v1beta1::PingResponse),
}
