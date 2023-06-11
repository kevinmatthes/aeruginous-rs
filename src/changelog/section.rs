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

use crate::{Fragment, RonlogReferences, Version};
use chrono::{DateTime, Local};
use std::str::FromStr;

/// A RONLOG section.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Section {
  /// The references of this section.
  references: RonlogReferences,

  /// The version this section documents.
  version: Version,

  /// The date the version this section is about was published.
  released: DateTime<Local>,

  /// The introductory text.
  introduction: Option<String>,

  /// The held fragment.
  changes: Fragment,
}

impl Section {
  crate::getters!(@fn @ref
    references: RonlogReferences,
    version: Version,
    released: DateTime<Local>,
    introduction: Option<String>,
    changes: Fragment
  );

  /// Add further changes.
  pub fn add_changes(&mut self, changes: Fragment) {
    self.changes.merge(changes);

    for (link, target) in self.changes.move_references() {
      self
        .references
        .entry(link)
        .and_modify(|t| *t = target.clone())
        .or_insert(target);
    }
  }

  /// Add another instance's contents this one's.
  pub fn merge(&mut self, mut other: Self) {
    if self.version == other.version {
      self.add_changes(other.changes.clone());

      match &self.introduction {
        Some(introduction_1) => {
          if let Some(introduction_2) = &other.introduction {
            let mut introduction_1 = introduction_1.clone();

            introduction_1.push('\n');
            introduction_1.push_str(introduction_2.as_str());

            self.introduction = Some(introduction_1);
          }
        }
        None => self.introduction = other.introduction.clone(),
      }

      for (link, target) in other.move_references() {
        self
          .references
          .entry(link)
          .and_modify(|t| *t = target.clone())
          .or_insert(target);
      }

      self.released = self.released.max(other.released);
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
  ///
  /// # Errors
  ///
  /// See [`Version::from_str`].
  pub fn new(
    mut changes: Fragment,
    version: &str,
    introduction: Option<String>,
    references: Option<RonlogReferences>,
  ) -> sysexits::Result<Self> {
    let mut references = references.unwrap_or_default();

    for (link, target) in changes.move_references() {
      references
        .entry(link)
        .and_modify(|t| *t = target.clone())
        .or_insert(target);
    }

    Ok(Self {
      references,
      version: Version::from_str(version)?,
      released: Local::now(),
      introduction,
      changes,
    })
  }
}

impl Eq for Section {}

impl Ord for Section {
  /// [`crate::RonlogSection`]s are sorted by their versions.
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.version.cmp(&other.version)
  }
}

impl PartialEq for Section {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl PartialOrd for Section {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

/******************************************************************************/
