
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_poe.
pub trait WeightInfo {
	fn create_claim(d: u32, ) -> Weight;
	fn revoke_claim(d: u32, ) -> Weight;
	fn transfer_claim(d: u32, ) -> Weight;
}

/// Weights for pallet_poe using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn create_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_955, 3528)
			// Standard Error: 4_590
			.saturating_add(Weight::from_parts(4_013, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_548_865, 3528)
			// Standard Error: 15_304
			.saturating_add(Weight::from_parts(27_459, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_716_488, 3528)
			// Standard Error: 10_996
			.saturating_add(Weight::from_parts(31_472, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn create_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3528`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_955, 3528)
			// Standard Error: 4_590
			.saturating_add(Weight::from_parts(4_013, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_548_865, 3528)
			// Standard Error: 15_304
			.saturating_add(Weight::from_parts(27_459, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PoeModule::Proofs` (r:1 w:1)
	/// Proof: `PoeModule::Proofs` (`max_values`: None, `max_size`: Some(63), added: 2538, mode: `MaxEncodedLen`)
	/// The range of component `d` is `[0, 10]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `85 + d * (1 ±0)`
		//  Estimated: `3528`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_716_488, 3528)
			// Standard Error: 10_996
			.saturating_add(Weight::from_parts(31_472, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}