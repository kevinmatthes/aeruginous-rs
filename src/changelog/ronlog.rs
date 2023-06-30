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

use crate::{Fragment, FromRon, PatternReader, PatternWriter, ToRon, Version};
use chrono::{DateTime, Local};
use std::{path::PathBuf, str::FromStr};
use sysexits::{ExitCode, Result};

/// The action to execute on a given RONLOG.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
  /// Initialise a new RONLOG.
  Init,

  /// Create the RONLOG section for a new version.
  Release,
}

crate::enum_trait!(Action {
  Init <-> "init",
  Release <-> "release"
});

#[derive(serde::Deserialize, serde::Serialize)]
struct Changelog {
  references: References,
  introduction: Option<String>,
  sections: Vec<Section>,
}

impl Changelog {
  fn add_section(&mut self, section: Section) {
    for s in &mut self.sections {
      if s == &section {
        s.merge(section);
        return;
      }
    }

    self
      .sections
      .insert(self.sections.partition_point(|s| s > &section), section);
  }

  fn init(
    path: &PathBuf,
    message: Option<String>,
    references: References,
    force: bool,
  ) -> Result<bool> {
    let result = !path.exists();

    if result || force {
      path.truncate(Box::new(Self::new(message, references).to_ron(2)?))?;
    }

    Ok(result)
  }

  #[must_use]
  const fn new(introduction: Option<String>, references: References) -> Self {
    Self {
      references,
      introduction,
      sections: Vec::new(),
    }
  }
}

struct Logic {
  cli: Ronlog,
  hyperlinks: References,
}

impl Logic {
  fn init(&self, message: Option<String>) -> Result<()> {
    if Changelog::init(
      &self.cli.output_file,
      message,
      self.hyperlinks.clone(),
      self.cli.force,
    )? {
      println!(
        "Successfully initialised new CHANGELOG in '{}'.",
        self.cli.output_file.display()
      );

      Ok(())
    } else if self.cli.force {
      println!(
        "Successfully re-initialised CHANGELOG in '{}'.",
        self.cli.output_file.display()
      );

      Ok(())
    } else {
      println!(
        "Use `--force` to overwrite the existing CHANGELOG in '{}'.",
        self.cli.output_file.display()
      );

      Err(ExitCode::Usage)
    }
  }

  fn main(&mut self) -> Result<()> {
    self.hyperlinks = self
      .cli
      .link
      .iter()
      .zip(self.cli.target.iter())
      .map(|(a, b)| (a.to_string(), b.to_string()))
      .collect();

    match self.cli.action {
      Action::Init => self.init(self.cli.message.clone()),
      Action::Release => self.release(),
    }
  }

  fn release(&self) -> Result<()> {
    if let Some(version) = &self.cli.version {
      let mut section = Section::new(
        Fragment::default(),
        version,
        self.cli.message.clone(),
        if self.hyperlinks.is_empty() {
          None
        } else {
          Some(self.hyperlinks.clone())
        },
      )?;

      if !self.cli.output_file.exists() {
        self.init(None)?;
      }

      for entry in std::fs::read_dir(&self.cli.input_directory)? {
        let entry = entry?.path();

        if entry
          .extension()
          .map_or(false, |e| e.to_str().map_or(false, |e| e == "ron"))
        {
          if let Ok(fragment) =
            Fragment::from_ron(&entry.read()?.try_into_string()?)
          {
            section.add_changes(fragment);
            std::fs::remove_file(entry)?;
          }
        }
      }

      let mut ronlog =
        Changelog::from_ron(&self.cli.output_file.read()?.try_into_string()?)?;

      for (link, target) in section.move_references() {
        ronlog
          .references
          .entry(link)
          .and_modify(|t| *t = target.clone())
          .or_insert(target);
      }

      ronlog.add_section(section);

      self.cli.output_file.truncate(Box::new(ronlog.to_ron(2)?))
    } else {
      eprintln!("No `--version` information provided for this mode.");
      Err(ExitCode::Usage)
    }
  }
}

/// The references known to RONLOG-related instances.
pub type References = std::collections::HashMap<String, String>;

/// Interact with RON CHANGELOGs.
#[derive(clap::Parser, Clone)]
pub struct Ronlog {
  /// The action on a certain RONLOG.
  action: Action,

  /// Whether to enforce this action.
  #[arg(long, short)]
  force: bool,

  /// The fragment storage to process.
  #[arg(default_value = ".", long = "input", short)]
  input_directory: String,

  /// A message to add as introduction.
  #[arg(long, short)]
  message: Option<String>,

  /// The hyperlinks to add.
  #[arg(long, short, visible_aliases = ["hyperlink"])]
  link: Vec<String>,

  /// The RONLOG to modify.
  #[arg(default_value = "CHANGELOG.ron", long = "output", short)]
  output_file: PathBuf,

  /// The hyperlinks' targets.
  #[arg(long, short)]
  target: Vec<String>,

  /// The version to use.
  #[arg(long, short)]
  version: Option<String>,
}

impl Ronlog {
  /// Process the CLI instructions.
  ///
  /// # Errors
  ///
  /// See [`sysexits::ExitCode`].
  pub fn main(&self) -> Result<()> {
    self.wrap().main()
  }

  fn wrap(&self) -> Logic {
    Logic {
      cli: self.clone(),
      hyperlinks: References::new(),
    }
  }
}

/// A RONLOG section.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Section {
  /// The references of this section.
  references: References,

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
    references: References,
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
  pub fn move_references(&mut self) -> References {
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
    references: Option<References>,
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
