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

// this file is for balance transfer tests (split from integration_tests)

mod common;

use common::*;
use frame_support::{assert_noop, assert_ok};
use gdev_runtime::*;
use sp_keyring::AccountKeyring;

/// test currency transfer
/// (does not take fees into account because it's only calls, not extrinsics)
/// test final balance of Alice and Eve accounts
#[test]
fn test_transfer() {
    ExtBuilder::new(1, 3, 4)
        .with_initial_balances(vec![
            (AccountKeyring::Alice.to_account_id(), 10_000),
            (AccountKeyring::Eve.to_account_id(), 10_000),
        ])
        .build()
        .execute_with(|| {
            // Alice gives 500 to Eve
            assert_ok!(Balances::transfer_allow_death(
                frame_system::RawOrigin::Signed(AccountKeyring::Alice.to_account_id()).into(),
                AccountKeyring::Eve.to_account_id().into(),
                500
            ));
            // check amounts
            assert_eq!(
                Balances::free_balance(AccountKeyring::Alice.to_account_id()),
                10_000 - 500
            );
            assert_eq!(
                Balances::free_balance(AccountKeyring::Eve.to_account_id()),
                10_000 + 500
            );
        })
}

/// test balance transfer without enough balance
#[test]
fn test_transfer_not_enough() {
    ExtBuilder::new(1, 3, 4)
        .with_initial_balances(vec![(AccountKeyring::Alice.to_account_id(), 599)])
        .build()
        .execute_with(|| {
            run_to_block(1);
            assert_noop!(
                Balances::transfer_allow_death(
                    frame_system::RawOrigin::Signed(AccountKeyring::Alice.to_account_id()).into(),
                    AccountKeyring::Dave.to_account_id().into(),
                    500
                ),
                sp_runtime::TokenError::Frozen // frozen because trying to transfer more than liquid
            );
        })
}

/// test balance transfer without enough balance
/// in this case, it is the total issuance which cause problem, hence the arithmetic underflow
#[test]
fn test_transfer_underflow() {
    ExtBuilder::new(1, 3, 4)
        .with_initial_balances(vec![(AccountKeyring::Alice.to_account_id(), 400)])
        .build()
        .execute_with(|| {
            run_to_block(1);
            assert_eq!(Balances::total_issuance(), 400);
            assert_noop!(
                Balances::transfer_allow_death(
                    frame_system::RawOrigin::Signed(AccountKeyring::Alice.to_account_id()).into(),
                    AccountKeyring::Dave.to_account_id().into(),
                    500 // larger than total issuance
                ),
                sp_runtime::ArithmeticError::Underflow
            );
        })
}

/// test balance transfer without enough balance
/// can not transfer 500 when only 499 available
#[test]
fn test_transfer_funds_unavailable() {
    ExtBuilder::new(1, 3, 4)
        .with_initial_balances(vec![
            (AccountKeyring::Alice.to_account_id(), 499),
            (AccountKeyring::Bob.to_account_id(), 200),
        ])
        .build()
        .execute_with(|| {
            run_to_block(1);
            assert_noop!(
                Balances::transfer_allow_death(
                    frame_system::RawOrigin::Signed(AccountKeyring::Alice.to_account_id()).into(),
                    AccountKeyring::Dave.to_account_id().into(),
                    500
                ),
                sp_runtime::TokenError::FundsUnavailable
            );
        })
}
