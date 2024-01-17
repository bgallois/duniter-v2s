// Copyright 2023 Axiom-Team
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

use crate::*;
use frame_support::pallet_prelude::*;

pub trait OnValidDistanceStatus<T: Config> {
    /// Handler for valid distance evaluation
    fn on_valid_distance_status(idty_index: T::IdtyIndex);
}

impl<T: Config> OnValidDistanceStatus<T> for () {
    fn on_valid_distance_status(_idty_index: T::IdtyIndex) {}
}

pub trait CheckRequestDistanceEvaluation<T: Config> {
    fn check_request_distance_evaluation(idty_index: T::IdtyIndex) -> Result<(), DispatchError>;
}

impl<T: Config> CheckRequestDistanceEvaluation<T> for () {
    fn check_request_distance_evaluation(_idty_index: T::IdtyIndex) -> Result<(), DispatchError> {
        Ok(())
    }
}
