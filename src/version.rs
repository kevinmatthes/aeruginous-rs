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

use sysexits::ExitCode;

/// The version information data structure.
///
/// Future releases of `aeruginous` might introduce changes due to which the
/// respective previous version's data would become invalid.  In order to offer
/// the possibility of an automatic conversion, the version information of the
/// application needs to be stored together with the time frames.  If a
/// conversion should be necessary, `aeruginous` will take care about the
/// required adjustments.
#[derive(Debug, PartialEq, Eq)]
pub struct Version {
  /// The major version.
  major: usize,

  /// The minor version.
  minor: usize,

  /// The patch level.
  patch: usize,
}

impl Version {
  crate::getters!(@fn @cp major: usize, minor: usize, patch: usize);

  /// Create a new version instance.
  #[must_use]
  pub const fn new(major: usize, minor: usize, patch: usize) -> Self {
    Self {
      major,
      minor,
      patch,
    }
  }

  /// Modify the major version of this version instance.
  pub fn set_major(&mut self, major: usize) {
    self.major = major;
  }

  /// Modify the minor version of this version instance.
  pub fn set_minor(&mut self, minor: usize) {
    self.minor = minor;
  }

  /// Modify the patch level of this version instance.
  pub fn set_patch(&mut self, patch: usize) {
    self.patch = patch;
  }
}

impl std::fmt::Display for Version {
  /// The string representation of a version instance.
  ///
  /// The given version instance will be formatted into a string using the
  /// format `v<major>.<minor>.<patch>`.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl std::str::FromStr for Version {
  type Err = ExitCode;

  /// Create a new version instance from a string slice.
  ///
  /// There can be up to three parts for a version instance, separated by dots
  /// each.  In case of any further parts, anything after the third dot will be
  /// ignored.  If one part should not be given, it will be mapped to `0`.  The
  /// version instance will be filled in the following order:
  ///
  /// 1. `major`
  /// 1. `minor`
  /// 1. `patch`
  ///
  /// Each part must be decimal.  Other representations are not allowed.  The
  /// first part is allowed to be prefixed `v`.
  ///
  /// If the string slice should be empty, the parsing will fail.  If at least
  /// one part should be introduced but empty, the parsing will fail.  If at
  /// least one part should contain non-numeric characters, the parsing will
  /// fail.
  ///
  /// If the parsing fails, [`sysexits::ExitCode::DataErr`] will be returned as
  /// [`Result::Err`].
  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = if string.starts_with('v') {
      string.strip_prefix('v').unwrap()
    } else {
      string
    }
    .split('.')
    .collect();
    let (major_version, minor_version, patch_level) = match parts.len() {
      1 => (parts[0].parse::<usize>(), Ok(0), Ok(0)),
      2 => (parts[0].parse::<usize>(), parts[1].parse::<usize>(), Ok(0)),
      _ => (
        parts[0].parse::<usize>(),
        parts[1].parse::<usize>(),
        parts[2].parse::<usize>(),
      ),
    };

    match (major_version, minor_version, patch_level) {
      (Ok(major), Ok(minor), Ok(patch)) => Ok(Self {
        major,
        minor,
        patch,
      }),
      _ => Err(ExitCode::DataErr),
    }
  }
}

/******************************************************************************/
