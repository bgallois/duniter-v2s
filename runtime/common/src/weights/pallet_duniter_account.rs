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

//! Autogenerated weights for `pallet_duniter_account`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-22, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `squirrel`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=2
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_duniter_account`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_duniter_account::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	fn unlink_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3591`
		// Minimum execution time: 10_296_000 picoseconds.
		Weight::from_parts(15_289_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Account::PendingNewAccounts` (r:1 w:1)
	/// Proof: `Account::PendingNewAccounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::RequestIdProvider` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestIdProvider` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::RequestsIds` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestsIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::CounterForRequestsIds` (r:1 w:1)
	/// Proof: `ProvideRandomness::CounterForRequestsIds` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Babe::EpochIndex` (r:1 w:0)
	/// Proof: `Babe::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ProvideRandomness::NexEpochHookIn` (r:1 w:0)
	/// Proof: `ProvideRandomness::NexEpochHookIn` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::RequestsReadyAtEpoch` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestsReadyAtEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Account::PendingRandomIdAssignments` (r:0 w:1)
	/// Proof: `Account::PendingRandomIdAssignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1]`.
	fn on_initialize_sufficient(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42 + i * (309 ±0)`
		//  Estimated: `3816 + i * (309 ±0)`
		// Minimum execution time: 10_313_000 picoseconds.
		Weight::from_parts(10_674_000, 0)
			.saturating_add(Weight::from_parts(0, 3816))
			// Standard Error: 2_538_302
			.saturating_add(Weight::from_parts(27_819_500, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 309).saturating_mul(i.into()))
	}
	/// Storage: `Account::PendingNewAccounts` (r:1 w:1)
	/// Proof: `Account::PendingNewAccounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `ProvideRandomness::RequestIdProvider` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestIdProvider` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::RequestsIds` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestsIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::CounterForRequestsIds` (r:1 w:1)
	/// Proof: `ProvideRandomness::CounterForRequestsIds` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Babe::EpochIndex` (r:1 w:0)
	/// Proof: `Babe::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ProvideRandomness::NexEpochHookIn` (r:1 w:0)
	/// Proof: `ProvideRandomness::NexEpochHookIn` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ProvideRandomness::RequestsReadyAtEpoch` (r:1 w:1)
	/// Proof: `ProvideRandomness::RequestsReadyAtEpoch` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Account::PendingRandomIdAssignments` (r:0 w:1)
	/// Proof: `Account::PendingRandomIdAssignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1]`.
	fn on_initialize_with_balance(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42 + i * (309 ±0)`
		//  Estimated: `3816 + i * (309 ±0)`
		// Minimum execution time: 6_629_000 picoseconds.
		Weight::from_parts(7_761_500, 0)
			.saturating_add(Weight::from_parts(0, 3816))
			// Standard Error: 3_335_635
			.saturating_add(Weight::from_parts(69_367_000, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 309).saturating_mul(i.into()))
	}
	/// Storage: `Account::PendingNewAccounts` (r:1 w:1)
	/// Proof: `Account::PendingNewAccounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[0, 1]`.
	fn on_initialize_no_balance(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42 + i * (74 ±0)`
		//  Estimated: `3507 + i * (74 ±0)`
		// Minimum execution time: 6_699_000 picoseconds.
		Weight::from_parts(7_645_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			// Standard Error: 10_642_627
			.saturating_add(Weight::from_parts(25_816_500, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(i.into()))
	}
	/// Storage: `Account::PendingRandomIdAssignments` (r:1 w:1)
	/// Proof: `Account::PendingRandomIdAssignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_filled_randomness_pending() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `116`
		//  Estimated: `3581`
		// Minimum execution time: 20_089_000 picoseconds.
		Weight::from_parts(21_261_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Account::PendingRandomIdAssignments` (r:1 w:0)
	/// Proof: `Account::PendingRandomIdAssignments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn on_filled_randomness_no_pending() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 4_511_000 picoseconds.
		Weight::from_parts(4_829_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
