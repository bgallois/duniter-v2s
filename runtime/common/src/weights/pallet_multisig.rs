
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/duniter
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output=runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_042_000 picoseconds.
		Weight::from_parts(2_440_917, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(44, 0).saturating_mul(z.into()))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `126 + s * (5 ±0)`
		//  Estimated: `3922`
		// Minimum execution time: 12_494_000 picoseconds.
		Weight::from_parts(11_613_522, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 4_196
			.saturating_add(Weight::from_parts(171_858, 0).saturating_mul(s.into()))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(759, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// The range of component `s` is `[3, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3922`
		// Minimum execution time: 8_831_000 picoseconds.
		Weight::from_parts(8_494_049, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 3_328
			.saturating_add(Weight::from_parts(98_532, 0).saturating_mul(s.into()))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(735, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `260 + s * (37 ±0)`
		//  Estimated: `3922`
		// Minimum execution time: 14_909_000 picoseconds.
		Weight::from_parts(13_975_505, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 5_589
			.saturating_add(Weight::from_parts(183_606, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(770, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131 + s * (5 ±0)`
		//  Estimated: `3922`
		// Minimum execution time: 11_033_000 picoseconds.
		Weight::from_parts(11_970_161, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 4_841
			.saturating_add(Weight::from_parts(102_165, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `3922`
		// Minimum execution time: 7_428_000 picoseconds.
		Weight::from_parts(7_946_425, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 2_616
			.saturating_add(Weight::from_parts(127_413, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Multisig Multisigs (r:1 w:1)
	/// Proof: Multisig Multisigs (max_values: None, max_size: Some(457), added: 2932, mode: MaxEncodedLen)
	/// The range of component `s` is `[2, 10]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `329 + s * (5 ±0)`
		//  Estimated: `3922`
		// Minimum execution time: 12_601_000 picoseconds.
		Weight::from_parts(13_390_599, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			// Standard Error: 3_815
			.saturating_add(Weight::from_parts(106_076, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
