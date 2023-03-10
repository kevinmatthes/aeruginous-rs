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

/// The parsing error type for this struct.
///
/// An instance of `Version` can be constructed from a given string slice.  In
/// case that the parsing should fail, an appropriate error type is required.
#[derive(Debug, PartialEq, Eq)]
pub struct ParsingError;

/// The version information data structure.
///
/// Future releases of `aeruginous` might introduce changes due to which the
/// respective previous version's data would become invalid.  In order to offer
/// the possibility of an automatic conversion, the version information of the
/// application need to be stored together with the time frames.  If a
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

#[cfg(test)]
mod eq {
  use crate::Version;

  #[test]
  fn equal() {
    assert_eq!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 1,
        minor: 2,
        patch: 3
      }
    );
  }

  #[test]
  fn unequal() {
    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 1,
        minor: 2,
        patch: 0
      }
    );
    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 1,
        minor: 0,
        patch: 3
      }
    );
    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 0,
        minor: 2,
        patch: 3
      }
    );

    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 0,
        minor: 0,
        patch: 3
      }
    );
    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 0,
        minor: 2,
        patch: 0
      }
    );
    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 1,
        minor: 0,
        patch: 0
      }
    );

    assert_ne!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      },
      Version {
        major: 0,
        minor: 0,
        patch: 0
      }
    );
  }
}

impl Version {
  /// Retrieve the major version of this version instance.
  #[must_use]
  pub const fn get_major(&self) -> usize {
    self.major
  }

  /// Retrieve the minor version of this version instance.
  #[must_use]
  pub const fn get_minor(&self) -> usize {
    self.minor
  }

  /// Retrieve the patch level of this version instance.
  #[must_use]
  pub const fn get_patch(&self) -> usize {
    self.patch
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

#[cfg(test)]
mod getter {
  use crate::Version;

  #[test]
  fn major() {
    assert_eq!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      }
      .get_major(),
      1
    );
  }

  #[test]
  fn minor() {
    assert_eq!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      }
      .get_minor(),
      2
    );
  }

  #[test]
  fn patch() {
    assert_eq!(
      Version {
        major: 1,
        minor: 2,
        patch: 3
      }
      .get_patch(),
      3
    );
  }
}

#[cfg(test)]
mod setter {
  use crate::Version;

  #[test]
  fn major() {
    let mut version = Version {
      major: 1,
      minor: 2,
      patch: 3,
    };
    version.set_major(0);
    let version = version;

    assert_eq!(version.get_major(), 0);
  }

  #[test]
  fn minor() {
    let mut version = Version {
      major: 1,
      minor: 2,
      patch: 3,
    };
    version.set_minor(0);
    let version = version;

    assert_eq!(version.get_minor(), 0);
  }

  #[test]
  fn patch() {
    let mut version = Version {
      major: 1,
      minor: 2,
      patch: 3,
    };
    version.set_patch(0);
    let version = version;

    assert_eq!(version.get_patch(), 0);
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

#[cfg(test)]
mod fmt {
  use crate::{Version, VERSION};
  use std::str::FromStr;

  #[test]
  fn crate_version_constant() {
    assert_eq!(VERSION, format!("{}", Version::from_str(VERSION).unwrap()));
  }

  #[test]
  fn from_string() {
    assert_eq!("v1.0.0", format!("{}", Version::from_str("1").unwrap()));
    assert_eq!("v1.0.0", format!("{}", Version::from_str("v1").unwrap()));
    assert_eq!("v1.2.0", format!("{}", Version::from_str("1.2").unwrap()));
    assert_eq!("v1.2.0", format!("{}", Version::from_str("v1.2").unwrap()));
    assert_eq!("v1.2.3", format!("{}", Version::from_str("1.2.3").unwrap()));
    assert_eq!(
      "v1.2.3",
      format!("{}", Version::from_str("v1.2.3").unwrap())
    );
  }

  #[test]
  fn simple_test() {
    assert_eq!(
      "v1.2.3",
      format!(
        "{}",
        Version {
          major: 1,
          minor: 2,
          patch: 3
        }
      )
    );
  }
}

impl std::str::FromStr for Version {
  type Err = ParsingError;

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
  /// If the parsing fails, `Err(VersionParsingError)` will be returned.
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
      _ => Err(ParsingError),
    }
  }
}

#[cfg(test)]
mod from_str {
  use crate::{Version, VersionParsingError};
  use std::str::FromStr;

  #[test]
  fn invalid_2_parts_and_only_one_numeric() {
    assert_eq!(Version::from_str("1.abc"), Err(VersionParsingError));
    assert_eq!(Version::from_str("abc.1"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_3_parts_and_just_two_numeric() {
    assert_eq!(Version::from_str("1.2.abc"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.abc.2"), Err(VersionParsingError));
    assert_eq!(Version::from_str("abc.1.2"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_3_parts_and_only_one_numeric() {
    assert_eq!(Version::from_str("1.abc.def"), Err(VersionParsingError));
    assert_eq!(Version::from_str("abc.1.def"), Err(VersionParsingError));
    assert_eq!(Version::from_str("abc.def.1"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_hexdecimal() {
    assert_eq!(Version::from_str("0x1"), Err(VersionParsingError));

    assert_eq!(Version::from_str("0x1.0x2"), Err(VersionParsingError));
    assert_eq!(Version::from_str("0x1.2"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.0x2"), Err(VersionParsingError));

    assert_eq!(Version::from_str("0x1.0x2.0x3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("0x1.0x2.3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("0x1.2.0x3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("0x1.2.3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.0x2.0x3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.0x2.3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.2.0x3"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_introduced_but_empty_parts() {
    assert_eq!(Version::from_str(""), Err(VersionParsingError));

    assert_eq!(Version::from_str("."), Err(VersionParsingError));
    assert_eq!(Version::from_str("1."), Err(VersionParsingError));
    assert_eq!(Version::from_str(".2"), Err(VersionParsingError));

    assert_eq!(Version::from_str(".."), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.."), Err(VersionParsingError));
    assert_eq!(Version::from_str(".2."), Err(VersionParsingError));
    assert_eq!(Version::from_str("..3"), Err(VersionParsingError));
    assert_eq!(Version::from_str("1.2."), Err(VersionParsingError));
    assert_eq!(Version::from_str("1..3"), Err(VersionParsingError));
    assert_eq!(Version::from_str(".2.3"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_only_letters_1_part() {
    assert_eq!(Version::from_str("abc"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_only_letters_2_parts() {
    assert_eq!(Version::from_str("abc.def"), Err(VersionParsingError));
  }

  #[test]
  fn invalid_only_letters_3_parts() {
    assert_eq!(Version::from_str("abc.def.ghi"), Err(VersionParsingError));
  }

  #[test]
  fn valid_1_part() {
    assert_eq!(
      Version::from_str("1"),
      Ok(Version {
        major: 1,
        minor: 0,
        patch: 0
      })
    );
  }

  #[test]
  fn valid_2_parts() {
    assert_eq!(
      Version::from_str("1.2"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 0
      })
    );
  }

  #[test]
  fn valid_3_parts() {
    assert_eq!(
      Version::from_str("1.2.3"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      })
    );
  }

  #[test]
  fn valid_3_parts_and_dot() {
    assert_eq!(
      Version::from_str("1.2.3."),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      })
    );
  }

  #[test]
  fn valid_4_parts() {
    assert_eq!(
      Version::from_str("1.2.3.4"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      })
    );
  }

  #[test]
  fn valid_4th_part_letter() {
    assert_eq!(
      Version::from_str("1.2.3.x"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      })
    );
  }

  #[test]
  fn valid_v_prefix() {
    assert_eq!(
      Version::from_str("v1"),
      Ok(Version {
        major: 1,
        minor: 0,
        patch: 0
      })
    );
    assert_eq!(
      Version::from_str("v1.2"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 0
      })
    );
    assert_eq!(
      Version::from_str("v1.2.3"),
      Ok(Version {
        major: 1,
        minor: 2,
        patch: 3
      })
    );
  }
}

/******************************************************************************/
