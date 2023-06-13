// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod rollback_operation;
pub use rollback_operation::*;

mod bits;
mod bytes;
mod serialize;
mod string;

use console::{network::prelude::*, types::Field};

/// Enum to represent the allowed set of Merkle tree operations.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FinalizeOperation<N: Network> {
    /// Initializes a new a mapping, (`mapping ID`).
    InitializeMapping(Field<N>),
    /// Inserts a key-value pair, (`mapping ID`, `key ID`, `value ID`).
    InsertKeyValue(Field<N>, Field<N>, Field<N>),
    /// Updates the key-value pair, (`mapping ID`, `index`, `key ID`, `value ID`).
    UpdateKeyValue(Field<N>, u64, Field<N>, Field<N>),
    /// Removes the key-value pair, (`mapping ID`, `index`).
    RemoveKeyValue(Field<N>, u64),
    /// Removes a mapping, (`mapping ID`).
    RemoveMapping(Field<N>),
}

impl<N: Network> FinalizeOperation<N> {
    /// Returns the mapping ID of the finalize operation.
    pub fn mapping_id(&self) -> Field<N> {
        match self {
            FinalizeOperation::InitializeMapping(mapping_id) => *mapping_id,
            FinalizeOperation::InsertKeyValue(mapping_id, _, _) => *mapping_id,
            FinalizeOperation::UpdateKeyValue(mapping_id, _, _, _) => *mapping_id,
            FinalizeOperation::RemoveKeyValue(mapping_id, _) => *mapping_id,
            FinalizeOperation::RemoveMapping(mapping_id) => *mapping_id,
        }
    }
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    /// Samples a random `InitializeMapping`.
    pub(crate) fn sample_initialize_mapping(rng: &mut TestRng) -> FinalizeOperation<CurrentNetwork> {
        FinalizeOperation::InitializeMapping(Uniform::rand(rng))
    }

    /// Samples a random `InsertKeyValue`.
    pub(crate) fn sample_insert_key_value(rng: &mut TestRng) -> FinalizeOperation<CurrentNetwork> {
        FinalizeOperation::InsertKeyValue(Uniform::rand(rng), Uniform::rand(rng), Uniform::rand(rng))
    }

    /// Samples a random `UpdateKeyValue`.
    pub(crate) fn sample_update_key_value(rng: &mut TestRng) -> FinalizeOperation<CurrentNetwork> {
        FinalizeOperation::UpdateKeyValue(Uniform::rand(rng), rng.gen(), Uniform::rand(rng), Uniform::rand(rng))
    }

    /// Samples a random `RemoveKeyValue`.
    pub(crate) fn sample_remove_key_value(rng: &mut TestRng) -> FinalizeOperation<CurrentNetwork> {
        FinalizeOperation::RemoveKeyValue(Uniform::rand(rng), rng.gen())
    }

    /// Samples a random `RemoveMapping`.
    pub(crate) fn sample_remove_mapping(rng: &mut TestRng) -> FinalizeOperation<CurrentNetwork> {
        FinalizeOperation::RemoveMapping(Uniform::rand(rng))
    }

    /// Samples a list of random `FinalizeOperation`.
    pub(crate) fn sample_finalize_operations() -> Vec<FinalizeOperation<CurrentNetwork>> {
        let rng = &mut TestRng::default();

        vec![
            sample_initialize_mapping(rng),
            sample_insert_key_value(rng),
            sample_update_key_value(rng),
            sample_remove_key_value(rng),
            sample_remove_mapping(rng),
            sample_initialize_mapping(rng),
            sample_insert_key_value(rng),
            sample_update_key_value(rng),
            sample_remove_key_value(rng),
            sample_remove_mapping(rng),
        ]
    }
}
