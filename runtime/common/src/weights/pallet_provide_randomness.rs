// Copyright 2021-2022 Axiom-Team
//
// This file is part of Duniter-v2S.
//
// Duniter-v2S is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Duniter-v2S is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Duniter-v2S. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_provide_randomness`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=file_header.txt
// --output=runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_provide_randomness`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_provide_randomness::WeightInfo for WeightInfo<T> {
	/// Storage: ProvideRandomness CounterForRequestsIds (r:1 w:1)
	/// Proof: ProvideRandomness CounterForRequestsIds (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ProvideRandomness RequestIdProvider (r:1 w:1)
	/// Proof Skipped: ProvideRandomness RequestIdProvider (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ProvideRandomness RequestsIds (r:1 w:1)
	/// Proof Skipped: ProvideRandomness RequestsIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Babe EpochIndex (r:1 w:0)
	/// Proof: Babe EpochIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: ProvideRandomness NexEpochHookIn (r:1 w:0)
	/// Proof Skipped: ProvideRandomness NexEpochHookIn (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ProvideRandomness RequestsReadyAtEpoch (r:1 w:1)
	/// Proof Skipped: ProvideRandomness RequestsReadyAtEpoch (max_values: None, max_size: None, mode: Measured)
	fn request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `3700`
		// Minimum execution time: 39_666_000 picoseconds.
		Weight::from_parts(41_056_000, 0)
			.saturating_add(Weight::from_parts(0, 3700))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ProvideRandomness RequestsReadyAtNextBlock (r:1 w:1)
	/// Proof Skipped: ProvideRandomness RequestsReadyAtNextBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	/// Proof: Babe AuthorVrfRandomness (max_values: Some(1), max_size: Some(33), added: 528, mode: MaxEncodedLen)
	/// Storage: ProvideRandomness RequestsIds (r:100 w:100)
	/// Proof Skipped: ProvideRandomness RequestsIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ProvideRandomness CounterForRequestsIds (r:1 w:1)
	/// Proof: ProvideRandomness CounterForRequestsIds (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Account PendingRandomIdAssignments (r:100 w:0)
	/// Proof Skipped: Account PendingRandomIdAssignments (max_values: None, max_size: None, mode: Measured)
	/// Storage: ProvideRandomness NexEpochHookIn (r:1 w:1)
	/// Proof Skipped: ProvideRandomness NexEpochHookIn (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `i` is `[1, 100]`.
	fn on_initialize(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383 + i * (59 ±0)`
		//  Estimated: `1869 + i * (2535 ±0)`
		// Minimum execution time: 21_528_000 picoseconds.
		Weight::from_parts(16_137_220, 0)
			.saturating_add(Weight::from_parts(0, 1869))
			// Standard Error: 10_917
			.saturating_add(Weight::from_parts(9_007_362, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 2535).saturating_mul(i.into()))
	}
	/// Storage: ProvideRandomness RequestsReadyAtNextBlock (r:1 w:0)
	/// Proof Skipped: ProvideRandomness RequestsReadyAtNextBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ProvideRandomness NexEpochHookIn (r:1 w:1)
	/// Proof Skipped: ProvideRandomness NexEpochHookIn (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Babe EpochIndex (r:1 w:0)
	/// Proof: Babe EpochIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: ProvideRandomness RequestsReadyAtEpoch (r:1 w:1)
	/// Proof Skipped: ProvideRandomness RequestsReadyAtEpoch (max_values: None, max_size: None, mode: Measured)
	/// Storage: Babe NextRandomness (r:1 w:0)
	/// Proof: Babe NextRandomness (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Babe EpochStart (r:1 w:0)
	/// Proof: Babe EpochStart (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: ProvideRandomness RequestsIds (r:100 w:100)
	/// Proof Skipped: ProvideRandomness RequestsIds (max_values: None, max_size: None, mode: Measured)
	/// Storage: ProvideRandomness CounterForRequestsIds (r:1 w:1)
	/// Proof: ProvideRandomness CounterForRequestsIds (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Account PendingRandomIdAssignments (r:100 w:0)
	/// Proof Skipped: Account PendingRandomIdAssignments (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[1, 100]`.
	fn on_initialize_epoch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `393 + i * (59 ±0)`
		//  Estimated: `3859 + i * (2535 ±0)`
		// Minimum execution time: 23_210_000 picoseconds.
		Weight::from_parts(19_962_422, 0)
			.saturating_add(Weight::from_parts(0, 3859))
			// Standard Error: 10_432
			.saturating_add(Weight::from_parts(9_390_408, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 2535).saturating_mul(i.into()))
	}
}
