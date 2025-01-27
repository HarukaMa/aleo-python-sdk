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

use crate::{
    account::{Address, RecordCiphertext, RecordPlaintext},
    types::ViewKeyNative,
};

use pyo3::prelude::*;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
};
#[pyclass(frozen)]
#[derive(Clone)]
pub struct ViewKey(ViewKeyNative);

impl ViewKey {
    pub fn from_native(view_key: ViewKeyNative) -> Self {
        Self(view_key)
    }
}

impl Deref for ViewKey {
    type Target = ViewKeyNative;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl ViewKey {
    /// Reads in an account view key from a base58 string.
    #[staticmethod]
    fn from_string(s: &str) -> anyhow::Result<Self> {
        let view_key = FromStr::from_str(s)?;
        Ok(Self(view_key))
    }

    /// Returns the address corresponding to the view key.
    fn to_address(&self) -> Address {
        let address = self.0.to_address();
        Address::from_native(address)
    }

    /// Decrypt a record ciphertext with a view key
    pub fn decrypt(&self, record_ciphertext: &RecordCiphertext) -> anyhow::Result<RecordPlaintext> {
        record_ciphertext.decrypt(self)
    }

    /// Determines whether the record belongs to the account.
    pub fn is_owner(&self, record_ciphertext: &RecordCiphertext) -> bool {
        record_ciphertext.is_owner(self)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}
