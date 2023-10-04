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

use snarkvm::console::account::{
    Address as AleoAddress,
    compute_key::ComputeKey as AleoComputeKey,
    private_key::PrivateKey as AleoPrivateKey,
    view_key::ViewKey as AleoViewKey,
};
use snarkvm::console::network::Testnet3 as CurrentNetwork;
use pyo3::prelude::*;
use std::str::FromStr;

mod accounts;
use accounts::*;

#[pymodule]
#[pyo3(name = "aleo_python_sdk")]
fn account(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PrivateKey>()?;
    Ok(())
}
