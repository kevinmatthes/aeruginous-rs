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

/// Create an instance from a reStructured Text string.
pub trait FromRst: Sized {
  /// Create an instance from valid reStructured Text.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  fn from_rst(rst: &str) -> Result<Self>;
}

/// Convert this instance into a reStructured Text string.
pub trait ToRst: Sized {
  /// Convert an instance to valid reStructured Text.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  fn to_rst(&self, header_level: u8) -> Result<String>;
}

/******************************************************************************/
