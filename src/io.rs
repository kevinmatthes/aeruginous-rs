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

//! Various IO utilities.

use crate::PatternReader;
use std::{fs::File, io::Write, path::PathBuf};
use sysexits::ExitCode;

/// Process given input in a given way and write the output to the given place.
pub fn process_input_files_or_stdin_to_output_file_or_stdout<T>(
  input_files: &Vec<PathBuf>,
  output_file: &Option<PathBuf>,
  instructions: T,
) -> ExitCode
where
  T: Fn(String) -> String,
{
  match input_files.read_string(true) {
    Ok(lines) => {
      write_to_output_file_or_stdout(output_file, &instructions(lines))
    }
    Err(code) => code,
  }
}

/// Write a buffer's content to either an output file or `stdout`.
///
/// The content of the given buffer will be written to the named output file.
/// If no output file is given, the content will be written to `stdout` instead.
///
/// The return value indicates whether the writing process was successful, i.e.,
/// the output file, if given, could be created, there was no loss of data, and
/// no writing error occured.
#[must_use]
pub fn write_to_output_file_or_stdout(
  output_file: &Option<PathBuf>,
  buffer: &str,
) -> ExitCode {
  output_file.as_ref().map_or_else(
    || {
      print!("{buffer}");
      ExitCode::Ok
    },
    |path| match File::create(path) {
      Ok(mut file) => {
        let buffer = buffer.as_bytes();
        match file.write(buffer) {
          Ok(count) => {
            if count == buffer.len() {
              ExitCode::Ok
            } else {
              eprintln!("Writing the buffer did not create an exact copy!");
              ExitCode::IoErr
            }
          }
          Err(error) => {
            eprintln!("{error}");
            ExitCode::IoErr
          }
        }
      }
      Err(error) => {
        eprintln!("{error}");
        ExitCode::CantCreat
      }
    },
  )
}

/******************************************************************************/
