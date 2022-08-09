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
macro_rules! runtime_apis {
    {$($custom:tt)*} => {
        impl_runtime_apis! {
            $($custom)*

            impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
                fn authorities() -> Vec<sp_authority_discovery::AuthorityId> {
                    AuthorityDiscovery::authorities()
                }
            }

            impl sp_consensus_babe::BabeApi<Block> for Runtime {
                fn configuration() -> sp_consensus_babe::BabeGenesisConfiguration {
                    // The choice of `c` parameter (where `1 - c` represents the
                    // probability of a slot being empty), is done in accordance to the
                    // slot duration and expected target block time, for safely
                    // resisting network delays of maximum two seconds.
                    // <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
                    use frame_support::traits::Get as _;
                    sp_consensus_babe::BabeGenesisConfiguration {
                        slot_duration: Babe::slot_duration(),
                        epoch_length: EpochDuration::get(),
                        c: BABE_GENESIS_EPOCH_CONFIG.c,
                        genesis_authorities: Babe::authorities().to_vec(),
                        randomness: Babe::randomness(),
                        allowed_slots: BABE_GENESIS_EPOCH_CONFIG.allowed_slots,
                    }
                }

                fn current_epoch_start() -> sp_consensus_babe::Slot {
                    Babe::current_epoch_start()
                }

                fn current_epoch() -> sp_consensus_babe::Epoch {
                    Babe::current_epoch()
                }

                fn next_epoch() -> sp_consensus_babe::Epoch {
                    Babe::next_epoch()
                }

                fn generate_key_ownership_proof(
                    _slot: sp_consensus_babe::Slot,
                    authority_id: sp_consensus_babe::AuthorityId,
                ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
                    use codec::Encode;

                    Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
                        .map(|p| p.encode())
                        .map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
                }

                fn submit_report_equivocation_unsigned_extrinsic(
                    equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
                    key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
                ) -> Option<()> {
                    let key_owner_proof = key_owner_proof.decode()?;

                    Babe::submit_unsigned_equivocation_report(
                        equivocation_proof,
                        key_owner_proof,
                    )
                }
            }

            impl sp_api::Core<Block> for Runtime {
                fn version() -> RuntimeVersion {
                    VERSION
                }

                fn execute_block(block: Block) {
                    Executive::execute_block(block)
                }

                fn initialize_block(header: &<Block as BlockT>::Header) {
                    Executive::initialize_block(header)
                }
            }

            impl sp_api::Metadata<Block> for Runtime {
                fn metadata() -> OpaqueMetadata {
                    OpaqueMetadata::new(Runtime::metadata().into())
                }
            }

            impl sp_block_builder::BlockBuilder<Block> for Runtime {
                fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
                    Executive::apply_extrinsic(extrinsic)
                }

                fn finalize_block() -> <Block as BlockT>::Header {
                    Executive::finalize_block()
                }

                fn inherent_extrinsics(
                    data: sp_inherents::InherentData,
                ) -> Vec<<Block as BlockT>::Extrinsic> {
                    data.create_extrinsics()
                }

                fn check_inherents(
                    block: Block,
                    data: sp_inherents::InherentData,
                ) -> sp_inherents::CheckInherentsResult {
                    data.check_extrinsics(&block)
                }
            }

            impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
                fn validate_transaction(
                    source: TransactionSource,
                    tx: <Block as BlockT>::Extrinsic,
                    block_hash: <Block as BlockT>::Hash,
                ) -> TransactionValidity {
                    // Filtered calls should not enter the tx pool.
                    if !<Runtime as frame_system::Config>::BaseCallFilter::contains(&tx.function)
                    {
                        return sp_runtime::transaction_validity::InvalidTransaction::Call.into();
                    }
                    Executive::validate_transaction(source, tx, block_hash)
                }
            }

            impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
                fn offchain_worker(header: &<Block as BlockT>::Header) {
                    Executive::offchain_worker(header)
                }
            }

            impl sp_session::SessionKeys<Block> for Runtime {
                fn decode_session_keys(
                    encoded: Vec<u8>,
                ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
                    opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
                }

                fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
                    opaque::SessionKeys::generate(seed)
                }
            }

            impl fg_primitives::GrandpaApi<Block> for Runtime {
                fn grandpa_authorities() -> GrandpaAuthorityList {
                    Grandpa::grandpa_authorities()
                }

                fn current_set_id() -> fg_primitives::SetId {
                    Grandpa::current_set_id()
                }

                fn submit_report_equivocation_unsigned_extrinsic(
                    _equivocation_proof: fg_primitives::EquivocationProof<
                        <Block as BlockT>::Hash,
                        NumberFor<Block>,
                    >,
                    _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
                ) -> Option<()> {
                    None
                }

                fn generate_key_ownership_proof(
                    _set_id: fg_primitives::SetId,
                    _authority_id: GrandpaId,
                ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
                    // NOTE: this is the only implementation possible since we've
                    // defined our key owner proof type as a bottom type (i.e. a type
                    // with no values).
                    None
                }
            }

            impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
                fn account_nonce(account: AccountId) -> Index {
                    System::account_nonce(account)
                }
            }

            impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
            for Runtime {
                fn query_info(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
                    TransactionPayment::query_info(uxt, len)
                }

                fn query_fee_details(
                    uxt: <Block as BlockT>::Extrinsic,
                    len: u32,
                ) -> pallet_transaction_payment::FeeDetails<Balance> {
                    TransactionPayment::query_fee_details(uxt, len)
                }
            }

            #[cfg(feature = "try-runtime")]
            impl frame_try_runtime::TryRuntime<Block> for Runtime {
                fn on_runtime_upgrade() -> (Weight, Weight) {
                    log::info!("try-runtime::on_runtime_upgrade.");
                    let weight = Executive::try_runtime_upgrade().unwrap();
                    (weight, BlockWeights::get().max_block)
                }

                fn execute_block_no_check(block: Block) -> Weight {
                    Executive::execute_block_no_check(block)
                }
            }

            #[cfg(feature = "runtime-benchmarks")]
			impl frame_benchmarking::Benchmark<Block> for Runtime {
				fn benchmark_metadata(extra: bool) -> (
					Vec<frame_benchmarking::BenchmarkList>,
					Vec<frame_support::traits::StorageInfo>,
				) {
					use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
					use frame_support::traits::StorageInfoTrait;

					use frame_system_benchmarking::Pallet as SystemBench;
					use frame_benchmarking::baseline::Pallet as Baseline;

					let mut list = Vec::<BenchmarkList>::new();
                    list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
					list_benchmark!(list, extra, pallet_balances, Balances);
					list_benchmark!(list, extra, pallet_multisig, Multisig);
					list_benchmark!(list, extra, pallet_oneshot_account, OneshotAccount);
					list_benchmark!(list, extra, pallet_proxy, Proxy);
					list_benchmark!(list, extra, pallet_scheduler, Scheduler);
					list_benchmark!(list, extra, pallet_timestamp, Timestamp);
					list_benchmark!(list, extra, pallet_universal_dividend, UniversalDividend);
					list_benchmark!(list, extra, pallet_upgrade_origin, UpgradeOrigin);
					list_benchmark!(list, extra, pallet_utility, Utility);

					let storage_info = AllPalletsWithSystem::storage_info();
					return (list, storage_info)
				}

				fn dispatch_benchmark(
					config: frame_benchmarking::BenchmarkConfig
				) -> Result<
					Vec<frame_benchmarking::BenchmarkBatch>,
					sp_runtime::RuntimeString,
				> {
					use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};
					// Trying to add benchmarks directly to some pallets caused cyclic dependency issues.
					// To get around that, we separated the benchmarks into its own crate.
					use frame_system_benchmarking::Pallet as SystemBench;
					use frame_benchmarking::baseline::Pallet as Baseline;

					impl frame_system_benchmarking::Config for Runtime {}
					impl frame_benchmarking::baseline::Config for Runtime {}

					let whitelist: Vec<TrackedStorageKey> = vec![
						// Block Number
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
						// Total Issuance
						hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
						// Execution Phase
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
						// Event Count
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
						// System Events
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
						// Treasury Account
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da95ecffd7b6c0f78751baa9d281e0bfa3a6d6f646c70792f74727372790000000000000000000000000000000000000000").to_vec().into(),
					];

					let mut batches = Vec::<BenchmarkBatch>::new();
					let params = (&config, &whitelist);
					add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
					add_benchmark!(params, batches, pallet_balances, Balances);
					add_benchmark!(params, batches, pallet_multisig, Multisig);
					add_benchmark!(params, batches, pallet_oneshot_account, OneshotAccount);
					add_benchmark!(params, batches, pallet_proxy, Proxy);
					add_benchmark!(params, batches, pallet_scheduler, Scheduler);
					add_benchmark!(params, batches, pallet_timestamp, Timestamp);
					add_benchmark!(params, batches, pallet_universal_dividend, UniversalDividend);
					add_benchmark!(params, batches, pallet_upgrade_origin, UpgradeOrigin);
					add_benchmark!(params, batches, pallet_utility, Utility);

					if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
					Ok(batches)
				}
			}
        }
    };
}
