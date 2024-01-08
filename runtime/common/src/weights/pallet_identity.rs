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

//! Autogenerated weights for `pallet_identity`
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: Identity IdentityIndexOf (r:2 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:2 w:2)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert StorageIdtyCertMeta (r:2 w:2)
	/// Proof Skipped: Cert StorageIdtyCertMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Identity NextIdtyIndex (r:1 w:1)
	/// Proof Skipped: Identity NextIdtyIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Identity IdentityChangeSchedule (r:1 w:1)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Cert StorageCertsRemovableOn (r:1 w:1)
	/// Proof Skipped: Cert StorageCertsRemovableOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert CertsByReceiver (r:1 w:1)
	/// Proof Skipped: Cert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithCert StorageIdtyCertMeta (r:1 w:0)
	/// Proof Skipped: SmithCert StorageIdtyCertMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn create_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1165`
		//  Estimated: `7105`
		// Minimum execution time: 80_562_000 picoseconds.
		Weight::from_parts(93_717_000, 0)
			.saturating_add(Weight::from_parts(0, 7105))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentitiesNames (r:1 w:1)
	/// Proof Skipped: Identity IdentitiesNames (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityChangeSchedule (r:2 w:2)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	fn confirm_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `661`
		//  Estimated: `6601`
		// Minimum execution time: 29_211_000 picoseconds.
		Weight::from_parts(31_414_000, 0)
			.saturating_add(Weight::from_parts(0, 6601))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Identity IdentityIndexOf (r:2 w:2)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:0)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn change_owner_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `837`
		//  Estimated: `6777`
		// Minimum execution time: 91_798_000 picoseconds.
		Weight::from_parts(104_299_000, 0)
			.saturating_add(Weight::from_parts(0, 6777))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Identity IdentityChangeSchedule (r:2 w:2)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:1)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `778`
		//  Estimated: `6718`
		// Minimum execution time: 77_566_000 picoseconds.
		Weight::from_parts(80_738_000, 0)
			.saturating_add(Weight::from_parts(0, 6718))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Identity IdentitiesNames (r:0 w:998)
	/// Proof Skipped: Identity IdentitiesNames (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[1, 1000]`.
	fn prune_item_identities_names(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_022_000 picoseconds.
		Weight::from_parts(2_149_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 972
			.saturating_add(Weight::from_parts(1_331_794, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn fix_sufficients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3591`
		// Minimum execution time: 6_683_000 picoseconds.
		Weight::from_parts(7_169_000, 0)
			.saturating_add(Weight::from_parts(0, 3591))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn link_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `371`
		//  Estimated: `3836`
		// Minimum execution time: 59_456_000 picoseconds.
		Weight::from_parts(62_204_000, 0)
			.saturating_add(Weight::from_parts(0, 3836))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 131_000 picoseconds.
		Weight::from_parts(152_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn do_revoke_identity_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 4_237_000 picoseconds.
		Weight::from_parts(4_452_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityChangeSchedule (r:2 w:1)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership CounterForMembership (r:1 w:1)
	/// Proof: Membership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Membership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: Membership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: UniversalDividend CurrentUdIndex (r:1 w:0)
	/// Proof: UniversalDividend CurrentUdIndex (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: SmithMembership Membership (r:1 w:1)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership CounterForMembership (r:1 w:1)
	/// Proof: SmithMembership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: SmithMembership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: SmithMembership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorityMembers Members (r:1 w:1)
	/// Proof Skipped: AuthorityMembers Members (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorityMembers OnlineAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers OnlineAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers OutgoingAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers OutgoingAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	/// Proof Skipped: AuthorityMembers AuthoritiesCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers IncomingAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers IncomingAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Session KeyOwner (r:0 w:4)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn do_revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1525`
		//  Estimated: `7465`
		// Minimum execution time: 95_659_000 picoseconds.
		Weight::from_parts(102_307_000, 0)
			.saturating_add(Weight::from_parts(0, 7465))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(20))
	}
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn do_remove_identity_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 4_394_000 picoseconds.
		Weight::from_parts(4_642_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership CounterForMembership (r:1 w:1)
	/// Proof: Membership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Membership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: Membership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:1)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership CounterForMembership (r:1 w:1)
	/// Proof: SmithMembership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: SmithMembership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: SmithMembership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorityMembers Members (r:1 w:1)
	/// Proof Skipped: AuthorityMembers Members (max_values: None, max_size: None, mode: Measured)
	/// Storage: AuthorityMembers OnlineAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers OnlineAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers OutgoingAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers OutgoingAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers AuthoritiesCounter (r:1 w:1)
	/// Proof Skipped: AuthorityMembers AuthoritiesCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AuthorityMembers IncomingAuthorities (r:1 w:1)
	/// Proof Skipped: AuthorityMembers IncomingAuthorities (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityIndexOf (r:0 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Session KeyOwner (r:0 w:4)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn do_remove_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1432`
		//  Estimated: `6192`
		// Minimum execution time: 99_220_000 picoseconds.
		Weight::from_parts(103_629_000, 0)
			.saturating_add(Weight::from_parts(0, 6192))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(22))
	}
	/// Storage: Identity IdentityChangeSchedule (r:1 w:0)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	fn prune_identities_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `3573`
		// Minimum execution time: 2_146_000 picoseconds.
		Weight::from_parts(2_339_000, 0)
			.saturating_add(Weight::from_parts(0, 3573))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Identity IdentityChangeSchedule (r:1 w:1)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn prune_identities_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3757`
		// Minimum execution time: 6_269_000 picoseconds.
		Weight::from_parts(6_723_000, 0)
			.saturating_add(Weight::from_parts(0, 3757))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityChangeSchedule (r:1 w:1)
	/// Proof Skipped: Identity IdentityChangeSchedule (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert CertsByReceiver (r:1 w:0)
	/// Proof Skipped: Cert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:1)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithCert CertsByReceiver (r:1 w:0)
	/// Proof Skipped: SmithCert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentityIndexOf (r:0 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn prune_identities_err() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1177`
		//  Estimated: `4642`
		// Minimum execution time: 46_324_000 picoseconds.
		Weight::from_parts(47_896_000, 0)
			.saturating_add(Weight::from_parts(0, 4642))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
