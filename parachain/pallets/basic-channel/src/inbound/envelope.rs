use ethabi::{Event, Param, ParamKind, Token};
use artemis_ethereum::{log::Log, H160};

use sp_core::RuntimeDebug;
use sp_std::prelude::*;
use sp_std::convert::TryFrom;

// Used to decode a raw Ethereum log into an [`Envelope`].
static EVENT_ABI: &Event = &Event {
	signature: "Message(address,address,uint64,bytes)",
	inputs: &[
		Param { kind: ParamKind::Address, indexed: false },
		Param { kind: ParamKind::Address, indexed: false },
		Param { kind: ParamKind::Uint(64), indexed: false },
		Param { kind: ParamKind::Bytes, indexed: false },
	],
	anonymous: false
};

/// An inbound message that has had its outer envelope decoded.
#[derive(Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Envelope {
	/// The address of the outbound channel on Ethereum that forwarded this message.
	pub channel: H160,
	/// The address on Ethereum where the message originated from
	pub origin: H160,
	/// The application on Ethereum where the message originated from.
	pub source: H160,
	/// A nonce for enforcing replay protection and ordering.
	pub nonce: u64,
	/// Fee paid by user for relaying the message
//	pub fee: u128,
	/// The inner payload generated from the source application.
	pub payload: Vec<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct EnvelopeDecodeError;

impl TryFrom<Log> for Envelope {
	type Error = EnvelopeDecodeError;

	fn try_from(log: Log) -> Result<Self, Self::Error> {
		let tokens = EVENT_ABI.decode(log.topics, log.data)
			.map_err(|_| EnvelopeDecodeError)?;

		let mut iter = tokens.into_iter();

		let origin = match iter.next().ok_or(EnvelopeDecodeError)? {
			Token::Address(origin) => origin,
			_ => return Err(EnvelopeDecodeError),
		};

		let source = match iter.next().ok_or(EnvelopeDecodeError)? {
			Token::Address(source) => source,
			_ => return Err(EnvelopeDecodeError)
		};

		let nonce = match iter.next().ok_or(EnvelopeDecodeError)? {
			Token::Uint(value) => {
				value.low_u64()
			}
			_ => return Err(EnvelopeDecodeError)
		};

		// let fee = match iter.next().ok_or(EnvelopeDecodeError)? {
		// 	Token::Uint(value) => {
		// 		value.low_u128()
		// 	}
		// 	_ => return Err(EnvelopeDecodeError)
		// };

		let payload = match iter.next().ok_or(EnvelopeDecodeError)? {
			Token::Bytes(payload) => payload,
			_ => return Err(EnvelopeDecodeError)
		};

		Ok(Self {
			channel: log.address,
			origin,
			source,
			nonce,
			payload,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex_literal::hex;

	const LOG: [u8; 317] = hex!(
		"
                f9013a942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0daab80e89869
                997d1cabbe1122788e90fe72b9234ff97a9217dcbb5126f3562fb90100000000
                00000000000000000089b4ab1ef20763630df9743acf155865600daff2000000
                000000000000000000774667629726ec1fabebcec0d9139bd1c8f72a23000000
                0000000000000000000000000000000000000000000000000000000001000000
                0000000000000000000000000000000000000000000000000000000080000000
                00000000000000000000000000000000000000000000000000000000570c0189
                b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd
                04a99fd6822c8558854ccde39a5684e7a56da27d0000c16ff286230000000000
                0000000000000000000000000000000000000000000000000000000000
	"
	);

	#[test]
	fn test_try_from_log() {
		let log: Log = rlp::decode(&LOG).unwrap();
		let envelope = Envelope::try_from(log).unwrap();

		assert_eq!(envelope.clone(),
			Envelope {
				channel: hex!["2ffa5ecdbe006d30397c7636d3e015eee251369f"].into(),
				origin: hex!["89b4ab1ef20763630df9743acf155865600daff2"].into(),
				source: hex!["774667629726ec1fabebcec0d9139bd1c8f72a23"].into(),
				nonce: 1,
				payload: envelope.payload,
			})
	}
}
