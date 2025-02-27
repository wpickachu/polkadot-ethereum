
//! Autogenerated weights for pallet_timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-06, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/tmp/artemis-benchmark-tOe/spec.json"), DB CACHE: 128

// Executed Command:
// target/release/artemis
// benchmark
// --chain
// /tmp/artemis-benchmark-tOe/spec.json
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_timestamp
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// runtime/snowbridge/src/weights/pallet_timestamp_weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_timestamp.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for WeightInfo<T> {
	fn set() -> Weight {
		(13_370_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_finalize() -> Weight {
		(6_850_000 as Weight)
	}
}
