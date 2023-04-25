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

use chrono::{DateTime, Local};

/// The data type of an ongoing time tracking session.
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
  /// A deprecated synonym for [`Self::new`].
  #[cfg(not(tarpaulin_include))]
  #[deprecated(note = "Renamed to `new`.", since = "0.2.1")]
  #[must_use]
  pub fn create() -> Self {
    Self::new()
  }

  /// Retrieve the creation time.
  #[must_use]
  pub const fn get(&self) -> &DateTime<Local> {
    &self.begin
  }

  /// Construct a new running instance.
  #[must_use]
  pub fn new() -> Self {
    Self {
      begin: Local::now(),
    }
  }
}

#[cfg(test)]
mod getters {
  use crate::Running;

  #[test]
  fn begin() {
    assert!(Running::new().get() <= &chrono::Local::now());
  }
}

impl Default for Running {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod default {
  use crate::Running;

  #[test]
  fn begin() {
    assert!(Running::default().get() <= &chrono::Local::now());
  }

  #[test]
  fn method_equality() {
    assert!(Running::default().get() <= Running::new().get());
  }
}

/******************************************************************************/
