//! # Bridge
//!
//! The Bridge module is the primary interface for submitting external messages to the parachain.
//!
//! ## Implementation
//!
//! Before a [Message] is dispatched to a target [`Application`], it is submitted to a [`Verifier`] for verification. The target application is determined using the [`AppId`] submitted along with the message.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `submit`: Submit a message for verification and dispatch.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]

mod channel;
pub mod primitives;

pub use pallet::*;

use frame_support::pallet;

#[pallet]
pub mod pallet {
	use frame_support::{
		traits::Get,
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
	};
	use frame_system::{self as system, ensure_signed};
	use sp_std::marker::PhantomData;
	use artemis_core::{ChannelId, SubmitOutbound, Message, MessageCommitment, Verifier, registry::AppRegistry};
	use crate::channel::inbound::make_inbound_channel;
	use crate::channel::outbound::make_outbound_channel;
	use crate::primitives::{InboundChannelData, OutboundChannelData};

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The verifier module responsible for verifying submitted messages.
		type Verifier: Verifier<<Self as system::Config>::AccountId>;

		type Apps: Get<AppRegistry>;

		type MessageCommitment: MessageCommitment;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	pub type InboundChannels<T: Config> = StorageMap<_, Identity, ChannelId, InboundChannelData, ValueQuery>;

	#[pallet::storage]
	pub type OutboundChannels<T: Config> = StorageMap<_, Identity, ChannelId, OutboundChannelData, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { }

	#[pallet::error]
	pub enum Error<T> {
    	/// Target application not found.
		AppNotFound
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn submit(origin: OriginFor<T>, channel_id: ChannelId, message: Message) -> DispatchResultWithPostInfo {
			let relayer = ensure_signed(origin)?;

			let mut channel = make_inbound_channel::<T>(channel_id);
			channel.submit(&relayer, &message)
				.map(|r| r.into())?;

			Ok(().into())
		}
	}

	impl<T: Config> SubmitOutbound for Pallet<T> {
		fn submit(channel_id: ChannelId, payload: &[u8]) -> DispatchResult {
			// Construct channel object from storage
			let mut channel = make_outbound_channel::<T>(channel_id);
			channel.submit(payload)
		}
	}
}




