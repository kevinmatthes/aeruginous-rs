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

use crate::{PatternIOProcessor, Prefer};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
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
      Self::CommentChanges(c) => c.main(),
      /*
      Self::GraphDescription { input_file } => {
        crate::AeruginousGraphDescription::main(input_file)
      }
      */
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
