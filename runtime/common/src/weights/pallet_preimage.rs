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

//! Autogenerated weights for `pallet_preimage`
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

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3548`
		// Minimum execution time: 38_160_000 picoseconds.
		Weight::from_parts(39_633_500, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			// Standard Error: 65
			.saturating_add(Weight::from_parts(2_347, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3548`
		// Minimum execution time: 25_789_000 picoseconds.
		Weight::from_parts(51_365_500, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			// Standard Error: 59
			.saturating_add(Weight::from_parts(2_238, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3548`
		// Minimum execution time: 24_406_000 picoseconds.
		Weight::from_parts(25_614_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			// Standard Error: 17
			.saturating_add(Weight::from_parts(2_811, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3548`
		// Minimum execution time: 60_607_000 picoseconds.
		Weight::from_parts(70_545_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3548`
		// Minimum execution time: 38_868_000 picoseconds.
		Weight::from_parts(46_917_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn request_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3548`
		// Minimum execution time: 30_541_000 picoseconds.
		Weight::from_parts(37_066_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn request_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3548`
		// Minimum execution time: 30_620_000 picoseconds.
		Weight::from_parts(44_732_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn request_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3548`
		// Minimum execution time: 32_593_000 picoseconds.
		Weight::from_parts(48_836_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn request_requested_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3548`
		// Minimum execution time: 18_050_000 picoseconds.
		Weight::from_parts(20_396_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unrequest_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3548`
		// Minimum execution time: 34_145_000 picoseconds.
		Weight::from_parts(38_096_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn unrequest_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3548`
		// Minimum execution time: 11_945_000 picoseconds.
		Weight::from_parts(15_583_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `106`
		//  Estimated: `3548`
		// Minimum execution time: 16_740_000 picoseconds.
		Weight::from_parts(19_662_000, 0)
			.saturating_add(Weight::from_parts(0, 3548))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1023 w:1023)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1023 w:1023)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(126), added: 2601, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:0 w:1023)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 1024]`.
	fn ensure_updated(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + n * (180 ±0)`
		//  Estimated: `990 + n * (2601 ±0)`
		// Minimum execution time: 16_685_000 picoseconds.
		Weight::from_parts(17_134_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 17_297
			.saturating_add(Weight::from_parts(13_805_632, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2601).saturating_mul(n.into()))
	}
}
