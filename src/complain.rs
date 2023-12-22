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
    /// Ignore CRLFs.
    pub fn ignore_carriage_return_line_feeds(&mut self) {
        self.ignore_carriage_return_line_feeds = true;
    }

    /// Ignore too long lines.
    pub fn ignore_line_width_issues(&mut self) {
        self.ignore_line_width_issues = true;
    }

    /// Ignore missing trailing newline characters.
    pub fn ignore_missing_final_line_feed(&mut self) {
        self.ignore_missing_final_line_feed = true;
    }

    /// Ignore the application of multiple indentation units.
    pub fn ignore_mixed_indentation(&mut self) {
        self.ignore_mixed_indentation = true;
    }

    /// Ignore tab characters in input lines.
    pub fn ignore_tabs_within_lines(&mut self) {
        self.ignore_tabs_within_lines = true;
    }

    /// Ignore lines ending with spaces and / or tab characters.
    pub fn ignore_trailing_white_space_characters(&mut self) {
        self.ignore_trailing_white_space_characters = true;
    }

    /// Ignore applications of the opposite indentation unit.
    pub fn ignore_wrong_indentation(&mut self) {
        self.ignore_wrong_indentation = true;
    }

    /// Set another indentation unit.
    pub fn indent_by(&mut self, i: IndentationUnit) {
        self.indent_by = i;
    }

    /// Process the input data.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`crate::ColourMessage`]
    /// - [`crate::ReadFile`]
    /// - [`sysexits::ExitCode::DataErr`]
    pub fn main(&self) -> Result<()> {
        self.wrap().main()
    }

    /// Create a new instance.
    #[must_use]
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self {
            files,
            ignore_carriage_return_line_feeds: false,
            ignore_line_width_issues: false,
            ignore_missing_final_line_feed: false,
            ignore_mixed_indentation: false,
            ignore_tabs_within_lines: false,
            ignore_trailing_white_space_characters: false,
            ignore_wrong_indentation: false,
            indent_by: IndentationUnit::Spaces,
            line_width: 80,
        }
    }

    /// Process this instance.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`Self::main`]
    pub fn process(&self) -> Result<usize> {
        self.wrap().process()
    }

    /// Query the current state of settings.
    pub const fn state(&self) -> (&Vec<PathBuf>, [bool; 7], IndentationUnit, usize) {
        (
            &self.files,
            [
                self.ignore_carriage_return_line_feeds,
                self.ignore_line_width_issues,
                self.ignore_missing_final_line_feed,
                self.ignore_mixed_indentation,
                self.ignore_tabs_within_lines,
                self.ignore_trailing_white_space_characters,
                self.ignore_wrong_indentation,
            ],
            self.indent_by,
            self.line_width,
        )
    }

    fn wrap(&self) -> Logic {
        Logic {
            cli: self.clone(),
            data: String::new(),
            errors: 0,
            total_errors: 0,
        }
    }
}

impl Default for Complain {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// The possible indentation units.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IndentationUnit {
    /// Indent by spaces.
    #[default]
    Spaces,

    /// Indent by tabs.
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
    total_errors: usize,
}

impl Logic {
    fn ac_0001(&mut self) -> Result<()> {
        if !self.data.ends_with('\n') {
            self.errors += 1;

            ceprintlns!(
                "AC-0001"!Green,
                "File not terminated by line feed."
            );
        }

        Ok(())
    }

    fn ac_0002(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.split_inclusive('\n') {
            if l.ends_with("\r\n") {
                self.errors += 1;

                ceprintlns!(
                    "AC-0002"!Yellow,
                    "CRLF in line {line}."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn ac_0003(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            let c = l.chars().count();

            if c > self.cli.line_width {
                self.errors += 1;

                ceprintlns!(
                    "AC-0003"!Red,
                    "Line {line} is {} character(s) too long.",
                    c - self.cli.line_width
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn ac_0004(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if l.ends_with(char::is_whitespace) {
                self.errors += 1;

                ceprintlns!(
                    "AC-0004"!Green,
                    "TWS in line {line}."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn ac_0005(&mut self) -> Result<()> {
        let mut line = 1;
        let trigger = match self.cli.indent_by {
            IndentationUnit::Spaces => '\t',
            IndentationUnit::Tabs => ' ',
        };

        for l in self.data.lines() {
            if l.starts_with(trigger) {
                self.errors += 1;

                ceprintlns!(
                    "AC-0005"!Green,
                    "Line {line} indented by {}.",
                    if trigger == '\t' { "tabs" } else { "spaces" }
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn ac_0006(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if l.split_once(|c| !char::is_whitespace(c)).is_some_and(
                |(indentation, _)| {
                    indentation.contains('\t') && indentation.contains(' ')
                },
            ) {
                self.errors += 1;

                ceprintlns!(
                    "AC-0006"!Yellow,
                    "Line {line} is indented by both spaces and tabs."
                );
            }

            line += 1;
        }

        Ok(())
    }

    fn ac_0007(&mut self) -> Result<()> {
        let mut line = 1;

        for l in self.data.lines() {
            if l.trim().contains('\t') {
                self.errors += 1;

                ceprintlns!(
                    "AC-0007"!Yellow,
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
            self.ac_0001()?;
        }

        if !self.cli.ignore_carriage_return_line_feeds {
            self.ac_0002()?;
        }

        if !self.cli.ignore_line_width_issues {
            self.ac_0003()?;
        }

        if !self.cli.ignore_trailing_white_space_characters {
            self.ac_0004()?;
        }

        if !self.cli.ignore_wrong_indentation {
            self.ac_0005()?;
        }

        if !self.cli.ignore_mixed_indentation {
            self.ac_0006()?;
        }

        if !self.cli.ignore_tabs_within_lines {
            self.ac_0007()?;
        }

        ceprintlns!("ˇ;{\"};ˇ"!Blue, "{} {}", self.errors, f.display());
        self.total_errors += self.errors;
        self.errors = 0;

        Ok(())
    }

    fn main(&mut self) -> Result<()> {
        if self.process()? == 0 {
            Ok(())
        } else {
            Err(sysexits::ExitCode::DataErr)
        }
    }

    fn process(&mut self) -> Result<usize> {
        for f in self.cli.files.clone() {
            self.complain(&f)?;
        }

        Ok(self.total_errors)
    }
}

/******************************************************************************/
