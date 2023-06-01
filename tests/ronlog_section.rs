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

use aeruginous::RonlogSection;
use std::collections::HashMap;

#[test]
fn move_fragments() {
  let references = [
    ("a".to_string(), "b".to_string()),
    ("c".to_string(), "d".to_string()),
  ];
  let mut section = RonlogSection::new(
    aeruginous::Fragment::new(
      &HashMap::from(references.clone()),
      &HashMap::new(),
    ),
    "v1.2.3",
    None,
    None,
  )
  .unwrap();

  assert_eq!(section.move_references(), HashMap::from(references));
  assert!(section.references().is_empty());
}

/******************************************************************************/
