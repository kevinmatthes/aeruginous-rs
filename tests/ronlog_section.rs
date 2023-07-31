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

use aeruginous::{Fragment, RonlogSection};
use std::collections::HashMap;

#[test]
fn add_changes() {
    let references = [
        ("a".to_string(), "b".to_string()),
        ("c".to_string(), "d".to_string()),
    ];
    let mut section =
        RonlogSection::new(Fragment::default(), "v1.2.3", None, None).unwrap();
    section.add_changes(Fragment::new(
        &HashMap::from(references.clone()),
        &HashMap::new(),
    ));

    assert_eq!(section.references(), &HashMap::from(references));
}

#[test]
fn cmp() {
    assert!(
        RonlogSection::new(Fragment::default(), "v1.2.3", None, None).unwrap()
            > RonlogSection::new(Fragment::default(), "v0.0.0", None, None)
                .unwrap()
    );
}

#[test]
fn merge_1() {
    let source = RonlogSection::new(
        Fragment::new(
            &HashMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string()),
            ]),
            &HashMap::new(),
        ),
        "v1.2.3",
        None,
        None,
    )
    .unwrap();
    let mut target =
        RonlogSection::new(Fragment::default(), "v1.2.3", None, None).unwrap();
    target.merge(source);

    assert_eq!(
        target,
        RonlogSection::new(
            Fragment::new(
                &HashMap::from([
                    ("a".to_string(), "b".to_string()),
                    ("c".to_string(), "d".to_string())
                ]),
                &HashMap::new()
            ),
            "v1.2.3",
            None,
            None
        )
        .unwrap()
    );
}

#[test]
fn merge_2() {
    let source = RonlogSection::new(
        Fragment::new(
            &HashMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string()),
            ]),
            &HashMap::new(),
        ),
        "v1.2.3",
        Some("source section".to_string()),
        None,
    )
    .unwrap();
    let mut target = RonlogSection::new(
        Fragment::default(),
        "v1.2.3",
        Some("target section".to_string()),
        None,
    )
    .unwrap();
    target.merge(source);

    assert_eq!(
        target,
        RonlogSection::new(
            Fragment::new(
                &HashMap::from([
                    ("a".to_string(), "b".to_string()),
                    ("c".to_string(), "d".to_string())
                ]),
                &HashMap::new()
            ),
            "v1.2.3",
            Some("target section\nsource section".to_string()),
            None
        )
        .unwrap()
    );
}

#[test]
fn move_references() {
    let references = [
        ("a".to_string(), "b".to_string()),
        ("c".to_string(), "d".to_string()),
    ];
    let mut section = RonlogSection::new(
        Fragment::new(&HashMap::from(references.clone()), &HashMap::new()),
        "v1.2.3",
        None,
        None,
    )
    .unwrap();

    assert_eq!(section.move_references(), HashMap::from(references));
    assert!(section.references().is_empty());
}

/******************************************************************************/
