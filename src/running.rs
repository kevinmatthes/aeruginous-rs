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

//! The time frame of the ongoing time tracking session.

use chrono::{DateTime, Local};

/// The data type for an ongoing time tracking session.
///
/// `aeruginous` saves the starting point of time of the current time tracking
/// session in a configuration file.  When the tracking session is ended, the
/// given point of time will be the begin of a new time frame to be appended to
/// the respective project.  When the time frame is saved in the correct
/// project, the configuration file for the finished time tracking session will
/// be removed.
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
