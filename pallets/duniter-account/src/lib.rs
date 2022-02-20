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

#![cfg_attr(not(feature = "std"), no_std)]

mod types;

pub use pallet::*;
pub use types::*;

use frame_support::pallet_prelude::*;
use frame_support::traits::{OnUnbalanced, StoredMap};
use frame_system::pallet_prelude::*;
use pallet_provide_randomness::RequestId;
use sp_core::H256;
use sp_runtime::traits::{Convert, Saturating, Zero};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::{Currency, ExistenceRequirement, StorageVersion};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // CONFIG //

    #[pallet::config]
    pub trait Config:
        frame_system::Config<AccountData = AccountData<Self::Balance>>
        + pallet_balances::Config
        + pallet_provide_randomness::Config<Currency = pallet_balances::Pallet<Self>>
    {
        type AccountIdToSalt: Convert<Self::AccountId, [u8; 32]>;
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type MaxNewAccountsPerBlock: Get<u32>;
        type NewAccountPrice: Get<Self::Balance>;
    }

    // STORAGE //

    #[pallet::storage]
    #[pallet::getter(fn pending_random_id_assignments)]
    pub type PendingRandomIdAssignments<T: Config> =
        StorageMap<_, Twox64Concat, RequestId, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn pending_new_accounts)]
    pub type PendingNewAccounts<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (), OptionQuery>;

    // GENESIS STUFF //

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub accounts: sp_std::collections::btree_set::BTreeSet<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                accounts: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for account_id in &self.accounts {
                PendingNewAccounts::<T>::insert(account_id.clone(), ());
            }
        }
    }

    // EVENTS //

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Random id assigned
        /// [account_id, random_id]
        RandomIdAssigned {
            account_id: T::AccountId,
            random_id: H256,
        },
    }

    // HOOKS //
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: T::BlockNumber) -> Weight {
            let mut total_weight = 0;
            for account_id in PendingNewAccounts::<T>::iter_keys()
                .drain()
                .take(T::MaxNewAccountsPerBlock::get() as usize)
            {
                if frame_system::Pallet::<T>::sufficients(&account_id) > 0 {
                    // If the account is self-sufficient, it is exempt from account creation fees
                    let request_id = pallet_provide_randomness::Pallet::<T>::force_request(
                        pallet_provide_randomness::RandomnessType::RandomnessFromTwoEpochsAgo,
                        H256(T::AccountIdToSalt::convert(account_id.clone())),
                    );
                    PendingRandomIdAssignments::<T>::insert(request_id, account_id);
                    total_weight += 100_000;
                } else {
                    // If the account is not self-sufficient, it must pay the account creation fees
                    frame_system::Pallet::<T>::inc_providers(&account_id);
                    let res = T::Currency::withdraw(
                        &account_id,
                        T::NewAccountPrice::get(),
                        frame_support::traits::WithdrawReasons::FEE,
                        ExistenceRequirement::KeepAlive,
                    );
                    if let Ok(imbalance) = res {
                        // The fees have been collected, we handle the collected amount and we
                        // request the random id
                        T::OnUnbalanced::on_unbalanced(imbalance);
                        let request_id = pallet_provide_randomness::Pallet::<T>::force_request(
                            pallet_provide_randomness::RandomnessType::RandomnessFromTwoEpochsAgo,
                            H256(T::AccountIdToSalt::convert(account_id.clone())),
                        );
                        PendingRandomIdAssignments::<T>::insert(request_id, account_id);
                        total_weight += 200_000;
                    } else {
                        // The charges could not be deducted, we slash the account
                        let account_data = frame_system::Pallet::<T>::get(&account_id);
                        let (imbalance, rest) = pallet_balances::Pallet::<T>::slash(
                            &account_id,
                            account_data.free.saturating_add(account_data.reserved),
                        );
                        debug_assert!(rest.is_zero());
                        T::OnUnbalanced::on_unbalanced(imbalance);
                        total_weight += 300_000;
                    }
                }
            }
            total_weight
        }
    }
}

impl<T> pallet_provide_randomness::OnFilledRandomness for Pallet<T>
where
    T: Config,
{
    fn on_filled_randomness(request_id: RequestId, randomness: H256) -> Weight {
        if let Some(account_id) = PendingRandomIdAssignments::<T>::take(request_id) {
            // Can only fail if the account has been deleted in the meantime,
            // but this case does not require any processing
            let res = frame_system::Pallet::<T>::mutate_exists(&account_id, |maybe_account_data| {
                if let Some(ref mut account_data) = maybe_account_data {
                    account_data.random_id = Some(randomness);
                }
            });
            if res.is_ok() {
                Self::deposit_event(Event::RandomIdAssigned {
                    account_id,
                    random_id: randomness,
                });
            }
            200_000
        } else {
            100_000
        }
    }
}

impl<T, AccountId, Balance>
    frame_support::traits::StoredMap<AccountId, pallet_balances::AccountData<Balance>> for Pallet<T>
where
    AccountId: Parameter
        + Member
        + MaybeSerializeDeserialize
        + core::fmt::Debug
        + sp_runtime::traits::MaybeDisplay
        + Ord
        + Into<[u8; 32]>
        + codec::MaxEncodedLen,
    Balance: Parameter
        + Member
        + sp_runtime::traits::AtLeast32BitUnsigned
        + codec::Codec
        + Default
        + Copy
        + MaybeSerializeDeserialize
        + core::fmt::Debug
        + codec::MaxEncodedLen
        + scale_info::TypeInfo,
    T: Config
        + frame_system::Config<AccountId = AccountId, AccountData = AccountData<Balance>>
        + pallet_balances::Config<Balance = Balance>
        + pallet_provide_randomness::Config,
{
    fn get(k: &AccountId) -> pallet_balances::AccountData<Balance> {
        frame_system::Account::<T>::get(k).data.into()
    }

    fn try_mutate_exists<R, E: From<sp_runtime::DispatchError>>(
        account_id: &AccountId,
        f: impl FnOnce(&mut Option<pallet_balances::AccountData<Balance>>) -> Result<R, E>,
    ) -> Result<R, E> {
        let account = frame_system::Account::<T>::get(account_id);
        let was_providing = account.data != Default::default();
        let mut some_data = if was_providing {
            Some(account.data.into())
        } else {
            None
        };
        let result = f(&mut some_data)?;
        let is_providing = some_data.is_some();
        if !was_providing && is_providing {
            // If the account does not exist, we should program its creation
            if !frame_system::Pallet::<T>::account_exists(account_id) {
                PendingNewAccounts::<T>::insert(account_id, ());
            }
        } else if was_providing && !is_providing {
            match frame_system::Pallet::<T>::dec_providers(account_id)? {
                frame_system::DecRefStatus::Reaped => return Ok(result),
                frame_system::DecRefStatus::Exists => {
                    // Update value as normal...
                }
            }
        } else if !was_providing && !is_providing {
            return Ok(result);
        }
        frame_system::Account::<T>::mutate(account_id, |a| {
            a.data.set_balances(some_data.unwrap_or_default())
        });
        Ok(result)
    }
}
