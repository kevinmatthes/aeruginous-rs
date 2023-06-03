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

/// The action to execute on a given RONLOG.
#[derive(Clone, Copy)]
pub enum Action {
  /// Initialise a new RONLOG.
  Init,

  /// Create the RONLOG section for a new version.
  Release,
}

impl std::fmt::Display for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Init => "init",
        Self::Release => "release",
      }
    )
  }
}

impl std::str::FromStr for Action {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, String> {
    match s {
      "init" => Ok(Self::Init),
      "release" => Ok(Self::Release),
      _ => Err(format!("the mode '{s}' is not supported, yet")),
    }
  }
}

/******************************************************************************/
