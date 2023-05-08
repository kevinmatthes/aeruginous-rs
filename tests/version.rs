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

use aeruginous::{Version, VERSION};
use std::str::FromStr;
use sysexits::ExitCode;

#[test]
fn fmt_crate_version_constant() {
  assert_eq!(VERSION, format!("{}", Version::from_str(VERSION).unwrap()));
}

#[test]
fn fmt_from_string() {
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
fn fmt_simple_test() {
  assert_eq!("v1.2.3", format!("{}", Version::new(1, 2, 3)));
}

#[test]
fn from_str_invalid_2_parts_and_only_one_numeric() {
  assert_eq!(Version::from_str("1.abc"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("abc.1"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_3_parts_and_just_two_numeric() {
  assert_eq!(Version::from_str("1.2.abc"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.abc.2"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("abc.1.2"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_3_parts_and_only_one_numeric() {
  assert_eq!(Version::from_str("1.abc.def"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("abc.1.def"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("abc.def.1"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_hexdecimal() {
  assert_eq!(Version::from_str("0x1"), Err(ExitCode::DataErr));

  assert_eq!(Version::from_str("0x1.0x2"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("0x1.2"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.0x2"), Err(ExitCode::DataErr));

  assert_eq!(Version::from_str("0x1.0x2.0x3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("0x1.0x2.3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("0x1.2.0x3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("0x1.2.3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.0x2.0x3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.0x2.3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.2.0x3"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_introduced_but_empty_parts() {
  assert_eq!(Version::from_str(""), Err(ExitCode::DataErr));

  assert_eq!(Version::from_str("."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str(".2"), Err(ExitCode::DataErr));

  assert_eq!(Version::from_str(".."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str(".2."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("..3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1.2."), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str("1..3"), Err(ExitCode::DataErr));
  assert_eq!(Version::from_str(".2.3"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_only_letters_1_part() {
  assert_eq!(Version::from_str("abc"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_only_letters_2_parts() {
  assert_eq!(Version::from_str("abc.def"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_invalid_only_letters_3_parts() {
  assert_eq!(Version::from_str("abc.def.ghi"), Err(ExitCode::DataErr));
}

#[test]
fn from_str_valid_1_part() {
  assert_eq!(Version::from_str("1"), Ok(Version::new(1, 0, 0)));
}

#[test]
fn from_str_valid_2_parts() {
  assert_eq!(Version::from_str("1.2"), Ok(Version::new(1, 2, 0)));
}

#[test]
fn from_str_valid_3_parts() {
  assert_eq!(Version::from_str("1.2.3"), Ok(Version::new(1, 2, 3)));
}

#[test]
fn from_str_valid_3_parts_and_dot() {
  assert_eq!(Version::from_str("1.2.3."), Ok(Version::new(1, 2, 3)));
}

#[test]
fn from_str_valid_4_parts() {
  assert_eq!(Version::from_str("1.2.3.4"), Ok(Version::new(1, 2, 3)));
}

#[test]
fn from_str_valid_4th_part_letter() {
  assert_eq!(Version::from_str("1.2.3.x"), Ok(Version::new(1, 2, 3)));
}

#[test]
fn from_str_valid_v_prefix() {
  assert_eq!(Version::from_str("v1"), Ok(Version::new(1, 0, 0)));
  assert_eq!(Version::from_str("v1.2"), Ok(Version::new(1, 2, 0)));
  assert_eq!(Version::from_str("v1.2.3"), Ok(Version::new(1, 2, 3)));
}

#[test]
fn set_major() {
  let mut version = Version::new(1, 2, 3);
  version.set_major(0);

  assert_eq!(version.major(), 0);
}

#[test]
fn set_minor() {
  let mut version = Version::new(1, 2, 3);
  version.set_minor(0);

  assert_eq!(version.minor(), 0);
}

#[test]
fn set_patch() {
  let mut version = Version::new(1, 2, 3);
  version.set_patch(0);

  assert_eq!(version.patch(), 0);
}

/******************************************************************************/
