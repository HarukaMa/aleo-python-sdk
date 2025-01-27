// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm::console::network::Testnet3;
use snarkvm::prelude::coinbase::{
    CoinbasePuzzle, CoinbaseVerifyingKey, EpochChallenge, ProverSolution,
};
use snarkvm::prelude::{
    Address, Ciphertext, ComputeKey, Plaintext, PrivateKey, Record, Signature, ViewKey,
};

// Account types
pub type AddressNative = Address<CurrentNetwork>;
pub type ComputeKeyNative = ComputeKey<CurrentNetwork>;
pub type PrivateKeyNative = PrivateKey<CurrentNetwork>;
pub type SignatureNative = Signature<CurrentNetwork>;
pub type ViewKeyNative = ViewKey<CurrentNetwork>;

// Network types
pub type CurrentNetwork = Testnet3;

// Record types
pub type CiphertextNative = Ciphertext<CurrentNetwork>;
pub type PlaintextNative = Plaintext<CurrentNetwork>;
pub type RecordCiphertextNative = Record<CurrentNetwork, CiphertextNative>;
pub type RecordPlaintextNative = Record<CurrentNetwork, PlaintextNative>;

// Coinbase types
pub type CoinbasePuzzleNative = CoinbasePuzzle<CurrentNetwork>;
pub type CoinbaseVerifyingKeyNative = CoinbaseVerifyingKey<CurrentNetwork>;
pub type EpochChallengeNative = EpochChallenge<CurrentNetwork>;
pub type ProverSolutionNative = ProverSolution<CurrentNetwork>;
