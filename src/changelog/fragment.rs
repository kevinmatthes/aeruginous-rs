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

use crate::{AppendAsLine, RonlogReferences};
use indexmap::IndexMap;
use sysexits::{ExitCode, Result};

/// The supported export formats.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExportFormat {
    /// Markdown.
    Md,

    /// Rusty Object Notation.
    Ron,

    /// reStructured Text.
    Rst,

    /// eXtensible Markup Language.
    Xml,
}

crate::enum_trait!(ExportFormat {
    Md <-> "md",
    Ron <-> "ron",
    Rst <-> "rst",
    Xml <-> "xml"
});

/// The fragment data structure for exporting the harvested changes.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    /// The hyperlinks to references for further reading.
    references: RonlogReferences,

    /// The harvested changes.
    changes: IndexMap<String, Vec<String>>,
}

impl Fragment {
    crate::getters!(@fn @ref
      references: RonlogReferences,
      changes: IndexMap<String, Vec<String>>
    );

    /// Insert a new change into a category.
    pub fn insert(&mut self, category: &str, change: &str) {
        self.changes
            .entry(category.to_string())
            .and_modify(|v| v.push(change.to_string()))
            .or_insert(vec![change.to_string()]);
    }

    /// Add another instance's contents to this one's.
    pub fn merge(&mut self, other: Self) {
        self.reference(other.references.clone());

        for (category, changes) in other.changes {
            for change in changes {
                self.insert(&category, &change);
            }
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
        changes: &IndexMap<String, Vec<String>>,
    ) -> Self {
        Self {
            references: references.clone(),
            changes: changes.clone(),
        }
    }

    /// Sort all entries and categories.
    pub fn sort(&mut self) {
        for (_, entries) in &mut self.changes {
            entries.sort();
        }

        self.changes.sort_by(|key_1, _, key_2, _| key_1.cmp(key_2));
    }

    /// Add references to this fragment.
    pub fn reference(&mut self, references: RonlogReferences) {
        for (link, target) in references {
            self.references
                .entry(link)
                .and_modify(|t| *t = target.clone())
                .or_insert(target);
        }
    }
}

impl crate::FromRst for Fragment {
    fn from_rst(rst: &str) -> Result<Self> {
        let mut category = String::new();
        let mut change = String::new();
        let mut previous_line = String::new();
        let mut result = Self::default();

        for line in rst.lines() {
            if line.starts_with(".. _") && line.contains(':') {
                if let Some(reference) = line.strip_prefix(".. _") {
                    if let Some((link, target)) = reference.split_once(':') {
                        result.reference(indexmap::IndexMap::from([(
                            link.trim().to_string(),
                            target.trim().to_string(),
                        )]));
                    }
                }
            } else if line.starts_with(|c| "=-.".contains(c))
                && !previous_line.is_empty()
            {
                if let Some(c) = line.chars().next() {
                    if c.to_string().repeat(previous_line.len()) == line {
                        category = previous_line;
                    }
                }
            } else if line.starts_with(|c| "*-".contains(c))
                && !category.is_empty()
            {
                if let Some(new_change) =
                    line.strip_prefix(|c| "*-".contains(c))
                {
                    change.append_as_line(new_change);
                }
            } else if line.starts_with("  ") && !line.trim().is_empty() {
                change.append_as_line(line);
            } else if line.trim().is_empty() && !change.is_empty() {
                result.insert(category.trim(), change.trim());
                change.clear();
            }

            previous_line = line.to_string();
        }

        Ok(result)
    }
}

impl crate::ToMd for Fragment {
    fn to_md(&self, header_level: u8) -> Result<String> {
        if (1..=3).contains(&header_level) {
            let header_introduction = "#".repeat(header_level.into());
            let mut result = String::new();

            for (link_name, target) in &self.references {
                result.append_as_line(format!("[{link_name}]:  {target}"));
            }

            if !self.references.is_empty() {
                result.push('\n');
            }

            for (category, changes) in &self.changes {
                result.append_as_line(format!(
                    "{header_introduction} {category}\n",
                ));

                for change in changes {
                    result.append_as_line(format!("- {change}\n"));
                }
            }

            Ok(result)
        } else {
            Err(ExitCode::DataErr)
        }
    }
}

impl crate::ToRst for Fragment {
    fn to_rst(&self, header_level: u8) -> Result<String> {
        let header_character = match header_level {
            1 => Ok("="),
            2 => Ok("-"),
            3 => Ok("."),
            _ => Err(ExitCode::DataErr),
        }?;
        let mut result = String::new();

        for (link_name, target) in &self.references {
            result.append_as_line(format!(".. _{link_name}:  {target}"));
        }

        if !self.references.is_empty() {
            result.push('\n');
        }

        for (category, changes) in &self.changes {
            result.append_as_line(format!(
                "{category}\n{}\n",
                header_character.repeat(category.len())
            ));

            for change in changes {
                result.append_as_line(format!("- {change}\n"));
            }
        }

        Ok(result)
    }
}

impl Default for Fragment {
    fn default() -> Self {
        Self::new(&RonlogReferences::new(), &IndexMap::new())
    }
}

/******************************************************************************/
