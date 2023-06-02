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

use crate::{
  PatternWriter, RonlogAction, RonlogReferences, RonlogSection, ToRon,
};
use std::path::PathBuf;
use sysexits::{ExitCode, Result};

#[derive(serde::Deserialize, serde::Serialize)]
struct Changelog {
  references: RonlogReferences,
  introduction: Option<String>,
  sections: Vec<RonlogSection>,
}

impl Changelog {
  fn init(path: &str, force: bool) -> Result<bool> {
    let mut path = PathBuf::from(path);
    path.push("CHANGELOG.ron");

    let result = !path.exists();

    if result || force {
      path.truncate(Box::new(Self::default().to_ron(2)?))?;
    }

    Ok(result)
  }

  #[must_use]
  fn new(introduction: Option<String>) -> Self {
    Self {
      references: RonlogReferences::new(),
      introduction,
      sections: Vec::new(),
    }
  }
}

impl Default for Changelog {
  fn default() -> Self {
    Self::new(None)
  }
}

struct Logic {
  cli: Ronlog,
}

impl Logic {
  fn init(&self) -> Result<()> {
    if Changelog::init(&self.cli.directory, self.cli.force)? {
      println!(
        "Successfully initialised new CHANGELOG in '{}'.",
        self.cli.directory
      );

      Ok(())
    } else if self.cli.force {
      println!(
        "Successfully re-initialised CHANGELOG in '{}'.",
        self.cli.directory
      );

      Ok(())
    } else {
      println!(
        "Use `--force` to overwrite the existing CHANGELOG in '{}'.",
        self.cli.directory
      );

      Err(ExitCode::Usage)
    }
  }

  fn main(&self) -> Result<()> {
    if matches!(self.cli.action, RonlogAction::Init) {
      self.init()
    } else {
      println!("{}", self.cli.action);
      Ok(())
    }
  }
}

/// Interact with RON CHANGELOGs.
#[derive(clap::Parser, Clone)]
pub struct Ronlog {
  /// The action on a certain RONLOG.
  action: RonlogAction,

  /// The directory to work on.
  #[arg(default_value = ".", long, short)]
  directory: String,

  /// Whether to enforce this action.
  #[arg(long, short)]
  force: bool,
}

impl Ronlog {
  /// Process the CLI instructions.
  ///
  /// # Errors
  ///
  pub fn main(&self) -> Result<()> {
    self.wrap().main()
  }

  fn wrap(&self) -> Logic {
    Logic { cli: self.clone() }
  }
}

/******************************************************************************/
