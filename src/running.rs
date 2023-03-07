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

//! # Running Time Frames
//!
//! `aeruginous` saves the starting point of time of the current time tracking
//! session in a configuration file which will be removed when the tracking
//! session is ended.

use chrono::{DateTime, Local};

/// The data type for an ongoing time tracking session.
pub struct Running {
  begin: DateTime<Local>,
}

impl Running {
  /// Construct a new running instance.
  pub fn create() -> Self {
    Running {
      begin: Local::now(),
    }
  }

  /// Retrieve the creation time.
  pub fn get(&self) -> &DateTime<Local> {
    &self.begin
  }
}

/******************************************************************************/
