// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Request Provenance

use std::fmt;
use serde::{Serialize, Deserialize};
/// RPC request origin
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum Origin {
	/// RPC server (includes request origin)
	Rpc(String),

	Unknown,
}

impl Default for Origin {
	fn default() -> Self {
		Origin::Unknown
	}
}

impl fmt::Display for Origin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Origin::Rpc(ref origin) => write!(f, "{} via RPC", origin),
			Origin::Unknown => write!(f, "unknown origin"),
		}
	}
}


