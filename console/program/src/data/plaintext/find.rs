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

use super::*;

impl<N: Network> Plaintext<N> {
    /// Returns the plaintext from the given path.
    pub fn find(&self, path: &[Access<N>]) -> Result<Plaintext<N>> {
        // Ensure the path is not empty.
        ensure!(!path.is_empty(), "Attempted to find plaintext with an empty path.");

        match self {
            // Halts if the value is not a struct.
            Self::Literal(..) => bail!("'{self}' is not a struct"),
            // Retrieve the value of the member (from the value).
            Self::Struct(..) | Self::List(..) => {
                // Initialize the plaintext starting from the top-level.
                let mut plaintext = self;

                // Initialize the output.
                let mut output = None;

                // Iterate through the path to retrieve the value.
                for (i, access) in path.iter().enumerate() {
                    // If this is not the last item in the path, ensure the value is either a struct or list.
                    if i != path.len() - 1 {
                        match (plaintext, access) {
                            (Self::Struct(members, ..), Access::Member(identifier)) => {
                                match members.get(identifier) {
                                    // Halts if the member is not a struct or list.
                                    Some(Self::Literal(..)) => bail!("'{identifier}' must be a struct or list"),
                                    // Retrieve the member and update `plaintext` for the next iteration.
                                    Some(member) => plaintext = member,
                                    // Halts if the member does not exist.
                                    None => bail!("Failed to locate member '{identifier}' in '{self}'"),
                                }
                            }
                            (Self::List(list, ..), Access::Index(index)) => {
                                match list.get(**index as usize) {
                                    // Halts if the element is not a struct or list.
                                    Some(Self::Literal(..)) => bail!("'{index}' must be a struct or list"),
                                    // Retrieve the element and update `plaintext` for the next iteration.
                                    Some(member) => plaintext = member,
                                    // Halts if the index is out of bounds.
                                    None => bail!("Index '{index}' for '{self}' is out of bounds"),
                                }
                            }
                            _ => bail!("Invalid access `{access}` for `{plaintext}`"),
                        }
                    }
                    // Otherwise, return the final member.
                    else {
                        match (plaintext, access) {
                            (Self::Struct(members, ..), Access::Member(identifier)) => {
                                match members.get(identifier) {
                                    // Retrieve the member and update `plaintext` for the next iteration.
                                    Some(member) => output = Some(member.clone()),
                                    // Halts if the member does not exist.
                                    None => bail!("Failed to locate member '{identifier}' in '{self}'"),
                                }
                            }
                            (Self::List(list, ..), Access::Index(index)) => {
                                match list.get(**index as usize) {
                                    // Retrieve the element and update `plaintext` for the next iteration.
                                    Some(member) => output = Some(member.clone()),
                                    // Halts if the index is out of bounds.
                                    None => bail!("Index '{index}' for '{self}' is out of bounds"),
                                }
                            }
                            _ => bail!("Invalid access `{access}` for `{plaintext}`"),
                        }
                    }
                }

                // Return the output.
                match output {
                    Some(output) => Ok(output),
                    None => bail!("Failed to locate member from path in '{self}': {:?}", path),
                }
            }
        }
    }
}
