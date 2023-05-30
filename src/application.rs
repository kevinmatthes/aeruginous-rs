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
  #[command(aliases = ["changelog"])]
  CommentChanges {
    /// Work with the commit messages' bodies instead of their summaries.
    #[arg(long, short = 'b')]
    body: bool,

    /// Only these categories shall be used to generate comments.
    #[arg(long, short = 'c')]
    category: Vec<String>,

    /// The delimiter to separate a category from the change description.
    #[arg(long, short = 'd')]
    delimiter: String,

    /// The count of commits to analyse, defaulting to infinity, if omitted.
    #[arg(aliases = ["count"], long, short = 'n')]
    depth: Option<usize>,

    /// The target format of the resulting fragment.
    #[arg(
      aliases = ["format"],
      default_value = "rst",
      long,
      short = 'f',
      value_parser = |f: &str| {
        if ["md", "ron", "rst"].contains(&f) {
          Ok(f.to_string())
        } else {
          Err(format!("extension '{f}' is not supported, yet"))
        }
      }
    )]
    extension: String,

    /// The default category to assign.
    #[arg(long, short = 'C')]
    fallback_category: Option<String>,

    /// The heading's level in the resulting fragment.
    #[arg(
      aliases = ["level"],
      default_value = "3",
      long,
      short = 'H',
      value_parser = clap::value_parser!(u8).range(1..=3)
    )]
    heading: u8,

    /// Set categories Added, Changed, Deprecated, Fixed, Removed, and Security.
    #[arg(long, short = 'k')]
    keep_a_changelog: bool,

    /// The hyperlinks to add as comments.
    #[arg(aliases = ["hyperlink"], long, short = 'l')]
    link: Vec<String>,

    /// The directory to write the generated fragment to.
    #[arg(
      aliases = ["dir", "directory"],
      default_value = ".",
      long = "output",
      short = 'o'
    )]
    output_directory: String,

    /// The hyperlinks' targets.
    #[arg(long, short = 't')]
    target: Vec<String>,
  },

  /*
  /// Rate an Aeruginous Graph Description (AGD).
  #[command(aliases = ["agd"])]
  GraphDescription {
    /// The AGD file to read.
    #[arg(short = 'i')]
    input_file: Option<PathBuf>,
  },
  */
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
    #[arg(long = "input", short = 'i')]
    input_file: Vec<PathBuf>,

    /// The Markdown file to write to, defaulting to [`std::io::Stdout`], if
    /// omitted.
    #[arg(long = "output", short = 'o')]
    output_file: Option<PathBuf>,
  },

  /// Remove CRLFs from the given file.
  Uncrlf {
    /// The file to edit; overrides `input_file` and `output_file`.
    #[arg(long = "edit", short = 'e')]
    file_to_edit: Option<PathBuf>,

    /// The file to read from, defaulting to [`std::io::Stdin`], if omitted.
    #[arg(long = "input", short = 'i')]
    input_file: Option<PathBuf>,

    /// The file to write to, defaulting to [`std::io::Stdout`], if omitted.
    #[arg(long = "output", short = 'o')]
    output_file: Option<PathBuf>,
  },
}

impl Action {
  /// Extract Markdown code from Rust documentation comments.
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
      Self::CommentChanges {
        body,
        category,
        delimiter,
        depth,
        extension,
        fallback_category,
        heading,
        keep_a_changelog,
        link,
        target,
        output_directory,
      } => crate::CommentChangesData::new(
        *depth,
        delimiter.to_string(),
        link
          .iter()
          .zip(target.iter())
          .map(|(a, b)| (a.to_string(), b.to_string()))
          .collect(),
        if *keep_a_changelog {
          let mut categories = vec![
            "Added".to_string(),
            "Changed".to_string(),
            "Deprecated".to_string(),
            "Fixed".to_string(),
            "Removed".to_string(),
            "Security".to_string(),
          ];
          categories.append(&mut category.clone());
          categories
        } else {
          category.clone()
        },
        *body,
      )
      .main(output_directory, *heading, extension, fallback_category),
      /*
      Self::GraphDescription { input_file } => {
        crate::AeruginousGraphDescription::main(input_file)
      }
      */
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
