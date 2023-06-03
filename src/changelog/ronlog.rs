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
  FromRon, PatternReader, PatternWriter, RonlogAction, RonlogReferences,
  RonlogSection, ToRon,
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
  fn init(
    path: &PathBuf,
    message: Option<String>,
    force: bool,
  ) -> Result<bool> {
    let result = !path.exists();

    if result || force {
      path.truncate(Box::new(Self::new(message).to_ron(2)?))?;
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

struct Logic {
  cli: Ronlog,
}

impl Logic {
  fn init(&self, message: Option<String>) -> Result<()> {
    if Changelog::init(&self.cli.output_file, message, self.cli.force)? {
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

  fn main(&self) -> Result<()> {
    match self.cli.action {
      RonlogAction::Init => self.init(self.cli.message.clone()),
      RonlogAction::Release => self.release(),
    }
  }

  fn release(&self) -> Result<()> {
    if !self.cli.output_file.exists() {
      self.init(None)?;
    }

    let ronlog =
      Changelog::from_ron(&self.cli.output_file.read()?.try_into_string()?)?;

    println!("{}", ronlog.to_ron(2)?);

    Ok(())
  }
}

/// Interact with RON CHANGELOGs.
#[derive(clap::Parser, Clone)]
pub struct Ronlog {
  /// The action on a certain RONLOG.
  action: RonlogAction,

  /// Whether to enforce this action.
  #[arg(long, short)]
  force: bool,

  /// The fragment storage to process.
  #[arg(default_value = ".", long = "input", short)]
  input_directory: String,

  /// A message to add as introduction.
  #[arg(long, short)]
  message: Option<String>,

  /// The RONLOG to modify.
  #[arg(default_value = "CHANGELOG.ron", long = "output", short)]
  output_file: PathBuf,
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
    Logic { cli: self.clone() }
  }
}

/******************************************************************************/
