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

/// Create a new Code Workspace.
#[cfg(feature = "mkcws")]
#[derive(clap::Parser, Clone)]
pub struct Mkcws {
    /// The directory to link.
    #[arg(long, short)]
    directory: std::path::PathBuf,

    /// The file to write the result to, defaulting to [`std::io::Stdin`], if
    /// omitted.
    #[arg(long, short)]
    output_file: Option<std::path::PathBuf>,
}

#[cfg(feature = "mkcws")]
impl Mkcws {
    /// Create a new Code Workspace.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`crate::PatternWriter::truncate`]
    pub fn main(&self) -> sysexits::Result<()> {
        use crate::PatternWriter;

        self.output_file.truncate(Box::new(
            "{ \"folders\" : [ { \"path\" : \"".to_string()
                + &format!("{}", self.directory.display())
                + "\" } ] }\n",
        ))
    }

    /// Create a new instance.
    pub fn new<T>(directory: T, output_file: Option<T>) -> Self
    where
        std::path::PathBuf: From<T>,
    {
        Self {
            directory: std::path::PathBuf::from(directory),
            output_file: output_file.map(Into::into),
        }
    }
}

/// Extract Markdown code from Rust documentation comments.
#[cfg(feature = "rs2md")]
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
    input_file: Vec<std::path::PathBuf>,

    /// The Markdown file to write to, defaulting to [`std::io::Stdout`], if
    /// omitted.
    #[arg(long = "output", short)]
    output_file: Option<std::path::PathBuf>,
}

#[cfg(feature = "rs2md")]
impl Rs2md {
    /// Extract Markdown code from Rust documentation comments.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`crate::PatternIOProcessor::io`]
    pub fn main(&self) -> sysexits::Result<()> {
        use crate::PatternIOProcessor;

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
        std::path::PathBuf: From<T>,
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

/// Remove CRLFs from the given file.
#[cfg(feature = "uncrlf")]
#[derive(clap::Parser, Clone)]
pub struct Uncrlf {
    /// The file to edit; overrides `input_file` and `output_file`.
    #[arg(long = "edit", short = 'e')]
    file_to_edit: Option<std::path::PathBuf>,

    /// The file to read from, defaulting to [`std::io::Stdin`], if omitted.
    #[arg(long = "input", short)]
    input_file: Option<std::path::PathBuf>,

    /// The file to write to, defaulting to [`std::io::Stdout`], if omitted.
    #[arg(long = "output", short)]
    output_file: Option<std::path::PathBuf>,
}

#[cfg(feature = "uncrlf")]
impl Uncrlf {
    /// Remove CRLFs from the given file.
    ///
    /// # Errors
    ///
    /// See
    ///
    /// - [`crate::PatternIOProcessor::io`]
    pub fn main(&self) -> sysexits::Result<()> {
        use crate::{PatternIOProcessor, Prefer};

        (|mut s: String| {
            s.retain(|c| c != '\r');
            s
        })
        .io(
            self.input_file.prefer(self.file_to_edit.clone()),
            self.output_file.prefer(self.file_to_edit.clone()),
        )
    }

    /// Create a new instance.
    pub fn new<T>(
        input_file: Option<T>,
        output_file: Option<T>,
        file_to_edit: Option<T>,
    ) -> Self
    where
        std::path::PathBuf: From<T>,
    {
        Self {
            file_to_edit: file_to_edit.map(Into::into),
            input_file: input_file.map(Into::into),
            output_file: output_file.map(Into::into),
        }
    }
}

/******************************************************************************/
