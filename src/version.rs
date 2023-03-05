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

//! # Version
//!
//! This module not only contains the version information of this CLI but also
//! defines utilities to encode, decode and compare the version information.
//! This is required in order to update the stored time frames if a new release
//! of `aeruginous` should change the interfaces and requirements concerning the
//! time frame interaction.

/// This crate's version.
pub const CRATE_VERSION: &str = "0.0.0";

/// The version information data structure.
///
/// It is going to be dumped into the configuration directory of `aeruginous` in
/// order to indicate the version of the application which was recently used to
/// modify the data.
#[derive(Debug)]
pub struct Version {
  /// The major version.
  major: usize,

  /// The minor version.
  minor: usize,

  /// The patch level.
  patch: usize,
}

/// The parsing error type for this struct.
///
/// An instance of `Version` can be constructed from a given string slice.  In
/// case that the parsing should fail, an appropriate error type is required.
#[derive(Debug, PartialEq)]
pub struct VersionParsingError;

impl Version {
  /// Retrieve the major version of this version instance.
  pub fn get_major(&self) -> usize {
    self.major
  }

  /// Retrieve the minor version of this version instance.
  pub fn get_minor(&self) -> usize {
    self.minor
  }

  /// Retrieve the patch level of this version instance.
  pub fn get_patch(&self) -> usize {
    self.patch
  }
}

impl PartialEq for Version {
  /// Two version instances are equal if all of their parts are equal to each
  /// other.
  fn eq(&self, other: &Self) -> bool {
    self.major == other.major
      && self.minor == other.minor
      && self.patch == other.patch
  }
}

impl std::str::FromStr for Version {
  type Err = VersionParsingError;

  /// Create a new version instance from a string slice.
  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = string.split('.').collect();
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
      _ => Err(VersionParsingError),
    }
  }
}

#[cfg(test)]
mod from_str {
  use crate::{Version, VersionParsingError};
  use std::str::FromStr;

  #[test]
  fn invalid_empty_slice() {
    assert_eq!(Version::from_str(""), Err(VersionParsingError));
  }

  #[test]
  fn invalid_empty_string() {
    assert_eq!(Version::from_str(&String::new()), Err(VersionParsingError));
  }

  #[test]
  fn invalid_only_letters() {
    assert_eq!(Version::from_str("abc"), Err(VersionParsingError));
  }

  #[test]
  fn valid_1_part() {
    assert_eq!(
      Ok(Version {
        major: 1,
        minor: 0,
        patch: 0
      }),
      Version::from_str("1")
    );
  }

  #[test]
  fn valid_2_parts() {
    assert_eq!(
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 0
      }),
      Version::from_str("1.2")
    );
  }

  #[test]
  fn valid_3_parts() {
    assert_eq!(
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      }),
      Version::from_str("1.2.3")
    );
  }

  #[test]
  fn valid_4_parts() {
    assert_eq!(
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      }),
      Version::from_str("1.2.3.4")
    );
  }

  #[test]
  fn valid_4th_part_letter() {
    assert_eq!(
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      }),
      Version::from_str("1.2.3.x")
    );
  }
}

/******************************************************************************/
