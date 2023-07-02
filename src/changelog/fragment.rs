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

use crate::RonlogReferences;
use std::collections::HashMap;

/// The supported export formats.
pub enum ExportFormat {
  /// Markdown.
  Md,

  /// Rusty Object Notation.
  Ron,

  /// reStructured Text.
  Rst,
}

crate::enum_trait!(ExportFormat {
  Md <-> "md",
  Ron <-> "ron",
  Rst <-> "rst"
});

/// The fragment data structure for exporting the harvested changes.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
  /// The hyperlinks to references for further reading.
  references: RonlogReferences,

  /// The harvested changes.
  changes: HashMap<String, Vec<String>>,
}

impl Fragment {
  crate::getters!(@fn @ref
    references: RonlogReferences,
    changes: HashMap<String, Vec<String>>
  );

  /// Add another instance's contents to this one's.
  pub fn merge(&mut self, other: Self) {
    for (link, target) in other.references {
      self
        .references
        .entry(link)
        .and_modify(|t| *t = target.clone())
        .or_insert(target);
    }

    for (category, changes) in other.changes {
      self.changes.entry(category.clone()).or_default();

      let mut change_list = self.changes[&category].clone();
      change_list.append(&mut changes.clone());
      self.changes.insert(category, change_list);
    }
  }

  /// Move all known references out of this instance.
  #[must_use]
  pub fn move_references(&mut self) -> RonlogReferences {
    let result = self.references.clone();
    self.references.clear();
    result
  }

  /// Create a new instance.
  #[must_use]
  pub fn new(
    references: &RonlogReferences,
    changes: &HashMap<String, Vec<String>>,
  ) -> Self {
    Self {
      references: references.clone(),
      changes: changes.clone(),
    }
  }
}

impl Default for Fragment {
  fn default() -> Self {
    Self::new(&RonlogReferences::new(), &HashMap::new())
  }
}

/******************************************************************************/
