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

use crate::{PatternReader, PatternWriter};
use std::path::PathBuf;
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
    Ok(lines) => output_file.write_string(&instructions(lines), true),
    Err(code) => code,
  }
}

/******************************************************************************/
