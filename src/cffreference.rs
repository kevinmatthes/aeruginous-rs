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

use crate::{AppendAsLine, PatternReader, PatternWriter};
use std::path::PathBuf;
use sysexits::Result;

/// Extract the citation information from a given and valid CFF file.
#[derive(clap::Parser, Clone)]
#[command(aliases = ["cffref", "cff-reference"])]
pub struct Cffreference {
  /// The CFF file to read from, defaulting to [`std::io::Stdin`], if omitted.
  #[arg(long = "input", short)]
  input_file: Option<PathBuf>,

  /// The CFF file to write to, defaulting to [`std::io::Stdout`], if omitted.
  #[arg(long = "output", short)]
  output_file: Option<PathBuf>,
}

impl Cffreference {
  /// Process the input data.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`crate::PatternBuffer::try_into_string`]
  /// - [`PatternReader::read`]
  /// - [`PatternWriter::append`]
  pub fn main(&self) -> Result<()> {
    self.wrap().main()
  }

  /// Create a new instance.
  #[must_use]
  pub const fn new(
    input_file: Option<PathBuf>,
    output_file: Option<PathBuf>,
  ) -> Self {
    Self {
      input_file,
      output_file,
    }
  }

  fn wrap(&self) -> Logic {
    Logic {
      cff_data: String::new(),
      cff_reference: String::new(),
      io: self.clone(),
      preferred_citation_reached: false,
      properties: Properties::default(),
      references_reached: false,
    }
  }
}

struct Logic {
  cff_data: String,
  cff_reference: String,
  io: Cffreference,
  preferred_citation_reached: bool,
  properties: Properties,
  references_reached: bool,
}

impl Logic {
  fn logic(&mut self, s: &str) -> String {
    for line in s.lines() {
      if self.references_reached
        && !matches!(line.chars().next(), Some(' ' | '-'))
      {
        self.references_reached = false;
      }

      if !line.is_empty()
        && !line.starts_with('#')
        && !line.starts_with("---")
        && !line.starts_with("...")
        && !line.starts_with("cff-version:")
        && !line.starts_with("message:")
        && !line.starts_with("references:")
        && !self.references_reached
      {
        if line.starts_with("preferred-citation:") {
          self.properties.find_preferred_citation();
        } else if line.starts_with("type:") {
          self.properties.find_type();
        }

        self.cff_data.append_as_line(line);
      } else if line.starts_with("references:") {
        self.references_reached = true;
      }
    }

    if self.properties.has_preferred_citation() {
      for line in self.cff_data.lines() {
        if self.preferred_citation_reached && line.starts_with(' ') {
          self
            .cff_reference
            .push_str(&("  ".to_string() + line + "\n"));
        } else if self.preferred_citation_reached {
          self.preferred_citation_reached = false;
        }

        if line.starts_with("preferred-citation:") {
          self.preferred_citation_reached = true;
        }
      }

      let mut lines = self.cff_reference.lines();

      lines
        .next()
        .map_or_else(String::new, |l| format!("  - {}\n", l.trim()))
        + &lines.map(|l| format!("{l}\n")).collect::<String>()
    } else {
      let mut lines = self.cff_data.lines();

      (if self.properties.has_type() {
        lines
          .next()
          .map_or_else(String::new, |l| format!("  - {}\n", l.trim()))
      } else {
        "  - type: software\n".to_string()
      }) + &lines.map(|l| format!("    {l}\n")).collect::<String>()
    }
  }

  fn main(&mut self) -> Result<()> {
    self.io.output_file.clone().append(Box::new(
      self.logic(&self.io.input_file.read()?.try_into_string()?),
    ))
  }
}

#[derive(Default)]
enum Properties {
  BothPreferredCitationAndType,

  #[default]
  NeitherPreferredCitationNorType,

  PreferredCitation,
  Type,
}

impl Properties {
  fn find_preferred_citation(&mut self) {
    if matches!(self, Self::NeitherPreferredCitationNorType) {
      *self = Self::PreferredCitation;
    } else if matches!(self, Self::Type) {
      *self = Self::BothPreferredCitationAndType;
    }
  }

  fn find_type(&mut self) {
    if matches!(self, Self::NeitherPreferredCitationNorType) {
      *self = Self::Type;
    } else if matches!(self, Self::PreferredCitation) {
      *self = Self::BothPreferredCitationAndType;
    }
  }

  const fn has_preferred_citation(&self) -> bool {
    matches!(
      self,
      Self::PreferredCitation | Self::BothPreferredCitationAndType
    )
  }

  const fn has_type(&self) -> bool {
    matches!(self, Self::Type | Self::BothPreferredCitationAndType)
  }
}

/******************************************************************************/
