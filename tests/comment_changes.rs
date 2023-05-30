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

use aeruginous::CommentChangesData;
use std::collections::HashMap;

#[test]
fn branch_name_repository_implicitly_opened() {
  let mut cc =
    CommentChangesData::new(None, String::new(), HashMap::new(), vec![], false);

  assert!(cc.branch_name().is_ok());
}

#[test]
fn branch_name_repository_previously_opened() {
  let mut cc =
    CommentChangesData::new(None, String::new(), HashMap::new(), vec![], false);
  cc.open_repository().unwrap();

  assert!(cc.branch_name().is_ok());
}

#[test]
fn generate_changelog_fragment_no_links() {
  let mut cc = CommentChangesData::new(
    None,
    '/'.to_string(),
    HashMap::new(),
    vec![],
    false,
  );
  cc.query_last_n_commits(&None).unwrap();

  assert!(!cc.generate_changelog_fragment(3, "rst").is_empty());
}

#[test]
fn generate_changelog_fragment_with_links() {
  let mut cc = CommentChangesData::new(
    None,
    '/'.to_string(),
    HashMap::from([("hyperlink".to_string(), "target".to_string())]),
    vec![],
    false,
  );
  cc.query_last_n_commits(&None).unwrap();

  assert!(!cc.generate_changelog_fragment(3, "rst").is_empty());
}

#[test]
fn resolve_links() {
  assert_eq!(
    CommentChangesData::new(None, String::new(), HashMap::new(), vec![], false)
      .resolve_links("rst"),
    String::new()
  );
  assert!([
    ".. _a.rs:  src/a.rs\n.. _b.rs:  src/b.rs\n.. _d.rs:  src/d.rs\n\n"
      .to_string(),
    ".. _a.rs:  src/a.rs\n.. _d.rs:  src/d.rs\n.. _b.rs:  src/b.rs\n\n"
      .to_string(),
    ".. _b.rs:  src/b.rs\n.. _a.rs:  src/a.rs\n.. _d.rs:  src/d.rs\n\n"
      .to_string(),
    ".. _b.rs:  src/b.rs\n.. _d.rs:  src/d.rs\n.. _a.rs:  src/a.rs\n\n"
      .to_string(),
    ".. _d.rs:  src/d.rs\n.. _a.rs:  src/a.rs\n.. _b.rs:  src/b.rs\n\n"
      .to_string(),
    ".. _d.rs:  src/d.rs\n.. _b.rs:  src/b.rs\n.. _a.rs:  src/a.rs\n\n"
      .to_string(),
  ]
  .contains(
    &CommentChangesData::new(
      None,
      String::new(),
      HashMap::from([
        ("a.rs".to_string(), "src/a.rs".to_string()),
        ("b.rs".to_string(), "src/b.rs".to_string()),
        ("d.rs".to_string(), "src/d.rs".to_string())
      ]),
      vec![],
      false
    )
    .resolve_links("rst")
  ));
}

/******************************************************************************/
