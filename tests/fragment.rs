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

use aeruginous::{Fragment, FragmentExportFormat, ToMd, ToRst};
use indexmap::IndexMap;
use sysexits::ExitCode;

#[test]
fn debug_trait() {
    assert_eq!(format!("{:?}", FragmentExportFormat::Md), "Md");
}

#[test]
fn merge() {
    let mut f1 = Fragment::new(
        &IndexMap::from([("a".to_string(), "b".to_string())]),
        &IndexMap::from([(
            "Added".to_string(),
            vec!["something".to_string(), "something else".to_string()],
        )]),
    );
    let f2 = Fragment::new(
        &IndexMap::from([("c".to_string(), "d".to_string())]),
        &IndexMap::from([
            ("Added".to_string(), vec!["nothing".to_string()]),
            (
                "Changed".to_string(),
                vec!["everything".to_string(), "nothing else".to_string()],
            ),
        ]),
    );
    f1.merge(f2);

    assert_eq!(
        f1,
        Fragment::new(
            &IndexMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string())
            ]),
            &IndexMap::from([
                (
                    "Added".to_string(),
                    vec![
                        "something".to_string(),
                        "something else".to_string(),
                        "nothing".to_string()
                    ]
                ),
                (
                    "Changed".to_string(),
                    vec!["everything".to_string(), "nothing else".to_string()]
                )
            ])
        )
    );
}

#[test]
fn move_fragments() {
    let references = [
        ("a".to_string(), "b".to_string()),
        ("c".to_string(), "d".to_string()),
    ];
    let mut fragment =
        Fragment::new(&IndexMap::from(references.clone()), &IndexMap::new());

    assert_eq!(fragment.move_references(), IndexMap::from(references));
    assert!(fragment.references().is_empty());
}

#[test]
fn sort() {
    let mut fragment = Fragment::new(
        &IndexMap::from([
            ("a".to_string(), "b".to_string()),
            ("c".to_string(), "d".to_string()),
        ]),
        &IndexMap::from([
            (
                "Changed".to_string(),
                vec![
                    "something else".to_string(),
                    "anything".to_string(),
                    "nothing".to_string(),
                ],
            ),
            (
                "Added".to_string(),
                vec![
                    "something".to_string(),
                    "nothing".to_string(),
                    "something else".to_string(),
                ],
            ),
        ]),
    );

    fragment.sort();

    assert_eq!(
        fragment.to_md(1).unwrap(),
        "\
[a]:  b
[c]:  d

# Added

- nothing

- something

- something else

# Changed

- anything

- nothing

- something else

"
    );
    assert_eq!(
        fragment.to_md(2).unwrap(),
        "\
[a]:  b
[c]:  d

## Added

- nothing

- something

- something else

## Changed

- anything

- nothing

- something else

"
    );
    assert_eq!(
        fragment.to_md(3).unwrap(),
        "\
[a]:  b
[c]:  d

### Added

- nothing

- something

- something else

### Changed

- anything

- nothing

- something else

"
    );
    assert_eq!(
        fragment.to_rst(1).unwrap(),
        "\
.. _a:  b
.. _c:  d

Added
=====

- nothing

- something

- something else

Changed
=======

- anything

- nothing

- something else

"
    );
    assert_eq!(
        fragment.to_rst(2).unwrap(),
        "\
.. _a:  b
.. _c:  d

Added
-----

- nothing

- something

- something else

Changed
-------

- anything

- nothing

- something else

"
    );
    assert_eq!(
        fragment.to_rst(3).unwrap(),
        "\
.. _a:  b
.. _c:  d

Added
.....

- nothing

- something

- something else

Changed
.......

- anything

- nothing

- something else

"
    );
}

#[test]
fn to_md_data_error() {
    assert_eq!(Fragment::default().to_md(0), Err(ExitCode::DataErr));
    assert_eq!(Fragment::default().to_md(4), Err(ExitCode::DataErr));
}

#[test]
fn to_rst_data_error() {
    assert_eq!(Fragment::default().to_rst(0), Err(ExitCode::DataErr));
    assert_eq!(Fragment::default().to_rst(4), Err(ExitCode::DataErr));
}

/******************************************************************************/
