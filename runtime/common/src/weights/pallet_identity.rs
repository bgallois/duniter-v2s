
//! Autogenerated weights for `pallet_identity`
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
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentitiesRemovableOn (r:1 w:1)
	/// Proof Skipped: Identity IdentitiesRemovableOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert StorageCertsRemovableOn (r:1 w:1)
	/// Proof Skipped: Cert StorageCertsRemovableOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert CertsByReceiver (r:1 w:1)
	/// Proof Skipped: Cert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn create_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `982`
		//  Estimated: `6922`
		// Minimum execution time: 38_137_000 picoseconds.
		Weight::from_parts(39_647_000, 0)
			.saturating_add(Weight::from_parts(0, 6922))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: Identity IdentityIndexOf (r:1 w:0)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentitiesNames (r:1 w:1)
	/// Proof Skipped: Identity IdentitiesNames (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership PendingMembership (r:1 w:1)
	/// Proof Skipped: Membership PendingMembership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:0)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Membership PendingMembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: Membership PendingMembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	fn confirm_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `955`
		//  Estimated: `4420`
		// Minimum execution time: 23_569_000 picoseconds.
		Weight::from_parts(24_735_000, 0)
			.saturating_add(Weight::from_parts(0, 4420))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership PendingMembership (r:1 w:1)
	/// Proof Skipped: Membership PendingMembership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert StorageIdtyCertMeta (r:1 w:0)
	/// Proof Skipped: Cert StorageIdtyCertMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Distance IdentityDistanceStatus (r:1 w:0)
	/// Proof Skipped: Distance IdentityDistanceStatus (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership CounterForMembership (r:1 w:1)
	/// Proof: Membership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Membership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: Membership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: UniversalDividend CurrentUdIndex (r:1 w:0)
	/// Proof: UniversalDividend CurrentUdIndex (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	fn validate_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1948`
		//  Estimated: `5413`
		// Minimum execution time: 39_059_000 picoseconds.
		Weight::from_parts(42_214_000, 0)
			.saturating_add(Weight::from_parts(0, 5413))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
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
	/// Storage: AuthorityMembers Members (r:1 w:0)
	/// Proof Skipped: AuthorityMembers Members (max_values: None, max_size: None, mode: Measured)
	fn change_owner_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1119`
		//  Estimated: `7059`
		// Minimum execution time: 76_290_000 picoseconds.
		Weight::from_parts(79_765_000, 0)
			.saturating_add(Weight::from_parts(0, 7059))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:0)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Cert CertsByReceiver (r:1 w:1)
	/// Proof Skipped: Cert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert StorageIdtyCertMeta (r:2 w:2)
	/// Proof Skipped: Cert StorageIdtyCertMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Identity IdentityIndexOf (r:0 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn revoke_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1498`
		//  Estimated: `7438`
		// Minimum execution time: 84_313_000 picoseconds.
		Weight::from_parts(87_192_000, 0)
			.saturating_add(Weight::from_parts(0, 7438))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	/// Storage: SmithMembership Membership (r:1 w:0)
	/// Proof Skipped: SmithMembership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Cert CertsByReceiver (r:1 w:1)
	/// Proof Skipped: Cert CertsByReceiver (max_values: None, max_size: None, mode: Measured)
	/// Storage: Cert StorageIdtyCertMeta (r:2 w:2)
	/// Proof Skipped: Cert StorageIdtyCertMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Parameters ParametersStorage (r:1 w:0)
	/// Proof Skipped: Parameters ParametersStorage (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Identity IdentityIndexOf (r:0 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity IdentitiesNames (r:0 w:1)
	/// Proof Skipped: Identity IdentitiesNames (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	fn force_remove_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1343`
		//  Estimated: `7283`
		// Minimum execution time: 38_696_000 picoseconds.
		Weight::from_parts(39_816_000, 0)
			.saturating_add(Weight::from_parts(0, 7283))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: Identity IdentitiesNames (r:0 w:998)
	/// Proof Skipped: Identity IdentitiesNames (max_values: None, max_size: None, mode: Measured)
	/// The range of component `i` is `[1, 1000]`.
	fn prune_item_identities_names(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 804_000 picoseconds.
		Weight::from_parts(888_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 939
			.saturating_add(Weight::from_parts(642_823, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	fn fix_sufficients() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3591`
		// Minimum execution time: 3_775_000 picoseconds.
		Weight::from_parts(4_240_000, 0)
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
		//  Measured:  `307`
		//  Estimated: `3772`
		// Minimum execution time: 45_159_000 picoseconds.
		Weight::from_parts(46_401_000, 0)
			.saturating_add(Weight::from_parts(0, 3772))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 28_000 picoseconds.
		Weight::from_parts(31_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn do_remove_identity_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 3_245_000 picoseconds.
		Weight::from_parts(3_383_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Identity Identities (r:1 w:1)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
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
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(126), added: 2601, mode: MaxEncodedLen)
	/// Storage: Membership Membership (r:1 w:1)
	/// Proof Skipped: Membership Membership (max_values: None, max_size: None, mode: Measured)
	/// Storage: Membership CounterForMembership (r:1 w:1)
	/// Proof: Membership CounterForMembership (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Membership MembershipsExpireOn (r:1 w:1)
	/// Proof Skipped: Membership MembershipsExpireOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: UniversalDividend CurrentUdIndex (r:1 w:0)
	/// Proof: UniversalDividend CurrentUdIndex (max_values: Some(1), max_size: Some(2), added: 497, mode: MaxEncodedLen)
	/// Storage: Identity CounterForIdentities (r:1 w:1)
	/// Proof: Identity CounterForIdentities (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Identity IdentityIndexOf (r:0 w:1)
	/// Proof Skipped: Identity IdentityIndexOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: Quota IdtyQuota (r:0 w:1)
	/// Proof: Quota IdtyQuota (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Session KeyOwner (r:0 w:4)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn do_remove_identity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1525`
		//  Estimated: `6192`
		// Minimum execution time: 57_854_000 picoseconds.
		Weight::from_parts(60_976_000, 0)
			.saturating_add(Weight::from_parts(0, 6192))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(22))
	}
	/// Storage: Identity IdentitiesRemovableOn (r:1 w:0)
	/// Proof Skipped: Identity IdentitiesRemovableOn (max_values: None, max_size: None, mode: Measured)
	fn prune_identities_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `108`
		//  Estimated: `3573`
		// Minimum execution time: 1_249_000 picoseconds.
		Weight::from_parts(1_330_000, 0)
			.saturating_add(Weight::from_parts(0, 3573))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: Identity IdentitiesRemovableOn (r:1 w:1)
	/// Proof Skipped: Identity IdentitiesRemovableOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn prune_identities_none() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `293`
		//  Estimated: `3758`
		// Minimum execution time: 4_441_000 picoseconds.
		Weight::from_parts(4_690_000, 0)
			.saturating_add(Weight::from_parts(0, 3758))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentitiesRemovableOn (r:1 w:1)
	/// Proof Skipped: Identity IdentitiesRemovableOn (max_values: None, max_size: None, mode: Measured)
	/// Storage: Identity Identities (r:1 w:0)
	/// Proof Skipped: Identity Identities (max_values: None, max_size: None, mode: Measured)
	fn prune_identities_err() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360`
		//  Estimated: `3825`
		// Minimum execution time: 5_680_000 picoseconds.
		Weight::from_parts(6_083_000, 0)
			.saturating_add(Weight::from_parts(0, 3825))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
