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

use std::{
  cmp::Ordering,
  fmt::{Display, Formatter, Result as FmtResult},
  str::FromStr,
};
use sysexits::ExitCode;

/// The range to increment a [`Version`] by.
#[derive(Clone, Copy)]
pub enum Range {
  /// Create a SemVer major release.
  Major,

  /// Create a SemVer minor release.
  Minor,

  /// Create a SemVer patch release.
  Patch,
}

impl Display for Range {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "{}",
      match self {
        Self::Major => "major",
        Self::Minor => "minor",
        Self::Patch => "patch",
      }
    )
  }
}

impl FromStr for Range {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "major" => Ok(Self::Major),
      "minor" => Ok(Self::Minor),
      "patch" => Ok(Self::Patch),
      _ => Err("please specify either 'major', 'minor', or 'patch'"),
    }
  }
}

/// The version information data structure.
#[derive(
  Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
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

  /// Increment this instance by a [`crate::VersionRange`].
  pub fn increment(&mut self, range: Range) -> &mut Self {
    match range {
      Range::Major => {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
      }
      Range::Minor => {
        self.minor += 1;
        self.patch = 0;
      }
      Range::Patch => self.patch += 1,
    }

    self
  }

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

impl Ord for Version {
  /// [`Version`]s are sorted by their parts.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use aeruginous::Version;
  ///
  /// assert!(Version::new(0, 0, 1) > Version::new(0, 0, 0));
  /// assert!(Version::new(0, 1, 0) > Version::new(0, 0, 2));
  /// assert!(Version::new(1, 0, 0) > Version::new(0, 2, 0));
  /// assert!(Version::new(2, 0, 0) > Version::new(1, 0, 0));
  /// ```
  fn cmp(&self, other: &Self) -> Ordering {
    match self.major.cmp(&other.major) {
      Ordering::Equal => match self.minor.cmp(&other.minor) {
        Ordering::Equal => self.patch.cmp(&other.patch),
        cmp => cmp,
      },
      cmp => cmp,
    }
  }
}

impl PartialOrd for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Display for Version {
  /// The string representation of a version instance.
  ///
  /// The given version instance will be formatted into a string using the
  /// format `v<major>.<minor>.<patch>`.
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl FromStr for Version {
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

    if let (Ok(major), Ok(minor), Ok(patch)) =
      (major_version, minor_version, patch_level)
    {
      Ok(Self {
        major,
        minor,
        patch,
      })
    } else {
      eprintln!("This version is invalid.");
      Err(ExitCode::DataErr)
    }
  }
}

/******************************************************************************/
