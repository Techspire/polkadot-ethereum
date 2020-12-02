//! Types for representing messages

use frame_support::RuntimeDebug;
use sp_std::vec::Vec;
use sp_core::H160;

use codec::{Encode, Decode};

/// Identifier for an application module registered within the runtime.
///
/// Typically an identifier of this type will hold an Ethereum contract address. This provides a mechanism
/// for cross-chain routing of messages.
pub type AppId = [u8; 20];

/// A message relayed from Ethereum.
#[derive(PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub struct Message {
	/// The raw message payload.
	///
	/// Its content is undefined and can only be decoded by target applications.
	pub payload: Vec<u8>,

	/// Input to the message verifier
	pub verification: VerificationInput,
}

/// Verification input for the message verifier.
///
/// This data type allows us to support multiple verification schemes. In the near future,
/// A light-client scheme will be added too.
#[derive(PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub enum VerificationInput {
	/// Basic scheme supports replay protection
	Basic {
		/// The block number of the block in which the event was included.
		block_number: u64,
		/// The index of the event within the block.
		event_index: u32,
	},
	/// Light-client-based scheme checks receipt inclusion proof
	/// TODO: use specialized structs, not byte arrays
	LightReceipt {
		// The receipt in the block
		receipt: Vec<u8>,
		// Merkle proof of inclusion in the block's receipts
		proof: Vec<u8>,
	},
	/// Light-client-based scheme checks transaction inclusion proof
	/// /// TODO: use specialized structs, not byte arrays
	LightTransaction {
		// The transaction in the block
		transaction: Vec<u8>,
		// Merke proof of inclusion in the block's transactions
		proof: Vec<u8>,
	},
	/// No verification scheme. Such messages will be dropped!
	None
}

/// ID for Bridged Assets
pub type BridgedAssetId = H160;
