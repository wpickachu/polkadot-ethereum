//! # InChannel
//!
//! An application that implements an incoming channel.
//!
//! ## Overview
//!
//! ## Interface
//!
//! This application implements the [`Application`] trait and conforms to its interface
//!
//! ### Dispatchable Calls
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{self as system, ensure_signed};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
};
use sp_std::prelude::*;
use sp_std::convert::TryInto;
use sp_core::{H160, U256};

use artemis_core::{Application, Commitments, SingleAsset};
use artemis_ethereum::Log;

mod payload;
use payload::{InPayload};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: system::Config {
	type Event: From<Event<Self>> + Into<<Self as system::Config>::Event>;

	type Asset: SingleAsset<<Self as system::Config>::AccountId>;

	type Commitments: Commitments;
}

decl_storage! {
	trait Store for Module<T: Config> as InChannelModule {
		Address get(fn address) config(): H160;
	}
}

decl_event!(
    /// Events for the ETH module.
	pub enum Event<T>
	{
		// Dispatched(String, U256),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The submitted payload could not be decoded.
		InvalidPayload,
	}
}

decl_module! {

	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		type Error = Error<T>;

		fn deposit_event() = default;

	}
}

impl<T: Config> Module<T> {

	fn handle_event(payload: InPayload<T::AccountId>) -> DispatchResult {
		// todo - dispatch to correct pallet
		// Self::deposit_event(RawEvent::Minted(payload.recipient_addr.clone(), payload.amount));
		// Ok(())
	}
}

impl<T: Config> Application for Module<T> {
	fn handle(payload: &[u8]) -> DispatchResult {
		// Decode ethereum Log event from RLP-encoded data, and try to convert to InPayload
		let payload_decoded = rlp::decode::<Log>(payload)
			.map_err(|_| Error::<T>::InvalidPayload)?
			.try_into()
			.map_err(|_| Error::<T>::InvalidPayload)?;

		Self::handle_event(payload_decoded)
	}

	fn address() -> H160 {
		Address::get()
	}
}
