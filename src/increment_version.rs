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

use crate::{ceprintlns, PatternReader, PatternWriter, Version, VersionRange};
use std::{path::PathBuf, str::FromStr};
use sysexits::{ExitCode, Result};

/// Increment a hard-coded version string in some files.
#[derive(clap::Parser, Clone)]
#[command(aliases = ["incver", "inc-ver", "incrementversion"])]
pub struct IncrementVersion {
  /// The files to work on.
  #[arg(long = "edit", short = 'e')]
  file_to_edit: Vec<PathBuf>,

  /// The old version to search for and replace.
  #[arg(long, short = 'v')]
  old_version: String,

  /// The increment range.
  #[arg(long, short)]
  range: VersionRange,

  /// In case of Rust projects:  which package's version shall be edited?
  #[arg(long, short = 'R')]
  rust_package: Option<String>,
}

impl IncrementVersion {
  /// Process the input data.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`crate::PatternBuffer::try_into_string`]
  /// - [`PatternReader::read`]
  /// - [`PatternWriter::truncate`]
  /// - [`Version::from_str`]
  /// - [`sysexits::ExitCode::DataErr`]
  /// - [`sysexits::ExitCode::Unavailable`]
  /// - [`sysexits::ExitCode::Usage`]
  pub fn main(&self) -> Result<()> {
    self.wrap().main()
  }

  /// Construct a new instance.
  #[must_use]
  pub fn new(
    file_to_edit: Vec<PathBuf>,
    old_version: String,
    range: VersionRange,
    rust_package: Option<String>,
  ) -> Self {
    Self {
      file_to_edit,
      old_version,
      range,
      rust_package,
    }
  }

  fn wrap(&self) -> Logic {
    Logic {
      cli: self.clone(),
      new_version: String::new(),
      old_version: self
        .old_version
        .strip_prefix('v')
        .unwrap_or(&self.old_version)
        .to_string(),
    }
  }
}

struct Logic {
  cli: IncrementVersion,
  new_version: String,
  old_version: String,
}

impl Logic {
  fn determine_new_version(&mut self) -> Result<()> {
    self.new_version = Version::from_str(&self.old_version)?
      .increment(self.cli.range)
      .to_string();
    self.new_version = self
      .new_version
      .strip_prefix('v')
      .unwrap_or(&self.new_version)
      .to_string();
    Ok(())
  }

  fn main(&mut self) -> Result<()> {
    self.determine_new_version()?;

    for file in &self.cli.file_to_edit {
      match file
        .file_name()
        .ok_or(ExitCode::Usage)?
        .to_str()
        .ok_or(ExitCode::DataErr)?
      {
        "Cargo.lock" => self.update_cargo_lock(file)?,
        "Cargo.toml" => self.update_cargo_toml(file)?,
        _ => self.update_normal_file(file)?,
      }
    }

    Ok(())
  }

  fn update_cargo_lock(&self, file: &PathBuf) -> Result<()> {
    if let Some(rust_package) = &self.cli.rust_package {
      let mut edited = false;
      let mut lock_file = match cargo_lock::Lockfile::load(file) {
        Ok(l) => Ok(l),
        Err(cargo_lock::Error::Io(e)) => {
          ceprintlns!("Error: "!Red, "{e}");
          Err(e.into())
        }
        Err(_) => {
          ceprintlns!("Error: "!Red, "unknown error.");
          Err(ExitCode::Unavailable)
        }
      }?;

      for package in &mut lock_file.packages {
        if package.name.as_str() == rust_package {
          match self.cli.range {
            VersionRange::Major => {
              package.version.major += 1;
              package.version.minor = 0;
              package.version.patch = 0;
            }
            VersionRange::Minor => {
              package.version.minor += 1;
              package.version.patch = 0;
            }
            VersionRange::Patch => package.version.patch += 1,
          }

          edited = true;
          break;
        }
      }

      if edited {
        file.truncate(Box::new(lock_file.clone().to_string()))
      } else {
        ceprintlns!("Package"!Red, "not found.");
        Err(ExitCode::DataErr)
      }
    } else {
      ceprintlns!("Package"!Red, "not specified.");
      Err(ExitCode::Usage)
    }
  }

  fn update_cargo_toml(&self, file: &PathBuf) -> Result<()> {
    if let Ok(mut manifest) =
      file.read()?.try_into_string()?.parse::<toml::Table>()
    {
      if manifest["package"]["version"].as_str().is_some() {
        manifest["package"]["version"] = self.new_version.clone().into();
        file.truncate(Box::new(manifest.to_string()))?;

        Ok(())
      } else {
        ceprintlns!(
          "Cargo.toml"!Red,
          "does not contain a `package.version` field."
        );

        Err(ExitCode::DataErr)
      }
    } else {
      ceprintlns!("Cargo.toml"!Red, "does not seem to be valid TOML.");
      Err(ExitCode::DataErr)
    }
  }

  fn update_normal_file(&self, file: &PathBuf) -> Result<()> {
    file.truncate(Box::new(
      file
        .read()?
        .try_into_string()?
        .replace(&self.old_version, &self.new_version),
    ))
  }
}

/******************************************************************************/
