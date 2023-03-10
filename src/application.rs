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

use crate::PatternIOProcessor;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use sysexits::ExitCode;

/// The supported application modes.
///
/// Depending on the given command line arguments, `aeruginous` will show a
/// different behaviour.
#[derive(Subcommand)]
pub enum Action {
  /// Extract the citation information from a given and valid CFF file.
  Cffreference {
    /// The CFF file to read from, defaulting to `stdin`, if omitted.
    #[arg(short = 'i')]
    input_file: Option<PathBuf>,

    /// The CFF file to write to, defaulting to `stdout`, if omitted.
    #[arg(short = 'o')]
    output_file: Option<PathBuf>,
  },

  /// Extract Markdown code from Rust documentation comments.
  Rs2md {
    /// Whether to extract Rust documentation line comments starting with `///`.
    #[arg(long = "inner")]
    extract_inner: bool,

    /// Whether to extract Rust documentation line comments starting with `//!`.
    #[arg(long = "outer")]
    extract_outer: bool,

    /// The Rust files to read from, defaulting to `stdin`, if omitted.
    #[arg(short = 'i')]
    input_files: Vec<PathBuf>,

    /// The Markdown file to write to, defaulting to `stdout`, if omitted.
    #[arg(short = 'o')]
    output_file: Option<PathBuf>,
  },
}

impl Action {
  /// Extract the citation information from a given and valid CFF file.
  fn cffreference(
    input_file: &Option<PathBuf>,
    output_file: &Option<PathBuf>,
  ) -> ExitCode {
    |s: String| -> String {
      let mut buffer = String::new();
      let mut has_preferred_citation = false;
      let mut has_type = false;
      let mut references = false;

      for line in s.lines() {
        if references {
          match line.chars().next() {
            Some(' ' | '-') => {}
            _ => {
              references = false;
            }
          }
        }

        if !line.is_empty()
          && !line.starts_with('#')
          && !line.starts_with("---")
          && !line.starts_with("...")
          && !line.starts_with("cff-version:")
          && !line.starts_with("message:")
          && !line.starts_with("references:")
          && !references
        {
          if line.starts_with("preferred-citation:") {
            has_preferred_citation = true;
          } else if line.starts_with("type:") {
            has_type = true;
          }

          buffer.push_str(&(String::from(line) + "\n"));
        } else if line.starts_with("references:") {
          references = true;
        }
      }

      if has_preferred_citation {
        let mut preferred_citation_reached = false;
        let mut result = String::new();

        for line in buffer.lines() {
          if preferred_citation_reached && line.starts_with(' ') {
            result.push_str(&(String::from("  ") + line + "\n"));
          } else if preferred_citation_reached {
            preferred_citation_reached = false;
          }

          if line.starts_with("preferred-citation:") {
            preferred_citation_reached = true;
          }
        }

        let mut lines = result.lines();

        lines
          .next()
          .map_or_else(String::new, |l| String::from("  - ") + l.trim() + "\n")
          + &lines.map(|l| String::from(l) + "\n").collect::<String>()
      } else {
        let mut lines = buffer.lines();

        (if has_type {
          lines.next().map_or_else(String::new, |l| {
            String::from("  - ") + l.trim() + "\n"
          })
        } else {
          String::from("  - type: software\n")
        }) + &lines
          .map(|l| String::from("    ") + l + "\n")
          .collect::<String>()
      }
    }
    .process(input_file, output_file, true, true)
  }

  /// Extract Markdown code from Rust documentation comments.
  fn rs2md(
    extract_inner: bool,
    extract_outer: bool,
    input_files: &Vec<PathBuf>,
    output_file: &Option<PathBuf>,
  ) -> ExitCode {
    |s: String| -> String {
      s.lines()
        .map(str::trim_start)
        .filter(|l| {
          (extract_inner && l.starts_with("///"))
            || (extract_outer && l.starts_with("//!"))
        })
        .map(|l| {
          String::from(l.chars().skip(4).collect::<String>().trim_end()) + "\n"
        })
        .collect::<String>()
    }
    .process(input_files, output_file, false, true)
  }

  /// Execute the selected action.
  #[must_use]
  pub fn run(&self) -> ExitCode {
    match self {
      Self::Cffreference {
        input_file,
        output_file,
      } => Self::cffreference(input_file, output_file),
      Self::Rs2md {
        extract_inner,
        extract_outer,
        input_files,
        output_file,
      } => {
        Self::rs2md(*extract_inner, *extract_outer, input_files, output_file)
      }
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

impl Clap {
  /// Retrieve the selected action.
  #[must_use]
  pub const fn action(&self) -> &Action {
    &self.action
  }
}

/******************************************************************************/
