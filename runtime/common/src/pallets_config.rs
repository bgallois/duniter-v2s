// Copyright 2021 Axiom-Team
//
// This file is part of Substrate-Libre-Currency.
//
// Substrate-Libre-Currency is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Substrate-Libre-Currency is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Substrate-Libre-Currency. If not, see <https://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! pallets_config {
	{$($custom:tt)*} => {
		$($custom)*

		// SYSTEM //

		parameter_types! {
			pub const Version: RuntimeVersion = VERSION;
		}

		impl frame_system::Config for Runtime {
			/// The basic call filter to use in dispatchable.
			type BaseCallFilter = BaseCallFilter;
			/// Block & extrinsics weights: base values and limits.
			type BlockWeights = BlockWeights;
			/// The maximum length of a block (in bytes).
			type BlockLength = BlockLength;
			/// The identifier used to distinguish between accounts.
			type AccountId = AccountId;
			/// The aggregated dispatch type that is available for extrinsics.
			type Call = Call;
			/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
			type Lookup = AccountIdLookup<AccountId, ()>;
			/// The index type for storing how many extrinsics an account has signed.
			type Index = Index;
			/// The index type for blocks.
			type BlockNumber = BlockNumber;
			/// The type for hashing blocks and tries.
			type Hash = Hash;
			/// The hashing algorithm used.
			type Hashing = BlakeTwo256;
			/// The header type.
			type Header = generic::Header<BlockNumber, BlakeTwo256>;
			/// The ubiquitous event type.
			type Event = Event;
			/// The ubiquitous origin type.
			type Origin = Origin;
			/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
			type BlockHashCount = BlockHashCount;
			/// The weight of database operations that the runtime can invoke.
			type DbWeight = DbWeight;
			/// Version of the runtime.
			type Version = Version;
			/// Converts a module to the index of the module in `construct_runtime!`.
			///
			/// This type is being generated by `construct_runtime!`.
			type PalletInfo = PalletInfo;
			/// What to do if a new account is created.
			type OnNewAccount = ();
			/// What to do if an account is fully reaped from the system.
			type OnKilledAccount = ();
			/// The data to be stored in an account.
			type AccountData = pallet_duniter_account::AccountData<Balance>;
			/// Weight information for the extrinsics of this pallet.
			type SystemWeightInfo = ();
			/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
			type SS58Prefix = SS58Prefix;
			/// The set code logic, just the default since we're not a parachain.
			type OnSetCode = ();
			type MaxConsumers = frame_support::traits::ConstU32<16>;
		}

		// SCHEDULER //

		parameter_types! {
			pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
				BlockWeights::get().max_block;
			pub const MaxScheduledPerBlock: u32 = 50;
		}
		/// Used the compare the privilege of an origin inside the scheduler.
		pub struct OriginPrivilegeCmp;
		impl frame_support::traits::PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
			fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<core::cmp::Ordering> {
				if left == right {
					Some(core::cmp::Ordering::Equal)
				} else {
					None
				}
			}
		}
		impl pallet_scheduler::Config for Runtime {
			type Event = Event;
			type Origin = Origin;
			type PalletsOrigin = OriginCaller;
			type Call = Call;
			type MaximumWeight = MaximumSchedulerWeight;
			type ScheduleOrigin = frame_system::EnsureSigned<AccountId>;
			type OriginPrivilegeCmp = OriginPrivilegeCmp;
			type MaxScheduledPerBlock = MaxScheduledPerBlock;
			type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
			type PreimageProvider = ();
			type NoPreimagePostponement = ();
		}

		// ACCOUNT //

		impl pallet_duniter_account::Config for Runtime {
			type AccountIdToSalt = sp_runtime::traits::ConvertInto;
			type Event = Event;
			type MaxNewAccountsPerBlock = frame_support::pallet_prelude::ConstU32<100>;
			type NewAccountPrice = frame_support::traits::ConstU64<200>;
		}

		// BLOCK CREATION //

		impl pallet_babe::Config for Runtime {
			type EpochDuration = EpochDuration;
			type ExpectedBlockTime = ExpectedBlockTime;

			// session module is the trigger
			type EpochChangeTrigger = pallet_babe::ExternalTrigger;

			type DisabledValidators = Session;

			type KeyOwnerProofSystem = Historical;

			type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
				KeyTypeId,
				pallet_babe::AuthorityId,
			)>>::Proof;

			type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
				KeyTypeId,
				pallet_babe::AuthorityId,
			)>>::IdentificationTuple;

			type HandleEquivocation =
				pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;

			type WeightInfo = ();

			type MaxAuthorities = MaxAuthorities;
		}

		impl pallet_timestamp::Config for Runtime {
			type Moment = u64;
			type OnTimestampSet = Babe;
			type MinimumPeriod = MinimumPeriod;
			type WeightInfo = ();
		}

		// MONEY MANAGEMENT //

		impl pallet_balances::Config for Runtime {
			type MaxLocks = MaxLocks;
			type MaxReserves = frame_support::pallet_prelude::ConstU32<5>;
			type ReserveIdentifier = [u8; 8];
			/// The type for recording an account's balance.
			type Balance = Balance;
			/// The ubiquitous event type.
			type Event = Event;
			type DustRemoval = ();
			type ExistentialDeposit = ExistentialDeposit;
			type AccountStore = Account;
			type WeightInfo = common_runtime::weights::pallet_balances::WeightInfo<Runtime>;
		}

		pub struct HandleFees;
		type NegativeImbalance = <Balances as frame_support::traits::Currency<AccountId>>::NegativeImbalance;
		impl frame_support::traits::OnUnbalanced<NegativeImbalance> for HandleFees {
			fn on_nonzero_unbalanced(amount: NegativeImbalance) {
				use frame_support::traits::Currency as _;

				if let Some(author) = Authorship::author() {
					Balances::resolve_creating(&author, amount);
				}
			}
		}
		impl pallet_transaction_payment::Config for Runtime {
			type OnChargeTransaction = CurrencyAdapter<Balances, HandleFees>;
			type TransactionByteFee = TransactionByteFee;
			type OperationalFeeMultiplier = frame_support::traits::ConstU8<5>;
			type WeightToFee = common_runtime::fees::WeightToFeeImpl<Balance>;
			type FeeMultiplierUpdate = ();
		}

		// CONSENSUS  //

		impl pallet_authority_discovery::Config for Runtime {
			type MaxAuthorities = MaxAuthorities;
		}
		impl pallet_authority_members::Config for Runtime {
			type Event = Event;
			type KeysWrapper = opaque::SessionKeysWrapper;
			type IsMember = SmithsMembership;
			type OnNewSession = OnNewSessionHandler<Runtime>;
			type OnRemovedMember = OnRemovedAuthorityMemberHandler<Runtime>;
			type MemberId = IdtyIndex;
			type MemberIdOf = Identity;
			type MaxAuthorities = MaxAuthorities;
			type MaxKeysLife = frame_support::pallet_prelude::ConstU32<1_000>;
			type MaxOfflineSessions = frame_support::pallet_prelude::ConstU32<100>;
			type RemoveMemberOrigin = EnsureRoot<Self::AccountId>;
		}
		impl pallet_authorship::Config for Runtime {
			type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
			type UncleGenerations = UncleGenerations;
			type FilterUncle = ();
			type EventHandler = ImOnline;
		}
		impl pallet_im_online::Config for Runtime {
			type AuthorityId = ImOnlineId;
			type Event = Event;
			type ValidatorSet = Historical;
			type NextSessionRotation = Babe;
			type ReportUnresponsiveness = Offences;
			type UnsignedPriority = ImOnlineUnsignedPriority;
			type WeightInfo = ();
			type MaxKeys = MaxKeys;
			type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
			type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
		}
		impl pallet_offences::Config for Runtime {
			type Event = Event;
			type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
			type OnOffenceHandler = ();
		}
		impl pallet_session::Config for Runtime {
			type Event = Event;
			type ValidatorId = AccountId;
			type ValidatorIdOf = sp_runtime::traits::ConvertInto;
			type ShouldEndSession = Babe;
			type NextSessionRotation = Babe;
			type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, AuthorityMembers>;
			type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
			type Keys = opaque::SessionKeys;
			type WeightInfo = ();
		}
		impl pallet_session::historical::Config for Runtime {
			type FullIdentification = ValidatorFullIdentification;
			type FullIdentificationOf = FullIdentificationOfImpl;
		}
		impl pallet_grandpa::Config for Runtime {
			type Event = Event;
			type Call = Call;

			type KeyOwnerProofSystem = ();

			type KeyOwnerProof =
				<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

			type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
				KeyTypeId,
				GrandpaId,
			)>>::IdentificationTuple;

			type HandleEquivocation = ();

			type WeightInfo = ();

			type MaxAuthorities = MaxAuthorities;
		}

		// UTILITIES //

		impl pallet_atomic_swap::Config for Runtime {
			type Event = Event;
			type SwapAction = pallet_atomic_swap::BalanceSwapAction<AccountId, Balances>;
			type ProofLimit = frame_support::traits::ConstU32<255>;
		}

		impl pallet_provide_randomness::Config for Runtime {
			type Currency = Balances;
			type Event = Event;
			type MaxRequests = frame_support::traits::ConstU32<1_000>;
			type RequestPrice = frame_support::traits::ConstU64<200>;
			type OnFilledRandomness = Account;
			type OnUnbalanced = ();
			type CurrentBlockRandomness = pallet_babe::CurrentBlockRandomness<Self>;
			type RandomnessFromOneEpochAgo = pallet_babe::RandomnessFromOneEpochAgo<Self>;
		}

		parameter_types! {
			// One storage item; key size 32, value size 8; .
			pub const ProxyDepositBase: Balance = deposit(1, 8);
			// Additional storage item size of 33 bytes.
			pub const ProxyDepositFactor: Balance = deposit(0, 33);
			pub const AnnouncementDepositBase: Balance = deposit(1, 8);
			pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
		}
		impl pallet_proxy::Config for Runtime {
			type Event = Event;
			type Call = Call;
			type Currency = Balances;
			type ProxyType = ProxyType;
			type ProxyDepositBase = ProxyDepositBase;
			type ProxyDepositFactor = ProxyDepositFactor;
			type MaxProxies = frame_support::traits::ConstU32<32>;
			type MaxPending = frame_support::traits::ConstU32<32>;
			type CallHasher = BlakeTwo256;
			type AnnouncementDepositBase = AnnouncementDepositBase;
			type AnnouncementDepositFactor = AnnouncementDepositFactor;
			type WeightInfo = pallet_proxy::weights::SubstrateWeight<Self>;
		}

		impl pallet_multisig::Config for Runtime {
			type Event = Event;
			type Call = Call;
			type Currency = Balances;
			type DepositBase = DepositBase;
			type DepositFactor = DepositFactor;
			type MaxSignatories = MaxSignatories;
			type WeightInfo = pallet_multisig::weights::SubstrateWeight<Self>;
		}

		impl pallet_utility::Config for Runtime {
			type Event = Event;
			type Call = Call;
			type PalletsOrigin = OriginCaller;
			type WeightInfo = pallet_utility::weights::SubstrateWeight<Self>;
		}

		// UNIVERSAL DIVIDEND //

		impl pallet_universal_dividend::Config for Runtime {
			type Currency = pallet_balances::Pallet<Runtime>;
			type Event = Event;
			type MembersCount = common_runtime::providers::UdAccountsProvider<Runtime>;
			type MembersIds = common_runtime::providers::UdAccountsProvider<Runtime>;
			type SquareMoneyGrowthRate = SquareMoneyGrowthRate;
			type UdCreationPeriod = UdCreationPeriod;
			type UdFirstReeval = UdFirstReeval;
			type UdReevalPeriod = UdReevalPeriod;
			type UdReevalPeriodInBlocks = UdReevalPeriodInBlocks;
			type UnitsPerUd = frame_support::traits::ConstU64<1_000>;
		}

		impl pallet_ud_accounts_storage::Config for Runtime {}

		// WEB OF TRUST //

		use frame_support::instances::Instance1;
		impl pallet_duniter_wot::Config<Instance1> for Runtime {
			type FirstIssuableOn = WotFirstCertIssuableOn;
			type IsSubWot = frame_support::traits::ConstBool<false>;
			type MinCertForMembership = WotMinCertForMembership;
			type MinCertForCreateIdtyRight = WotMinCertForCreateIdtyRight;
		}

		impl pallet_identity::Config for Runtime {
			type ConfirmPeriod = ConfirmPeriod;
			type Event = Event;
			type EnsureIdtyCallAllowed = Wot;
			type IdtyCreationPeriod = IdtyCreationPeriod;
			type IdtyIndex = IdtyIndex;
			type IdtyNameValidator = IdtyNameValidatorImpl;
			type IdtyValidationOrigin = EnsureRoot<Self::AccountId>;
			type IsMember = Membership;
			type OnIdtyChange = Wot;
			type MaxDisabledPeriod = MaxDisabledPeriod;
			type RemoveIdentityConsumers = RemoveIdentityConsumersImpl<Self>;
		}

		impl pallet_membership::Config<frame_support::instances::Instance1> for Runtime {
			type IsIdtyAllowedToRenewMembership = Wot;
			type IsIdtyAllowedToRequestMembership = Wot;
			type Event = Event;
			type IdtyId = IdtyIndex;
			type IdtyIdOf = Identity;
			type MembershipPeriod = MembershipPeriod;
			type MetaData = pallet_duniter_wot::MembershipMetaData<AccountId>;
			type OnEvent = OnMembershipEventHandler<Wot, Runtime>;
			type PendingMembershipPeriod = PendingMembershipPeriod;
			type RenewablePeriod = RenewablePeriod;
			type RevocationPeriod = frame_support::traits::ConstU32<0>;
		}

		impl pallet_certification::Config<Instance1> for Runtime {
			type CertPeriod = CertPeriod;
			type Event = Event;
			type IdtyIndex = IdtyIndex;
			type IdtyIndexOf = Identity;
			type IsCertAllowed = Wot;
			type MaxByIssuer = MaxByIssuer;
			type MinReceivedCertToBeAbleToIssueCert = MinReceivedCertToBeAbleToIssueCert;
			type OnNewcert = Wot;
			type OnRemovedCert = Wot;
			type CertRenewablePeriod = CertRenewablePeriod;
			type ValidityPeriod = ValidityPeriod;
		}

		// SMITHS SUB-WOT //

		use frame_support::instances::Instance2;
		impl pallet_duniter_wot::Config<Instance2> for Runtime {
			type FirstIssuableOn = SmithsWotFirstCertIssuableOn;
			type IsSubWot = frame_support::traits::ConstBool<true>;
			type MinCertForMembership = SmithsWotMinCertForMembership;
			type MinCertForCreateIdtyRight = frame_support::traits::ConstU32<0>;
		}

		impl pallet_membership::Config<Instance2> for Runtime {
			type IsIdtyAllowedToRenewMembership = ();
			type IsIdtyAllowedToRequestMembership = ();
			type Event = Event;
			type IdtyId = IdtyIndex;
			type IdtyIdOf = Identity;
			type MembershipPeriod = SmithMembershipPeriod;
			type MetaData = SmithsMembershipMetaData<opaque::SessionKeysWrapper>;
			type OnEvent = OnSmithMembershipEventHandler<SmithsSubWot, Runtime>;
			type PendingMembershipPeriod = SmithPendingMembershipPeriod;
			type RenewablePeriod = SmithRenewablePeriod;
			type RevocationPeriod = frame_support::traits::ConstU32<0>;
		}

		impl pallet_certification::Config<Instance2> for Runtime {
			type CertPeriod = SmithCertPeriod;
			type Event = Event;
			type IdtyIndex = IdtyIndex;
			type IdtyIndexOf = Identity;
			type IsCertAllowed = SmithsSubWot;
			type MaxByIssuer = SmithMaxByIssuer;
			type MinReceivedCertToBeAbleToIssueCert = SmithMinReceivedCertToBeAbleToIssueCert;
			type OnNewcert = SmithsSubWot;
			type OnRemovedCert = SmithsSubWot;
			type CertRenewablePeriod = SmithCertRenewablePeriod;
			type ValidityPeriod = SmithValidityPeriod;
		}

		pub struct SmithMembersDefaultVote;
		impl pallet_collective::DefaultVote for SmithMembersDefaultVote {
			fn default_vote(
				_prime_vote: Option<bool>,
				_yes_votes: u32,
				_no_votes: u32,
				_len: u32,
			) -> bool {
				false
			}
		}
		pub struct SmithMembersStorage;
		impl sp_runtime::traits::IsMember<AccountId> for SmithMembersStorage {
			fn is_member(account_id: &AccountId) -> bool {
				use sp_runtime::traits::Convert as _;
				if let Some(idty_index) = Identity::convert(account_id.clone()) {
					pallet_membership::Pallet::<Runtime, Instance2>::is_member(&idty_index)
				} else {
					false
				}
			}
		}
		impl pallet_collective::MembersStorage<AccountId> for SmithMembersStorage {
			fn members_count() -> u32 {
				pallet_membership::Membership::<Runtime, Instance2>::count()
			}
		}
		impl pallet_collective::Config<Instance2> for Runtime {
			type Origin = Origin;
			type Proposal = Call;
			type Event = Event;
			type MotionDuration = frame_support::pallet_prelude::ConstU32<10_000>;
			type MaxProposals = frame_support::pallet_prelude::ConstU32<10>;
			type MaxMembers = frame_support::pallet_prelude::ConstU32<1_000>;
			type MembersStorage = SmithMembersStorage;
			type DefaultVote = SmithMembersDefaultVote;
			type WeightInfo = ();
		}
	};
}
