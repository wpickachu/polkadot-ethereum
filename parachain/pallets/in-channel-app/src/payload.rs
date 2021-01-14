use ethabi::{Event as ABIEvent, Param, ParamKind, Token};
use artemis_ethereum::{DecodeError, log::Log, H160, U256,};
use sp_core::Bytes;

use sp_core::RuntimeDebug;
use sp_std::prelude::*;
use sp_std::convert::TryFrom;

static EVENT_ABI: &ABIEvent = &ABIEvent {
	signature: "NewMessage(uint256,address,string,bytes)",
	inputs: &[
		Param { kind: ParamKind::Uint(256), indexed: false },
		Param { kind: ParamKind::Address, indexed: false },
		// Param { kind: ParamKind::String, indexed: false },
		// Param { kind: ParamKind::Bytes, indexed: false },
	],
	anonymous: false
};

#[derive(Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct InPayload<AccountId: codec::Decode> {
	pub nonce: U256,
	pub senderAddress: AccountId,
	// pub targetApplicationId: String,
	// pub payload: Bytes,
}

impl<AccountId: codec::Decode> TryFrom<Log> for InPayload<AccountId>{
	type Error = DecodeError;

	fn try_from(log: Log) -> Result<Self, Self::Error> {
		// todo
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex_literal::hex;

	const LOG_DATA: [u8; 155] = hex!("
		f899947c5c2fb581612f040ebf9e74f94c9eac8681a95fe1a0691df88ac0
		2f64f3b39fb1b52b940a2730e41ae20f39eec131634df2f8edce77b86000
		0000000000000000000000cffeaaf7681c89285d65cfbe808b80e5026965
		73d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a5
		6da27d00000000000000000000000000000000000000000000000000038d
		7ea4c68000
	");

	#[test]
	fn test_from_log_conversion() {
		let log: Log = rlp::decode(&LOG_DATA).unwrap();
	
		assert_eq!(
			InPayload::try_from(log).unwrap(),
			InPayload {
				nonce: U256::from_dec_str("0").unwrap(),
				senderAddress: hex!["cffeaaf7681c89285d65cfbe808b80e502696573"].into(),
				targetApplicationId: "eth-app",
				payload: "asdf",
			}
		);
	}
}
