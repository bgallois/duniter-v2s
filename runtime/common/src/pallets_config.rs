// Copyright 2021 Axiom-Team
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
            type RuntimeCall = RuntimeCall;
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
            type RuntimeEvent = RuntimeEvent;
            /// The ubiquitous origin type.
            type RuntimeOrigin = RuntimeOrigin;
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
            type AccountData = pallet_duniter_account::AccountData<Balance, IdtyIndex>;
            /// Weight information for the extrinsics of this pallet.
            type SystemWeightInfo = common_runtime::weights::frame_system::WeightInfo<Runtime>;
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
            pub const NoPreimagePostponement: Option<u32> = Some(10);
        }
        impl pallet_scheduler::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeOrigin = RuntimeOrigin;
            type PalletsOrigin = OriginCaller;
            type RuntimeCall = RuntimeCall;
            type MaximumWeight = MaximumSchedulerWeight;
            type ScheduleOrigin = EnsureRoot<AccountId>;
            type OriginPrivilegeCmp = EqualPrivilegeOnly;
            type MaxScheduledPerBlock = MaxScheduledPerBlock;
            type WeightInfo = common_runtime::weights::pallet_scheduler::WeightInfo<Runtime>;
            type Preimages = Preimage;
        }

        // ACCOUNT //

        impl pallet_duniter_account::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type AccountIdToSalt = sp_runtime::traits::ConvertInto;
            type MaxNewAccountsPerBlock = frame_support::pallet_prelude::ConstU32<1>;
            type NewAccountPrice = frame_support::traits::ConstU64<300>;
            type WeightInfo = common_runtime::weights::pallet_duniter_account::WeightInfo<Runtime>;
            // does currency adapter in any case, but adds "refund with quota" feature
            type InnerOnChargeTransaction = CurrencyAdapter<Balances, HandleFees>;
            type Refund = Quota;
        }

        // QUOTA //
        pub struct TreasuryAccountId;
        impl frame_support::pallet_prelude::Get<AccountId> for TreasuryAccountId {
            fn get() -> AccountId {
                // TODO optimize: make this a constant
                // calling Treasury.account_id() actually requires computation
                Treasury::account_id()
            }
        }
        parameter_types! {
            pub const ReloadRate: BlockNumber = 1 * HOURS; // faster than DAYS
            pub const MaxQuota: Balance = 1000; // 10 ĞD
        }
        impl pallet_quota::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            // type IdtyId = IdtyIndex;
            type ReloadRate = ReloadRate;
            type MaxQuota = MaxQuota;
            type RefundAccount = TreasuryAccountId;
            type WeightInfo = common_runtime::weights::pallet_quota::WeightInfo<Runtime>;
        }

        // BLOCK CREATION //

        impl pallet_babe::Config for Runtime {
            type EpochDuration = EpochDuration;
            type ExpectedBlockTime = ExpectedBlockTime;
            // session module is the trigger
            type EpochChangeTrigger = pallet_babe::ExternalTrigger;
            type DisabledValidators = Session;
            type KeyOwnerProof = <Historical as KeyOwnerProofSystem<(
                KeyTypeId,
                pallet_babe::AuthorityId,
            )>>::Proof;
            type EquivocationReportSystem =
                pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
            type WeightInfo = common_runtime::weights::pallet_babe::WeightInfo<Runtime>;
            type MaxAuthorities = MaxAuthorities;
        }

        impl pallet_timestamp::Config for Runtime {
            type Moment = u64;
            type OnTimestampSet = (Babe, UniversalDividend);
            type MinimumPeriod = MinimumPeriod;
            type WeightInfo = common_runtime::weights::pallet_timestamp::WeightInfo<Runtime>;
        }

        // MONEY MANAGEMENT //

        impl pallet_balances::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type MaxLocks = MaxLocks;
            type MaxReserves = frame_support::pallet_prelude::ConstU32<5>;
            type ReserveIdentifier = [u8; 8];
            /// The type for recording an account's balance.
            type Balance = Balance;
            type DustRemoval = HandleDust;
            type ExistentialDeposit = ExistentialDeposit;
            type AccountStore = Account;
			type HoldIdentifier = ();
			type FreezeIdentifier = ();
			type MaxHolds = frame_support::pallet_prelude::ConstU32<0>;
			type MaxFreezes = frame_support::pallet_prelude::ConstU32<0>;
            type WeightInfo = common_runtime::weights::pallet_balances::WeightInfo<Runtime>;
        }

        // Take Dust from Balances and put it in the Treasury pot
        pub struct HandleDust;
        type CreditOf = frame_support::traits::tokens::fungible::Credit<AccountId, Balances>;
        impl frame_support::traits::OnUnbalanced<CreditOf> for HandleDust {
            fn on_nonzero_unbalanced(amount: CreditOf) {
                use frame_support::traits::Currency as _;
                use frame_support::traits::Imbalance as _;
                let imbalance = NegativeImbalance::new(amount.peek());
                Balances::resolve_creating(&Treasury::account_id(), imbalance);
            }
        }

        // fees are moved to the treasury
        pub struct HandleFees;
        type NegativeImbalance = <Balances as frame_support::traits::Currency<AccountId>>::NegativeImbalance;
        impl frame_support::traits::OnUnbalanced<NegativeImbalance> for HandleFees {
            fn on_nonzero_unbalanced(amount: NegativeImbalance) {
                use frame_support::traits::Currency as _;

                // fee is moved to treasury
                Balances::resolve_creating(&Treasury::account_id(), amount);
                // should move the tip to author
                // if let Some(author) = Authorship::author() {
                //     Balances::resolve_creating(&author, amount);
                // }
            }
        }
        pub struct OnChargeTransaction;

        parameter_types! {
            pub FeeMultiplier: pallet_transaction_payment::Multiplier = pallet_transaction_payment::Multiplier::one();
        }
        impl pallet_transaction_payment::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            // does a filter on the call
            type OnChargeTransaction = OneshotAccount;
            type OperationalFeeMultiplier = frame_support::traits::ConstU8<5>;
            type WeightToFee = common_runtime::fees::WeightToFeeImpl<Balance>;
            type LengthToFee = common_runtime::fees::LengthToFeeImpl<Balance>;
            type FeeMultiplierUpdate = pallet_transaction_payment::ConstFeeMultiplier<FeeMultiplier>;
        }
        impl pallet_oneshot_account::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            // when call is not oneshot account, fall back to duniter-account implementation
            type InnerOnChargeTransaction = Account;
            type WeightInfo = common_runtime::weights::pallet_oneshot_account::WeightInfo<Runtime>;
        }

        // CONSENSUS  //

        impl pallet_authority_discovery::Config for Runtime {
            type MaxAuthorities = MaxAuthorities;
        }
        impl pallet_authority_members::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type IsMember = SmithMembership;
            type OnNewSession = OnNewSessionHandler<Runtime>;
            type OnRemovedMember = OnRemovedAuthorityMemberHandler<Runtime>;
            type MemberId = IdtyIndex;
            type MemberIdOf = common_runtime::providers::IdentityIndexOf<Self>;
            type MaxAuthorities = MaxAuthorities;
            type RemoveMemberOrigin = EnsureRoot<Self::AccountId>;
			type WeightInfo = common_runtime::weights::pallet_authority_members::WeightInfo<Runtime>;
        }
        impl pallet_authorship::Config for Runtime {
            type EventHandler = ImOnline;
            type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
        }
        impl pallet_im_online::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type AuthorityId = ImOnlineId;
            type ValidatorSet = Historical;
            type NextSessionRotation = Babe;
            type ReportUnresponsiveness = Offences;
            type UnsignedPriority = ImOnlineUnsignedPriority;
            type WeightInfo = common_runtime::weights::pallet_im_online::WeightInfo<Runtime>;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type MaxKeys = MaxAuthorities;
            #[cfg(feature = "runtime-benchmarks")]
            type MaxKeys = frame_support::traits::ConstU32<1_000>; // At least 1000 to be benchmarkable see https://github.com/paritytech/substrate/blob/e94cb0dafd4f30ff29512c1c00ec513ada7d2b5d/frame/im-online/src/benchmarking.rs#L35
            type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
            type MaxPeerDataEncodingSize = MaxPeerDataEncodingSize;
        }
        impl pallet_offences::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
            type OnOffenceHandler = AuthorityMembers;
        }
        impl pallet_session::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type ValidatorId = AccountId;
            type ValidatorIdOf = sp_runtime::traits::ConvertInto;
            type ShouldEndSession = Babe;
            type NextSessionRotation = Babe;
            type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, AuthorityMembers>;
            type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
            type Keys = opaque::SessionKeys;
            type WeightInfo = common_runtime::weights::pallet_session::WeightInfo<Runtime>;
        }
        impl pallet_session::historical::Config for Runtime {
            type FullIdentification = ValidatorFullIdentification;
            type FullIdentificationOf = FullIdentificationOfImpl;
        }
        impl pallet_grandpa::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type KeyOwnerProof =
                <Historical as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
            type EquivocationReportSystem =
            		pallet_grandpa::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
            type WeightInfo = common_runtime::weights::pallet_grandpa::WeightInfo<Runtime>;
            type MaxAuthorities = MaxAuthorities;
			type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
		}
        parameter_types! {
            // BondingDuration::get() * SessionsPerEra::get();
            pub const MaxSetIdSessionEntries: u32 = 1000;
        }

        // ONCHAIN GOVERNANCE //

		#[cfg(feature = "runtime-benchmarks")]
		parameter_types! {
			pub const WorstCaseOrigin: pallet_collective::RawOrigin<AccountId, TechnicalCommitteeInstance> =
				pallet_collective::RawOrigin::<AccountId, TechnicalCommitteeInstance>::Members(2, 3);
		}

		impl pallet_upgrade_origin::Config for Runtime {
			type RuntimeEvent = RuntimeEvent;
			type Call = RuntimeCall;
			type UpgradableOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 2, 3>;
			type WeightInfo = common_runtime::weights::pallet_upgrade_origin::WeightInfo<Runtime>;
			#[cfg(feature = "runtime-benchmarks")]
			type WorstCaseOriginType = pallet_collective::RawOrigin<AccountId, TechnicalCommitteeInstance>;
			#[cfg(feature = "runtime-benchmarks")]
			type WorstCaseOrigin = WorstCaseOrigin;
		}

        parameter_types! {
            pub const PreimageMaxSize: u32 = 4096 * 1024;
            pub const PreimageBaseDeposit: Balance = deposit(2, 64);
            pub const PreimageByteDeposit: Balance = deposit(0, 1);
        }

        impl pallet_preimage::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_preimage::WeightInfo<Runtime>;
            type Currency = Balances;
            type ManagerOrigin = EnsureRoot<AccountId>;
            type BaseDeposit = PreimageBaseDeposit;
            type ByteDeposit = PreimageByteDeposit;
        }

        // UTILITIES //

        impl pallet_atomic_swap::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type SwapAction = pallet_atomic_swap::BalanceSwapAction<AccountId, Balances>;
            type ProofLimit = frame_support::traits::ConstU32<1_024>;
        }

        impl pallet_provide_randomness::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type GetCurrentEpochIndex = GetCurrentEpochIndex<Self>;
            type MaxRequests = frame_support::traits::ConstU32<100>;
            type RequestPrice = frame_support::traits::ConstU64<2_000>;
            type OnFilledRandomness = Account;
            type OnUnbalanced = Treasury;
            type ParentBlockRandomness = pallet_babe::ParentBlockRandomness<Self>;
            type RandomnessFromOneEpochAgo = pallet_babe::RandomnessFromOneEpochAgo<Self>;
			type WeightInfo = common_runtime::weights::pallet_provide_randomness::WeightInfo<Runtime>;
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
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type Currency = Balances;
            type ProxyType = ProxyType;
            type ProxyDepositBase = ProxyDepositBase;
            type ProxyDepositFactor = ProxyDepositFactor;
            type MaxProxies = frame_support::traits::ConstU32<32>;
            type MaxPending = frame_support::traits::ConstU32<32>;
            type CallHasher = BlakeTwo256;
            type AnnouncementDepositBase = AnnouncementDepositBase;
            type AnnouncementDepositFactor = AnnouncementDepositFactor;
            type WeightInfo = common_runtime::weights::pallet_proxy::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const DepositBase: Balance = DEPOSIT_PER_ITEM;
            pub const DepositFactor: Balance = DEPOSIT_PER_BYTE * 32;
        }
        impl pallet_multisig::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type Currency = Balances;
            type DepositBase = DepositBase;
            type DepositFactor = DepositFactor;
            type MaxSignatories = MaxSignatories;
            type WeightInfo = common_runtime::weights::pallet_multisig::WeightInfo<Runtime>;
        }

        impl pallet_utility::Config for Runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type PalletsOrigin = OriginCaller;
            type WeightInfo = common_runtime::weights::pallet_utility::WeightInfo<Runtime>;
        }

        parameter_types! {
            pub const Burn: Permill = Permill::zero();
            pub const ProposalBond: Permill = Permill::from_percent(1);
            pub const ProposalBondMaximum: Option<Balance> = None;
            pub const SpendPeriod: BlockNumber = DAYS;
            // Treasury account address:
            // gdev/gtest: 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
            pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
        }
        impl pallet_treasury::Config for Runtime {
            type ApproveOrigin = TreasuryApproveOrigin;
            type Burn = Burn;
            type BurnDestination = ();
            type Currency = Balances;
            type RuntimeEvent = RuntimeEvent;
            type OnSlash = Treasury;
            type ProposalBond = ProposalBond;
            type ProposalBondMinimum = frame_support::traits::ConstU64<10_000>;
            type ProposalBondMaximum = ProposalBondMaximum;
            type MaxApprovals = frame_support::traits::ConstU32<100>;
            type PalletId = TreasuryPalletId;
            type RejectOrigin = TreasuryRejectOrigin;
            type SpendFunds = TreasurySpendFunds<Self>;
            type SpendPeriod = SpendPeriod;
            type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u64>;
			type WeightInfo = common_runtime::weights::pallet_treasury::WeightInfo<Runtime>;
        }

        // UNIVERSAL DIVIDEND //

		pub struct MembersCount;
		impl frame_support::pallet_prelude::Get<Balance> for MembersCount {
			fn get() -> Balance {
				<Membership as sp_membership::traits::MembersCount>::members_count() as Balance
			}
		}

        impl pallet_universal_dividend::Config for Runtime {
            type MomentIntoBalance = sp_runtime::traits::ConvertInto;
            type Currency = Balances;
            type RuntimeEvent = RuntimeEvent;
			type MaxPastReeval = frame_support::traits::ConstU32<160>;
            type MembersCount = MembersCount;
            type MembersStorage = common_runtime::providers::UdMembersStorage<Runtime>;
			type MembersStorageIter = common_runtime::providers::UdMembersStorageIter<Runtime>;
            type SquareMoneyGrowthRate = SquareMoneyGrowthRate;
            type UdCreationPeriod = UdCreationPeriod;
            type UdReevalPeriod = UdReevalPeriod;
            type UnitsPerUd = frame_support::traits::ConstU64<1_000>;
			type WeightInfo = common_runtime::weights::pallet_universal_dividend::WeightInfo<Runtime>;
            #[cfg(feature = "runtime-benchmarks")]
            type AccountIdOf = common_runtime::providers::IdentityAccountIdProvider<Self>;
        }

        // WEB OF TRUST //

        use frame_support::instances::Instance1;
        impl pallet_duniter_wot::Config<Instance1> for Runtime {
            type FirstIssuableOn = WotFirstCertIssuableOn;
            type IsDistanceOk = common_runtime::providers::MainWotIsDistanceOk<Runtime>;
            type IsSubWot = frame_support::traits::ConstBool<false>;
            type MinCertForMembership = WotMinCertForMembership;
            type MinCertForCreateIdtyRight = WotMinCertForCreateIdtyRight;
        }

        impl pallet_identity::Config for Runtime {
			type ChangeOwnerKeyPeriod = ChangeOwnerKeyPeriod;
            type ConfirmPeriod = ConfirmPeriod;
            type CheckIdtyCallAllowed = (Wot, SmithSubWot);
            type IdtyCreationPeriod = IdtyCreationPeriod;
			type IdtyData = IdtyData;
            type IdtyIndex = IdtyIndex;
            type AccountLinker = Account;
            type IdtyNameValidator = IdtyNameValidatorImpl;
            type IdtyRemovalOtherReason = pallet_duniter_wot::IdtyRemovalWotReason;
            type Signer = <Signature as sp_runtime::traits::Verify>::Signer;
			type Signature = Signature;
            type OnIdtyChange = (common_runtime::handlers::OnIdtyChangeHandler<Runtime>, Wot, Quota, Account);
            type RemoveIdentityConsumers = RemoveIdentityConsumersImpl<Self>;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_identity::WeightInfo<Runtime>;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkSetupHandler = common_runtime::providers::BenchmarkSetupHandler<Runtime>;
        }

        impl pallet_membership::Config<frame_support::instances::Instance1> for Runtime {
            type CheckMembershipCallAllowed = Wot;
            type IdtyId = IdtyIndex;
            type IdtyIdOf = common_runtime::providers::IdentityIndexOf<Self>;
            type AccountIdOf = common_runtime::providers::IdentityAccountIdProvider<Self>;
            type MembershipPeriod = MembershipPeriod;
            type OnEvent = OnMembershipEventHandler<Wot, Runtime>;
            type PendingMembershipPeriod = PendingMembershipPeriod;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_membership_membership::WeightInfo<Runtime>;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkSetupHandler = common_runtime::providers::BenchmarkSetupHandler<Runtime>;
        }

        impl pallet_certification::Config<Instance1> for Runtime {
            type CertPeriod = CertPeriod;
            type IdtyIndex = IdtyIndex;
            type OwnerKeyOf = Identity;
            type CheckCertAllowed = Wot;
            type MaxByIssuer = MaxByIssuer;
            type MinReceivedCertToBeAbleToIssueCert = MinReceivedCertToBeAbleToIssueCert;
            type OnNewcert = Wot;
            type OnRemovedCert = Wot;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_certification_cert::WeightInfo<Runtime>;
            type ValidityPeriod = ValidityPeriod;
        }
        parameter_types! {
            pub const MinAccessibleReferees: Perbill = Perbill::from_percent(80);
        }
        impl pallet_distance::Config for Runtime {
            type Currency = Balances;
            type EvaluationPrice = frame_support::traits::ConstU64<1000>;
            type MinAccessibleReferees = MinAccessibleReferees;
            type ResultExpiration = frame_support::traits::ConstU32<720>;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_distance::WeightInfo<Runtime>;
        }

        // SMITHS SUB-WOT //

        use frame_support::instances::Instance2;
        impl pallet_duniter_wot::Config<Instance2> for Runtime {
            type FirstIssuableOn = SmithWotFirstCertIssuableOn;
            type IsDistanceOk = pallet_duniter_wot::traits::DistanceAlwaysOk;
            type IsSubWot = frame_support::traits::ConstBool<true>;
            type MinCertForMembership = SmithWotMinCertForMembership;
            type MinCertForCreateIdtyRight = frame_support::traits::ConstU32<0>;
        }

        impl pallet_membership::Config<Instance2> for Runtime {
            type CheckMembershipCallAllowed = SmithSubWot;
            type IdtyId = IdtyIndex;
            type IdtyIdOf = common_runtime::providers::IdentityIndexOf<Self>;
            type AccountIdOf = common_runtime::providers::IdentityAccountIdProvider<Self>;
            type MembershipPeriod = SmithMembershipPeriod;
            type OnEvent = OnSmithMembershipEventHandler<SmithSubWot, Runtime>;
            type PendingMembershipPeriod = SmithPendingMembershipPeriod;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_membership_smith_membership::WeightInfo<Runtime>;
            #[cfg(feature = "runtime-benchmarks")]
            type BenchmarkSetupHandler = common_runtime::providers::BenchmarkSetupHandler<Runtime>;
        }

        impl pallet_certification::Config<Instance2> for Runtime {
            type CertPeriod = SmithCertPeriod;
            type IdtyIndex = IdtyIndex;
            type OwnerKeyOf = Identity;
            type CheckCertAllowed = SmithSubWot;
            type MaxByIssuer = SmithMaxByIssuer;
            type MinReceivedCertToBeAbleToIssueCert = SmithMinReceivedCertToBeAbleToIssueCert;
            type OnNewcert = SmithSubWot;
            type OnRemovedCert = SmithSubWot;
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = common_runtime::weights::pallet_certification_smith_cert::WeightInfo<Runtime>;
            type ValidityPeriod = SmithValidityPeriod;
        }

        pub struct TechnicalCommitteeDefaultVote;
        impl pallet_collective::DefaultVote for TechnicalCommitteeDefaultVote {
            fn default_vote(
                _prime_vote: Option<bool>,
                _yes_votes: u32,
                _no_votes: u32,
                _len: u32,
            ) -> bool {
                false
            }
        }
        parameter_types! {
            pub const TechnicalCommitteeMotionDuration: BlockNumber = 7 * DAYS;
            pub MaxProposalWeight: Weight = Perbill::from_percent(50) * BlockWeights::get().max_block;
        }
        impl pallet_collective::Config<Instance2> for Runtime {
            type RuntimeOrigin = RuntimeOrigin;
            type Proposal = RuntimeCall;
            type RuntimeEvent = RuntimeEvent;
            type MotionDuration = TechnicalCommitteeMotionDuration;
            type MaxProposals = frame_support::pallet_prelude::ConstU32<20>;
            type MaxMembers = frame_support::pallet_prelude::ConstU32<100>;
            type WeightInfo = common_runtime::weights::pallet_collective::WeightInfo<Runtime>;
            type SetMembersOrigin = EnsureRoot<AccountId>;
            type MaxProposalWeight = MaxProposalWeight;
            #[cfg(not(feature = "runtime-benchmarks"))]
            type DefaultVote = TechnicalCommitteeDefaultVote;
            #[cfg(feature = "runtime-benchmarks")]
            type DefaultVote = pallet_collective::PrimeDefaultVote; // Overwrite with a  default vote that can return `true` sometimes as it is necessary for benchmarking
        }
    };
}
