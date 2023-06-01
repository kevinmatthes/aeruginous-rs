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
use chrono::Local;
use std::str::FromStr;

/// A RONLOG section.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Section {
  /// The references of this section.
  references: crate::RonlogReferences,

  /// The version this section documents.
  version: crate::Version,

  /// The date the version this section is about was published.
  released: chrono::DateTime<Local>,

  /// The introductory text.
  introduction: Option<String>,

  /// The held fragment.
  changes: Fragment,
}

impl Section {
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
      references.entry(link).or_insert(target);
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

/******************************************************************************/
