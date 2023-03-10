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

//! The application's subcommands.

use clap::Subcommand;
use std::{
  fs::File,
  io::{stdin, BufRead, BufReader, Write},
  path::PathBuf,
};
use sysexits::ExitCode;

/// The supported application modes.
///
/// Depending on the given command line arguments, `aeruginous` will show a
/// different behaviour.
#[derive(Subcommand)]
pub enum Action {
  /// Extract Markdown code from Rust documentation comments.
  Rs2md {
    /// Whether to extract Rust documentation line comments starting with `///`.
    #[arg(long = "inner")]
    extract_inner: Option<bool>,

    /// Whether to extract Rust documentation line comments starting with `//!`.
    #[arg(long = "outer")]
    extract_outer: Option<bool>,

    /// The Rust files to read from or `stdin`, if omitted.
    #[arg(short = 'i')]
    input_files: Vec<PathBuf>,

    /// The Markdown file to write to or `stdout`, if omitted.
    #[arg(short = 'o')]
    output_file: Option<PathBuf>,
  },
}

impl Action {
  /// Extract Markdown code from Rust documentation comments.
  fn rs2md(
    extract_inner: &Option<bool>,
    extract_outer: &Option<bool>,
    input_files: &Vec<PathBuf>,
    output_file: &Option<PathBuf>,
  ) -> ExitCode {
    let mut lines = Vec::<u8>::new();

    if !input_files.is_empty() {
      for file in input_files {
        match File::open(file) {
          Ok(file) => match BufReader::new(file).fill_buf() {
            Ok(string) => {
              lines.append(&mut string.to_vec());
            }
            Err(error) => {
              eprintln!("{error}");
              return ExitCode::IoErr;
            }
          },
          Err(error) => {
            eprintln!("{error}");
            return ExitCode::NoInput;
          }
        }
      }
    } else {
      loop {
        let mut line = String::new();

        match stdin().read_line(&mut line) {
          Ok(0) => {
            break;
          }
          Ok(_) => lines.append(&mut line.as_bytes().to_vec()),
          Err(error) => {
            eprintln!("{error}");
            return ExitCode::IoErr;
          }
        }
      }
    }

    match String::from_utf8(lines) {
      Ok(lines) => {
        let lines = lines
          .lines()
          .map(|l| l.trim_start())
          .filter(|l| {
            (match extract_inner {
              Some(boolean) => *boolean,
              None => false,
            } && l.starts_with("///"))
              || (match extract_outer {
                Some(boolean) => *boolean,
                None => false,
              } && l.starts_with("//!"))
          })
          .map(|l| {
            String::from(l.chars().skip(3).collect::<String>().trim()) + "\n"
          })
          .collect::<String>();

        match output_file {
          Some(path) => match File::create(path) {
            Ok(mut file) => match file.write(lines.as_bytes()) {
              Ok(_) => ExitCode::Ok,
              Err(error) => {
                eprintln!("{error}");
                ExitCode::IoErr
              }
            },
            Err(error) => {
              eprintln!("{error}");
              ExitCode::CantCreat
            }
          },
          None => {
            print!("{lines}");
            ExitCode::Ok
          }
        }
      }
      Err(error) => {
        eprintln!("{error}");
        ExitCode::DataErr
      }
    }
  }

  /// Execute the selected action.
  pub fn run(&self) -> ExitCode {
    match self {
      Self::Rs2md {
        extract_inner,
        extract_outer,
        input_files,
        output_file,
      } => Self::rs2md(extract_inner, extract_outer, input_files, output_file),
    }
  }
}

/******************************************************************************/
