use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	traits::Get,
	Parameter,
};
use frame_system::{self as system, ensure_signed};
use sp_core::H160;
use sp_std::prelude::*;
use sp_std::convert::TryFrom;
use artemis_core::{
	ChannelId, Message, MessageId,
	MessageDispatch, Verifier,
	rewards::RewardRelayer,
};

use envelope::Envelope;

use sp_runtime::traits::Zero;

mod benchmarking;

#[cfg(test)]
mod test;

mod envelope;

pub trait Config: system::Config {
	type Event: From<Event> + Into<<Self as system::Config>::Event>;

	/// Verifier module for message verification.
	type Verifier: Verifier;

	/// Verifier module for message verification.
	type MessageDispatch: MessageDispatch<Self, MessageId>;

	/// Source of funds to pay relayers
	type RewardsAccount: Get<Self::AccountId>;

	/// Fee type
	type InboundMessageFee: PartialOrd + Parameter + Zero + From<u64>;

	type RewardRelayer: RewardRelayer<Self::AccountId, Self::InboundMessageFee>;
}

decl_storage! {
	trait Store for Module<T: Config> as IncentivizedInboundModule {
		pub SourceChannel get(fn source_channel) config(): H160;
		pub Nonce: u64;
	}
}

decl_event! {
	pub enum Event {

	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Message came from an invalid outbound channel on the Ethereum side.
		InvalidSourceChannel,
		/// Message has an invalid envelope.
		InvalidEnvelope,
		/// Message has an unexpected nonce.
		InvalidNonce,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		pub fn submit(origin, message: Message) -> DispatchResult {
			let relayer = ensure_signed(origin)?;
			// submit message to verifier for verification
			let log = T::Verifier::verify(&message)?;

			// Decode log into an Envelope
			let envelope = Envelope::try_from(log).map_err(|_| Error::<T>::InvalidEnvelope)?;

			// Verify that the message was submitted to us from a known
			// outbound channel on the ethereum side
			if envelope.channel != SourceChannel::get() {
				return Err(Error::<T>::InvalidSourceChannel.into())
			}

			// Verify message nonce
			Nonce::try_mutate(|nonce| -> DispatchResult {
				if envelope.nonce != *nonce + 1 {
					Err(Error::<T>::InvalidNonce.into())
				} else {
					*nonce += 1;
					Ok(())
				}
			})?;

			T::RewardRelayer::pay_relayer(&T::RewardsAccount::get(), &relayer, 0.into());

			let message_id = MessageId::new(ChannelId::Incentivized, envelope.nonce);
			T::MessageDispatch::dispatch(envelope.source, message_id, &envelope.payload);

			Ok(())
		}
	}
}
