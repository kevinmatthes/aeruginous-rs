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
  #[arg(long = "input", short = 'i')]
  input_file: Option<PathBuf>,

  /// The CFF file to write to, defaulting to [`std::io::Stdout`], if omitted.
  #[arg(long = "output", short = 'o')]
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

  /// Create a logic instance.
  fn wrap(&self) -> CffreferenceLogic {
    CffreferenceLogic {
      cff_data: String::new(),
      cff_reference: String::new(),
      io: self.clone(),
      preferred_citation_reached: false,
      properties: CffreferenceProperties::default(),
      references_reached: false,
    }
  }
}

/// The internal logic of this mode.
struct CffreferenceLogic {
  /// The held CFF data.
  cff_data: String,

  /// The CFF reference object to write to the output stream.
  cff_reference: String,

  /// The streams to operate on.
  io: Cffreference,

  /// Whether the `preferred-citation` was already reached.
  preferred_citation_reached: bool,

  /// The properties of the given CFF file.
  properties: CffreferenceProperties,

  /// Whether the `references` were already reached.
  references_reached: bool,
}

impl CffreferenceLogic {
  /// Extract the citation information from a given and valid CFF file.
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

  /// Run the CFF data extraction.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`crate::PatternBuffer::try_into_string`]
  /// - [`PatternReader::read`]
  /// - [`PatternWriter::append`]
  fn main(&mut self) -> Result<()> {
    self.io.output_file.clone().append(Box::new(
      self.logic(&self.io.input_file.read()?.try_into_string()?),
    ))
  }
}

/// Some information about the found information.
#[derive(Default)]
enum CffreferenceProperties {
  /// The given CFF has both an explicit type and a preferred citation.
  BothPreferredCitationAndType,

  /// The given CFF has neither an explicit type nor a preferred citation.
  #[default]
  NeitherPreferredCitationNorType,

  /// The given CFF has a preferred citation.
  PreferredCitation,

  /// The given CFF has an explicit type.
  Type,
}

impl CffreferenceProperties {
  /// A preferred citation was found.
  fn find_preferred_citation(&mut self) {
    if matches!(self, Self::NeitherPreferredCitationNorType) {
      *self = Self::PreferredCitation;
    } else if matches!(self, Self::Type) {
      *self = Self::BothPreferredCitationAndType;
    }
  }

  /// An explicit type was found.
  fn find_type(&mut self) {
    if matches!(self, Self::NeitherPreferredCitationNorType) {
      *self = Self::Type;
    } else if matches!(self, Self::PreferredCitation) {
      *self = Self::BothPreferredCitationAndType;
    }
  }

  /// Is there a preferred citation defined?
  const fn has_preferred_citation(&self) -> bool {
    matches!(
      self,
      Self::PreferredCitation | Self::BothPreferredCitationAndType
    )
  }

  /// Is there an explicit type given?
  const fn has_type(&self) -> bool {
    matches!(self, Self::Type | Self::BothPreferredCitationAndType)
  }
}

/******************************************************************************/
