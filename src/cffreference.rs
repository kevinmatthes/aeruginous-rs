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

use crate::{AppendAsLine, PatternIOProcessor};
use std::path::PathBuf;

/// Extract the citation information from a given and valid CFF file.
#[derive(clap::Parser)]
#[command(aliases = ["cffref", "cff-reference"])]
pub struct Cffreference {
  /// The CFF file to read from, defaulting to [`std::io::Stdin`], if omitted.
  #[arg(long = "input", short = 'i')]
  input_file: Option<PathBuf>,

  /// The CFF file to write to, defaulting to [`std::io::Stdout`], if omitted.
  #[arg(long = "output", short = 'o')]
  output_file: Option<PathBuf>,
}

impl Cffreference {
  /// Extract the citation information from a given and valid CFF file.
  fn logic(s: &str) -> String {
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

        buffer.append_as_line(line);
      } else if line.starts_with("references:") {
        references = true;
      }
    }

    if has_preferred_citation {
      let mut preferred_citation_reached = false;
      let mut result = String::new();

      for line in buffer.lines() {
        if preferred_citation_reached && line.starts_with(' ') {
          result.push_str(&("  ".to_string() + line + "\n"));
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
        .map_or_else(String::new, |l| "  - ".to_string() + l.trim() + "\n")
        + &lines.map(|l| l.to_string() + "\n").collect::<String>()
    } else {
      let mut lines = buffer.lines();

      (if has_type {
        lines
          .next()
          .map_or_else(String::new, |l| "  - ".to_string() + l.trim() + "\n")
      } else {
        "  - type: software\n".to_string()
      }) + &lines
        .map(|l| "    ".to_string() + l + "\n")
        .collect::<String>()
    }
  }

  /// Run the CFF data extraction.
  ///
  /// # Errors
  ///
  /// See [`PatternIOProcessor::io_append`].
  pub fn main(&self) -> sysexits::Result<()> {
    |s: String| -> String { Self::logic(&s) }
      .io_append(&self.input_file, &self.output_file)
  }
}

/******************************************************************************/
