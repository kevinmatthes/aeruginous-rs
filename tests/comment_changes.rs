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

use aeruginous::CommentChanges;

#[test]
fn branch_name_repository_implicitly_opened() {
  let mut cc = CommentChanges::new(None, String::new(), vec![], vec![]);

  assert!(cc.branch_name().is_ok());
}

#[test]
fn branch_name_repository_previously_opened() {
  let mut cc = CommentChanges::new(None, String::new(), vec![], vec![]);
  cc.open_repository().unwrap();

  assert!(cc.branch_name().is_ok());
}

#[test]
fn generate_changelog_fragment_no_links() {
  let mut cc = CommentChanges::new(None, '/'.to_string(), vec![], vec![]);
  cc.update_changes().unwrap();

  assert!(!cc.generate_changelog_fragment().is_empty());
}

#[test]
fn generate_changelog_fragment_with_links() {
  let mut cc = CommentChanges::new(
    None,
    '/'.to_string(),
    vec![("hyperlink".to_string(), "target".to_string())],
    vec![],
  );
  cc.update_changes().unwrap();

  assert!(!cc.generate_changelog_fragment().is_empty());
}

#[test]
fn resolve_links() {
  assert_eq!(
    CommentChanges::new(None, String::new(), vec![], vec![]).resolve_links(),
    String::new()
  );
  assert_eq!(
    CommentChanges::new(
      None,
      String::new(),
      vec![
        ("a.rs".to_string(), "src/a.rs".to_string()),
        ("b.rs".to_string(), "src/b.rs".to_string()),
        ("d.rs".to_string(), "src/d.rs".to_string())
      ],
      vec![]
    )
    .resolve_links(),
    ".. _a.rs:  src/a.rs\n.. _b.rs:  src/b.rs\n.. _d.rs:  src/d.rs\n\n"
      .to_string()
  );
}

/******************************************************************************/
