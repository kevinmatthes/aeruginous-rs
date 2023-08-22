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

use crate::{ceprintlns, ReadFile};
use std::path::PathBuf;
use sysexits::Result;

/// Complain about certain stylistic issues.
#[allow(clippy::struct_excessive_bools)]
#[derive(clap::Parser, Clone)]
#[clap(visible_aliases = ["aercom"])]
pub struct Complain {
    /// The files to analyse.
    files: Vec<PathBuf>,

    /// Whether to ignore CRLFs.
    #[arg(long)]
    ignore_carriage_return_line_feeds: bool,

    /// Whether to ignore line width issues.
    #[arg(long)]
    ignore_line_width_issues: bool,

    /// Whether to ignore if a file should not be terminated by a line feed.
    #[arg(long)]
    ignore_missing_final_line_feed: bool,

    /// Whether to ignore the usage of mixed indentation units.
    #[arg(long)]
    ignore_mixed_indentation: bool,

    /// Whether to ignore tabs within lines.
    #[arg(long)]
    ignore_tabs_within_lines: bool,

    /// Whether to ignore TWS.
    #[arg(long)]
    ignore_trailing_white_space_characters: bool,

    /// Whether to ignore the usage of wrong indentation units.
    #[arg(long)]
    ignore_wrong_indentation: bool,

    /// The indentation unit.
    #[arg(default_value = "spaces", long, short)]
    indent_by: IndentationUnit,

    /// The maximum line width to check for.
    #[arg(
        default_value = "80",
        long,
        short,
        visible_aliases = ["length", "line", "width"]
    )]
    line_width: usize,
}

impl Complain {
    /// Process the input data.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`crate::ColourMessage`]
    /// - [`crate::ReadFile`]
    pub fn main(&self) -> Result<()> {
        self.wrap().main()
    }

    fn wrap(&self) -> Logic {
        Logic {
            cli: self.clone(),
            data: String::new(),
            errors: 0,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
enum IndentationUnit {
    #[default]
    Spaces,
    Tabs,
}

crate::enum_trait!(IndentationUnit {
    Spaces <-> "spaces",
    Tabs <-> "tabs"
});

struct Logic {
    cli: Complain,
    data: String,
    errors: usize,
}

impl Logic {
    fn aercom_0001(&mut self) -> Result<()> {
        if !self.data.ends_with('\n') {
            self.errors += 1;

            ceprintlns!(
                "AERCOM-0001"!Green,
                "File not terminated by line feed."
            );
        }

        Ok(())
    }

    fn aercom_0002(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.split_inclusive('\n') {
            if l.ends_with("\r\n") {
                self.errors += 1;

                ceprintlns!(
                    "AERCOM-0002"!Yellow,
                    "CRLF in line {line}."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn aercom_0003(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            let c = l.chars().count();

            if c > self.cli.line_width {
                self.errors += 1;

                ceprintlns!(
                    "AERCOM-0003"!Red,
                    "Line {line} is {} character(s) too long.",
                    c - self.cli.line_width
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn aercom_0004(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if l.ends_with(char::is_whitespace) {
                self.errors += 1;

                ceprintlns!(
                    "AERCOM-0004"!Green,
                    "TWS in line {line}."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn aercom_0005(&mut self) -> Result<()> {
        let mut line = 1;
        let trigger = match self.cli.indent_by {
            IndentationUnit::Spaces => '\t',
            IndentationUnit::Tabs => ' ',
        };

        for l in self.data.lines() {
            if l.starts_with(trigger) {
                self.errors += 1;

                ceprintlns!(
                    "AERCOM-0005"!Green,
                    "Line {line} indented by {}.",
                    match trigger {
                        '\t' => "tabs",
                        ' ' => "spaces",
                        _ => unreachable!(),
                    }
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn aercom_0006(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if let Some((indentation, _)) =
                l.split_once(|c| !char::is_whitespace(c))
            {
                if indentation.contains('\t') && indentation.contains(' ') {
                    self.errors += 1;

                    ceprintlns!(
                        "AERCOM-0006"!Yellow,
                        "Line {line} is indented by both spaces and tabs."
                    );
                }
            }

            line += 1;
        }

        Ok(())
    }

    fn aercom_0007(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if l.trim().contains('\t') {
                self.errors += 1;

                ceprintlns!(
                    "AERCOM-0007"!Yellow,
                    "Tabs within line {line}."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn complain(&mut self, f: &PathBuf) -> Result<()> {
        self.data = f.read()?;

        if !self.cli.ignore_missing_final_line_feed {
            self.aercom_0001()?;
        }

        if !self.cli.ignore_carriage_return_line_feeds {
            self.aercom_0002()?;
        }

        if !self.cli.ignore_line_width_issues {
            self.aercom_0003()?;
        }

        if !self.cli.ignore_trailing_white_space_characters {
            self.aercom_0004()?;
        }

        if !self.cli.ignore_wrong_indentation {
            self.aercom_0005()?;
        }

        if !self.cli.ignore_mixed_indentation {
            self.aercom_0006()?;
        }

        if !self.cli.ignore_tabs_within_lines {
            self.aercom_0007()?;
        }

        ceprintlns!("  ˇ;{\"};ˇ  "!Blue, "{} {}", self.errors, f.display());
        self.errors = 0;

        Ok(())
    }

    fn main(&mut self) -> Result<()> {
        for f in self.cli.files.clone() {
            self.complain(&f)?;
        }

        if self.errors == 0 {
            Ok(())
        } else {
            Err(sysexits::ExitCode::DataErr)
        }
    }
}

/******************************************************************************/
