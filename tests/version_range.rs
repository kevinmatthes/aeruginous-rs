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

use aeruginous::VersionRange;
use std::str::FromStr;

#[test]
fn from_str_failure() {
  assert!(VersionRange::from_str("").is_err());
}

#[test]
fn str_identity_major() {
  assert_eq!(
    VersionRange::from_str(&format!("{}", VersionRange::Major)).unwrap(),
    VersionRange::Major
  );
}

#[test]
fn str_identity_minor() {
  assert_eq!(
    VersionRange::from_str(&format!("{}", VersionRange::Minor)).unwrap(),
    VersionRange::Minor
  );
}

#[test]
fn str_identity_patch() {
  assert_eq!(
    VersionRange::from_str(&format!("{}", VersionRange::Patch)).unwrap(),
    VersionRange::Patch
  );
}

/******************************************************************************/
