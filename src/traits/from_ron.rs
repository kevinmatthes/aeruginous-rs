/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2023 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

use sysexits::Result;

/// Create an instance from a RON string.
pub trait FromRon<'a>: serde::Deserialize<'a> {
  /// Create an instance implementing [`serde::Deserialize`] from valid RON.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  fn from_ron(ron: &'a str) -> Result<Self>;
}

impl<'a, T: serde::Deserialize<'a>> FromRon<'a> for T {
  fn from_ron(ron: &'a str) -> Result<Self> {
    ron::de::from_str(ron).map_or(Err(sysexits::ExitCode::DataErr), Ok)
  }
}

/******************************************************************************/
