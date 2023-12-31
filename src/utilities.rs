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
use std::path::PathBuf;
use sysexits::Result;

/// Extract Markdown code from Rust documentation comments.
#[derive(clap::Parser, Clone)]
pub struct Rs2md {
    /// Whether to extract Rust documentation comments starting with `///`.
    #[arg(long = "inner")]
    extract_inner: bool,

    /// Whether to extract Rust documentation comments starting with `//!`.
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
}

impl Rs2md {
    /// Extract Markdown code from Rust documentation comments.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`PatternIOProcessor::io`]
    pub fn main(&self) -> Result<()> {
        (|s: String| {
            s.lines()
                .map(str::trim_start)
                .filter(|l| {
                    (self.extract_inner && l.starts_with("///"))
                        || (self.extract_outer && l.starts_with("//!"))
                })
                .map(|l| {
                    if l.len() > 3 {
                        l.split_at(4).1.trim_end().to_string() + "\n"
                    } else {
                        "\n".to_string()
                    }
                })
                .collect::<String>()
        })
        .io(&self.input_file, &self.output_file)
    }

    /// Create a new instance.
    pub fn new<T>(
        input_file: Vec<T>,
        output_file: Option<T>,
        extract_inner: bool,
        extract_outer: bool,
    ) -> Self
    where
        PathBuf: From<T>,
    {
        Self {
            extract_inner,
            extract_outer,
            input_file: {
                let mut i = Vec::new();

                for file in input_file {
                    i.push(file.into());
                }

                i
            },
            output_file: output_file.map(Into::into),
        }
    }
}

/******************************************************************************/
