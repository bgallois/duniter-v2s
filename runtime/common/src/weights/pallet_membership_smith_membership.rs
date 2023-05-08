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

//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benjamin-xps139380`, CPU: `Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/duniter
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: SmithMembership PendingMembership (r:1 w:1)
	// Storage: SmithMembership Membership (r:1 w:0)
	// Storage: Parameters ParametersStorage (r:1 w:0)
	// Storage: SmithMembership PendingMembershipsExpireOn (r:1 w:1)
	fn force_request_membership() -> Weight {
		// Minimum execution time: 96_077 nanoseconds.
		Weight::from_ref_time(98_570_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: Identity Identities (r:1 w:0)
	// Storage: SmithMembership PendingMembership (r:1 w:1)
	// Storage: SmithMembership Membership (r:1 w:0)
	// Storage: Parameters ParametersStorage (r:1 w:0)
	// Storage: SmithMembership PendingMembershipsExpireOn (r:1 w:1)
	fn request_membership() -> Weight {
		// Minimum execution time: 122_839 nanoseconds.
		Weight::from_ref_time(125_861_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:2 w:0)
	// Storage: SmithMembership Membership (r:2 w:1)
	// Storage: SmithCert StorageIdtyCertMeta (r:1 w:0)
	// Storage: Parameters ParametersStorage (r:1 w:0)
	// Storage: SmithMembership PendingMembership (r:1 w:1)
	// Storage: SmithMembership CounterForMembership (r:1 w:1)
	// Storage: SmithMembership MembershipsExpireOn (r:1 w:1)
	fn claim_membership() -> Weight {
		// Minimum execution time: 165_369 nanoseconds.
		Weight::from_ref_time(167_607_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: SmithMembership Membership (r:1 w:1)
	// Storage: Identity Identities (r:1 w:0)
	// Storage: Parameters ParametersStorage (r:1 w:0)
	// Storage: SmithMembership MembershipsExpireOn (r:1 w:1)
	fn renew_membership() -> Weight {
		// Minimum execution time: 121_761 nanoseconds.
		Weight::from_ref_time(125_210_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityIndexOf (r:1 w:0)
	// Storage: SmithMembership Membership (r:1 w:1)
	// Storage: SmithMembership CounterForMembership (r:1 w:1)
	// Storage: AuthorityMembers Members (r:1 w:1)
	// Storage: AuthorityMembers OnlineAuthorities (r:1 w:1)
	// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	// Storage: AuthorityMembers IncomingAuthorities (r:1 w:1)
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Session KeyOwner (r:0 w:4)
	fn revoke_membership() -> Weight {
		// Minimum execution time: 234_173 nanoseconds.
		Weight::from_ref_time(239_334_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
}
