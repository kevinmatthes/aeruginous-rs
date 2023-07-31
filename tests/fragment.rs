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

use aeruginous::Fragment;
use std::collections::HashMap;

#[test]
fn merge() {
    let mut f1 = Fragment::new(
        &HashMap::from([("a".to_string(), "b".to_string())]),
        &HashMap::from([(
            "Added".to_string(),
            vec!["something".to_string(), "something else".to_string()],
        )]),
    );
    let f2 = Fragment::new(
        &HashMap::from([("c".to_string(), "d".to_string())]),
        &HashMap::from([
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
            &HashMap::from([
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string())
            ]),
            &HashMap::from([
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
        Fragment::new(&HashMap::from(references.clone()), &HashMap::new());

    assert_eq!(fragment.move_references(), HashMap::from(references));
    assert!(fragment.references().is_empty());
}

/******************************************************************************/
