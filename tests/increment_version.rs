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

use aeruginous::{IncrementVersion, VersionRange::Patch, VERSION};
use std::fs::{copy, remove_file};

#[test]
fn success() {
    copy(".version", "tests/.version").unwrap();
    copy("Cargo.lock", "tests/Cargo.lock").unwrap();
    copy("Cargo.toml", "tests/Cargo.toml").unwrap();

    assert!(IncrementVersion::new(
        vec![
            "tests/.version".into(),
            "tests/Cargo.lock".into(),
            "tests/Cargo.toml".into()
        ],
        vec!["tests/Cargo.toml".into()],
        VERSION.to_string(),
        Some("aeruginous".to_string()),
        Patch
    )
    .main()
    .is_ok());

    remove_file("tests/.version").unwrap();
    remove_file("tests/Cargo.lock").unwrap();
    remove_file("tests/Cargo.toml").unwrap();
}

/******************************************************************************/
