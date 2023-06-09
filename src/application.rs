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
  AppendAsLine, PatternIOProcessor, PatternReader, PatternWriter, Prefer,
};
use clap::{Parser, Subcommand};
use std::{io::BufRead, path::PathBuf, str::FromStr};
use sysexits::Result;

/// The supported application modes.
///
/// Depending on the given command line arguments, `aeruginous` will show a
/// different behaviour.  Each variant of this enum will trigger the associated
/// application mode in order to fulfill a certain task.  The variants
/// themselves, in turn, are defined as anonymous structs with their fields
/// being the accepted command line arguments and options, respectively.
#[derive(Subcommand)]
pub enum Action {
  /// Extract the citation information from a given and valid CFF file.
  Cffreference(crate::Cffreference),

  /// Increment the release date in CFFs.
  #[command(aliases = ["cffrel", "cff-rel", "cffreleasetoday"])]
  CffReleaseToday {
    /// The file to work on.
    file_to_edit: PathBuf,
  },

  /// Create comments on the commits of a branch in this repository.
  CommentChanges(crate::CommentChanges),

  /*
  /// Rate an Aeruginous Graph Description (AGD).
  #[command(aliases = ["agd"])]
  GraphDescription {
    /// The AGD file to read.
    #[arg(short = 'i')]
    input_file: Option<PathBuf>,
  },
  */
  /// Increment a hard-coded version string in some files.
  #[command(aliases = ["incver", "inc-ver"])]
  IncrementVersion {
    /// The files to work on.
    #[arg(long = "edit", short = 'e')]
    file_to_edit: Vec<PathBuf>,

    /// The old version to search for and replace.
    #[arg(long, short = 'v')]
    old_version: String,

    /// The increment range.
    #[arg(long, short)]
    range: crate::VersionRange,
  },

  /// Interact with RON CHANGELOGs.
  Ronlog(crate::Ronlog),

  /// Extract Markdown code from Rust documentation comments.
  Rs2md {
    /// Whether to extract Rust documentation line comments starting with `///`.
    #[arg(long = "inner")]
    extract_inner: bool,

    /// Whether to extract Rust documentation line comments starting with `//!`.
    #[arg(long = "outer")]
    extract_outer: bool,

    /// The Rust files to read from, defaulting to [`std::io::Stdin`], if
    /// omitted.
    #[arg(long = "input", short)]
    input_file: Vec<PathBuf>,

    /// The Markdown file to write to, defaulting to [`std::io::Stdout`], if
    /// omitted.
    #[arg(long = "output", short)]
    output_file: Option<PathBuf>,
  },

  /// Remove CRLFs from the given file.
  Uncrlf {
    /// The file to edit; overrides `input_file` and `output_file`.
    #[arg(long = "edit", short = 'e')]
    file_to_edit: Option<PathBuf>,

    /// The file to read from, defaulting to [`std::io::Stdin`], if omitted.
    #[arg(long = "input", short)]
    input_file: Option<PathBuf>,

    /// The file to write to, defaulting to [`std::io::Stdout`], if omitted.
    #[arg(long = "output", short)]
    output_file: Option<PathBuf>,
  },
}

impl Action {
  fn rs2md(s: &str, extract_inner: bool, extract_outer: bool) -> String {
    s.lines()
      .map(str::trim_start)
      .filter(|l| {
        (extract_inner && l.starts_with("///"))
          || (extract_outer && l.starts_with("//!"))
      })
      .map(|l| {
        if l.len() > 3 {
          l.split_at(4).1.trim_end().to_string() + "\n"
        } else {
          "\n".to_string()
        }
      })
      .collect::<String>()
  }

  /// Execute the selected action.
  ///
  /// # Errors
  ///
  /// See [`PatternIOProcessor::io`].
  pub fn run(&self) -> Result<()> {
    match self {
      Self::Cffreference(c) => c.main(),
      Self::CffReleaseToday { file_to_edit } => {
        let mut buffer = String::new();

        for line in
          std::io::BufReader::new(std::fs::File::open(file_to_edit)?).lines()
        {
          let line = line?;

          if line.starts_with("date-released:") {
            buffer.append_as_line(format!(
              "date-released: {}",
              chrono::Local::now().date_naive().format("%Y-%m-%d")
            ));
          } else {
            buffer.append_as_line(line);
          }
        }

        file_to_edit.truncate(Box::new(buffer))
      }
      Self::CommentChanges(c) => c.main(),
      /*
      Self::GraphDescription { input_file } => {
        crate::AeruginousGraphDescription::main(input_file)
      }
      */
      Self::IncrementVersion {
        file_to_edit,
        old_version,
        range,
      } => {
        let v = crate::Version::from_str(old_version)?
          .increment(*range)
          .to_string();
        let v = v.strip_prefix('v').unwrap_or(&v);

        for file in file_to_edit {
          file.truncate(Box::new(
            file
              .read()?
              .try_into_string()?
              .split(old_version.strip_prefix('v').unwrap_or(old_version))
              .collect::<Vec<&str>>()
              .join(v.strip_prefix('v').unwrap_or(v)),
          ))?;
        }

        Ok(())
      }
      Self::Ronlog(r) => r.main(),
      Self::Rs2md {
        extract_inner,
        extract_outer,
        input_file,
        output_file,
      } => (|s: String| -> String {
        Self::rs2md(&s, *extract_inner, *extract_outer)
      })
      .io(input_file, output_file),
      Self::Uncrlf {
        file_to_edit,
        input_file,
        output_file,
      } => |mut s: String| -> String {
        s.retain(|c| c != '\r');
        s
      }
      .io(
        input_file.prefer(file_to_edit.clone()),
        output_file.prefer(file_to_edit.clone()),
      ),
    }
  }
}

/// The command line argument configuration.
#[derive(Parser)]
#[clap(about, version)]
pub struct Clap {
  /// The action to perform.
  #[clap(subcommand)]
  action: Action,
}

crate::getters!(@ref Clap { action: Action });

/******************************************************************************/
