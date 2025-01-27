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

use crate::types::CoinbaseVerifyingKeyNative;

use pyo3::prelude::*;

use std::ops::Deref;

#[pyclass(frozen)]
pub struct CoinbaseVerifyingKey(CoinbaseVerifyingKeyNative);

impl CoinbaseVerifyingKey {
    pub fn from_native(key: CoinbaseVerifyingKeyNative) -> Self {
        Self(key)
    }
}

impl Deref for CoinbaseVerifyingKey {
    type Target = CoinbaseVerifyingKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl CoinbaseVerifyingKey {}
