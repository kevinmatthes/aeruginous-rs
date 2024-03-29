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

use crate::{AppendAsLine, PatternWriter};
use clap::{Parser, Subcommand};
use std::{io::BufRead, path::PathBuf};
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
    /// Create a CFF from a given manifest file.
    #[cfg(feature = "cff-create")]
    CffCreate(crate::CffCreate),

    /// Extract the citation information from a given and valid CFF file.
    Cffreference(crate::Cffreference),

    /// ⚠️  DEPRECATED.
    ///
    /// Increment the release date in CFFs.
    ///
    /// This application mode is deprecated.  Please use `aeruginous
    /// increment-version [-e|-R] <FILE>` instead.
    #[command(aliases = ["cffrel", "cff-rel", "cffreleasetoday"])]
    #[deprecated(since = "3.6.2", note = "use `IncrementVersion` instead")]
    CffReleaseToday {
        /// The file to work on.
        file_to_edit: PathBuf,
    },

    /// Create comments on the commits of a branch in this repository.
    CommentChanges(crate::CommentChanges),

    /// Complain about certain stylistic issues.
    Complain(crate::Complain),

    /*
    /// Rate an Aeruginous Graph Description (AGD).
    #[command(visible_aliases = ["agd"])]
    GraphDescription {
      /// The AGD file to read.
      #[arg(short = 'i')]
      input_file: Option<PathBuf>,
    },
    */
    /// Increment a hard-coded version string in some files.
    IncrementVersion(crate::IncrementVersion),

    /// Create a new Code Workspace.
    #[cfg(feature = "mkcws")]
    Mkcws(crate::Mkcws),

    /// Interact with RON CHANGELOGs.
    Ronlog(crate::Ronlog),

    /// Extract Markdown code from Rust documentation comments.
    #[cfg(feature = "rs2md")]
    Rs2md(crate::Rs2md),

    /// Remove CRLFs from the given file.
    #[cfg(feature = "uncrlf")]
    Uncrlf(crate::Uncrlf),
}

impl Action {
    /// Execute the selected action.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn run(&self) -> Result<()> {
        match self {
            #[cfg(feature = "cff-create")]
            Self::CffCreate(c) => c.main(),
            Self::Cffreference(c) => c.main(),
            #[allow(deprecated)]
            Self::CffReleaseToday { file_to_edit } => {
                crate::ceprintlns!(
                    "DEPRECATED"!Red,
                    "Please use `increment-version` instead."
                );

                let mut buffer = String::new();

                for line in
                    std::io::BufReader::new(std::fs::File::open(file_to_edit)?)
                        .lines()
                {
                    let line = line?;

                    if line.starts_with("date-released:") {
                        buffer.append_as_line(format!(
                            "date-released: {}",
                            chrono::Local::now().format("%Y-%m-%d")
                        ));
                    } else {
                        buffer.append_as_line(line);
                    }
                }

                file_to_edit.truncate(Box::new(buffer))
            }
            Self::CommentChanges(c) => c.main(),
            Self::Complain(c) => c.main(),
            /*
            Self::GraphDescription { input_file } => {
              crate::AeruginousGraphDescription::main(input_file)
            }
            */
            Self::IncrementVersion(i) => i.main(),
            #[cfg(feature = "mkcws")]
            Self::Mkcws(m) => m.main(),
            Self::Ronlog(r) => r.main(),
            #[cfg(feature = "rs2md")]
            Self::Rs2md(r) => r.main(),
            #[cfg(feature = "uncrlf")]
            Self::Uncrlf(u) => u.main(),
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
