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

use crate::{ceprintlns, AppendAsLine, Version, VersionRange};
use aeruginous_io::{PathBufLikeReader, PathBufLikeTruncation};
use std::{path::PathBuf, str::FromStr};
use sysexits::{ExitCode, Result};

/// Increment a hard-coded version string in some files.
#[derive(clap::Parser, Clone)]
#[command(visible_aliases = ["incver", "inc-ver", "incrementversion"])]
pub struct IncrementVersion {
    /// The files to work on.
    #[arg(long = "edit", short = 'e')]
    file_to_edit: Vec<PathBuf>,

    /// The files to update and reformat.
    #[arg(long = "rewrite", short = 'R')]
    file_to_rewrite: Vec<PathBuf>,

    /// The old version to search for and replace.
    #[arg(long, short = 'v')]
    old_version: String,

    /// In case of Rust projects:  which package's version shall be edited?
    #[arg(long, short)]
    package: Option<String>,

    /// The increment range.
    #[arg(long, short)]
    range: VersionRange,
}

impl IncrementVersion {
    /// Process the input data.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn main(&self) -> Result<()> {
        self.wrap().main()
    }

    /// Construct a new instance.
    #[must_use]
    pub fn new(
        file_to_edit: Vec<PathBuf>,
        file_to_rewrite: Vec<PathBuf>,
        old_version: String,
        package: Option<String>,
        range: VersionRange,
    ) -> Self {
        Self {
            file_to_edit,
            file_to_rewrite,
            old_version,
            package,
            range,
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

    fn edit_cargo_lock(&self, file: &PathBuf) -> Result<()> {
        if let Some(rust_package) = &self.cli.package {
            let mut edited = false;
            let mut lock_file = match cargo_lock::Lockfile::load(file) {
                Ok(l) => Ok(l),
                Err(cargo_lock::Error::Io(e)) => Err(e.into()),
                Err(_) => Err(ExitCode::Unavailable),
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
                lock_file.to_string().truncate_loudly(file)
            } else {
                ceprintlns!("Package"!Red, "not found.");
                Err(ExitCode::DataErr)
            }
        } else {
            ceprintlns!("Package"!Red, "not specified.");
            Err(ExitCode::Usage)
        }
    }

    fn edit_cargo_toml(&self, file: &PathBuf) -> Result<()> {
        let mut buffer = String::new();
        let mut package_reached = false;
        let mut package_updated = false;

        for line in file.read_loudly()?.lines() {
            if line.starts_with("[package]") {
                package_reached = true;
            }

            if line.starts_with("version")
                && line.contains(&self.old_version)
                && package_reached
                && !package_updated
            {
                package_updated = true;
                buffer.append_as_line(
                    line.replace(&self.old_version, &self.new_version),
                );
            } else {
                buffer.append_as_line(line);
            }
        }

        buffer.truncate_loudly(file)
    }

    fn edit_citation_cff(&self, file: &PathBuf) -> Result<()> {
        let mut buffer = String::new();

        for line in file.read_loudly()?.lines() {
            if line.starts_with("version:") {
                buffer.append_as_line(
                    line.replace(&self.old_version, &self.new_version),
                );
            } else if line.starts_with("date-released:") {
                buffer.append_as_line(format!(
                    "date-released: {}",
                    chrono::Local::now().format("%Y-%m-%d")
                ));
            } else {
                buffer.append_as_line(line);
            }
        }

        buffer.truncate_loudly(file)
    }

    fn edit_normal_file(&self, file: &PathBuf) -> Result<()> {
        file.read_loudly()?
            .replace(&self.old_version, &self.new_version)
            .truncate_loudly(file)
    }

    fn main(&mut self) -> Result<()> {
        macro_rules! for_file_in {
            ($v:ident { $( $n:literal -> $m:ident ),+ }) => {
                for file in &self.cli.$v {
                    match file
                        .file_name()
                        .ok_or(ExitCode::Usage)?
                        .to_str()
                        .ok_or(ExitCode::DataErr)?
                    {
                        $(
                            $n => self.$m(file)?,
                        )+
                        "Cargo.lock" => self.edit_cargo_lock(file)?,
                        _ => match file.extension() {
                            None => self.edit_normal_file(file)?,
                            Some(extension) => {
                                match extension
                                    .to_str()
                                    .ok_or(ExitCode::DataErr)?
                                {
                                    "cff" => self.edit_citation_cff(file)?,
                                    _ => self.edit_normal_file(file)?,
                                }
                            }
                        }
                    }
                }
            };
        }

        self.determine_new_version()?;
        for_file_in!(file_to_edit { "Cargo.toml" -> edit_cargo_toml });
        for_file_in!(file_to_rewrite { "Cargo.toml" -> rewrite_cargo_toml });

        Ok(())
    }

    fn rewrite_cargo_toml(&self, file: &PathBuf) -> Result<()> {
        if let Ok(mut manifest) = file.read_loudly()?.parse::<toml::Table>() {
            if manifest.get("package").is_some() {
                if manifest["package"].get("version").is_some() {
                    manifest["package"]["version"] =
                        self.new_version.clone().into();
                    manifest.to_string().truncate_loudly(file)?;

                    Ok(())
                } else {
                    ceprintlns!(
                      "Cargo.toml"!Red,
                      "does not contain a `package.version` field."
                    );

                    Err(ExitCode::DataErr)
                }
            } else {
                ceprintlns!(
                    "Cargo.toml"!Red,
                    "does not contain a `package` section."
                );

                Err(ExitCode::DataErr)
            }
        } else {
            ceprintlns!("Cargo.toml"!Red, "does not seem to be valid TOML.");
            Err(ExitCode::DataErr)
        }
    }
}

/******************************************************************************/
