//! # Assets
//!
//! The Assets module provides functionality for handling fungible assets.
//!
//! - [`assets::Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//
//! ## Overview
//!
//! The assets module is used by the Polkadot-Ethereum bridge to store ETH and ERC20 token balances.
//!
//! ### Implementations
//!
//! The Assets module provides implementations for the following traits.
//!
//! - [`MultiAsset`](../artemis_core/assets/trait.MultiAsset.html): Functions for dealing with a
//! multiple fungible assets.
//! - [`SingleAsset`](../artemis_core/assets/trait.SingleAsset.html): Functions for dealing with a
//! single fungible asset.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `transfer`: Transferring a balance between accounts.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		Blake2_128Concat,
		storage::types::{StorageMap, StorageDoubleMap, ValueQuery},
		traits::{Hooks, GenesisBuild, IsType, Get},
		dispatch::{DispatchResult, DispatchError, DispatchResultWithPostInfo},
	};
	use sp_std::marker::PhantomData;
	use frame_system::ensure_signed;
	use sp_runtime::traits::StaticLookup;
	use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
	use sp_std::prelude::*;

	use sp_core::{U256};
	use artemis_core::assets::{AssetId, MultiAsset, SingleAsset};
	use sp_std::marker;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn total_issuance)]
	pub type TotalIssuance<T: Config> = StorageMap<_, Blake2_128Concat, AssetId, U256, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub type Balances<T: Config> = StorageDoubleMap<_, Blake2_128Concat, AssetId, Blake2_128Concat, T::AccountId, U256, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		balances: Vec<(AssetId, T::AccountId, U256)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				balances: vec![],
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for &(ref asset_id, ref who, amount) in self.balances.iter() {
				let total_issuance = TotalIssuance::<T>::get(asset_id);
				TotalIssuance::<T>::insert(asset_id, total_issuance + amount);
				Balances::<T>::insert(asset_id, who, amount);
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { }

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transferred(AssetId, T::AccountId, T::AccountId, U256),
	}

	#[pallet::error]
	pub enum Error<T> {
		TotalIssuanceOverflow,
		TotalIssuanceUnderflow,
		BalanceOverflow,
		InsufficientBalance
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfer some free balance to another account.
		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>,
						asset_id: AssetId,
						dest: <T::Lookup as StaticLookup>::Source,
						amount: U256) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;

			<Self as MultiAsset<_>>::transfer(asset_id, &who, &dest, amount)
				.map(|r| r.into())?;

			Self::deposit_event(Event::<T>::Transferred(asset_id, who, dest, amount));

			Ok(().into())
		}
	}

	impl<T: Config> MultiAsset<T::AccountId> for Pallet<T> {

		fn total_issuance(asset_id: AssetId) -> U256 {
			Pallet::<T>::total_issuance(asset_id)
		}

		fn balance(asset_id: AssetId, who: &T::AccountId) -> U256 {
			Pallet::<T>::balances(asset_id, who)
		}

		fn deposit(asset_id: AssetId, who: &T::AccountId, amount: U256) -> DispatchResult  {
			if amount.is_zero() {
				return Ok(())
			}
			<Balances<T>>::try_mutate(asset_id, who, |balance| -> Result<(), DispatchError> {
				let current_total_issuance = Self::total_issuance(asset_id);
				let new_total_issuance = current_total_issuance.checked_add(amount)
					.ok_or(Error::<T>::TotalIssuanceOverflow)?;
				*balance = balance.checked_add(amount)
					.ok_or(Error::<T>::BalanceOverflow)?;
				TotalIssuance::<T>::insert(asset_id, new_total_issuance);
				Ok(())
			})
		}

		fn withdraw(asset_id: AssetId, who: &T::AccountId, amount: U256) -> DispatchResult  {
			if amount.is_zero() {
				return Ok(())
			}
			<Balances<T>>::try_mutate(asset_id, who, |balance| -> Result<(), DispatchError> {
				let current_total_issuance = Self::total_issuance(asset_id);
				let new_total_issuance = current_total_issuance.checked_sub(amount)
					.ok_or(Error::<T>::TotalIssuanceUnderflow)?;
				*balance = balance.checked_sub(amount)
					.ok_or(Error::<T>::InsufficientBalance)?;
				TotalIssuance::<T>::insert(asset_id, new_total_issuance);
				Ok(())
			})
		}

		fn transfer(
			asset_id: AssetId,
			from: &T::AccountId,
			to: &T::AccountId,
			amount: U256)
		-> DispatchResult {
			if amount.is_zero() || from == to {
				return Ok(())
			}
			<Balances<T>>::try_mutate(asset_id, from, |from_balance| -> DispatchResult {
				<Balances<T>>::try_mutate(asset_id, to, |to_balance| -> DispatchResult {
					*from_balance = from_balance.checked_sub(amount).ok_or(Error::<T>::InsufficientBalance)?;
					*to_balance = to_balance.checked_add(amount).ok_or(Error::<T>::BalanceOverflow)?;
					Ok(())
				})
			})
		}
	}

	pub struct SingleAssetAdaptor<T, I>(marker::PhantomData<(T, I)>);

	impl<T, I> SingleAsset<T::AccountId> for SingleAssetAdaptor<T, I>
	where
		T: Config,
		I: Get<AssetId>,
	{

		fn total_issuance() -> U256 {
			Pallet::<T>::total_issuance(I::get())
		}

		fn balance(who: &T::AccountId) -> U256 {
			Pallet::<T>::balances(I::get(), who)
		}

		fn deposit(
			who: &T::AccountId,
			amount: U256,
		) -> DispatchResult {
			<Pallet<T> as MultiAsset<_>>::deposit(I::get(), who, amount)
		}

		fn withdraw(
			who: &T::AccountId,
			amount: U256,
		) -> DispatchResult {
			<Pallet<T> as MultiAsset<_>>::withdraw(I::get(), who, amount)
		}

		fn transfer(
			source: &T::AccountId,
			dest: &T::AccountId,
			amount: U256,
		) -> DispatchResult {
			<Pallet<T> as MultiAsset<_>>::transfer(I::get(), source, dest, amount)
		}
	}
}


