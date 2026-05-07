//! Private validator connection to a remote signer.

pub mod celestia;
pub mod message;
pub mod moca;

pub use cometbft_proto::privval::v1beta1;

/// Message type with moca-cometbft (Greenfield) extensions.
///
/// Note: tags 7/8 are `SignReveal` (moca-specific), and Ping is shifted to
/// 9/10. This is wire-incompatible with vanilla CometBFT and Celestia.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub sum: Option<message::Sum>,
}
