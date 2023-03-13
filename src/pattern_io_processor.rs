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

//! Convenient implementations of IO processing patterns.

use crate::{PatternReader, PatternWriter};
use sysexits::ExitCode;

/// Read some input, process it, and write it to the intended destination.
pub trait PatternIOProcessor {
  /// Process input in a given manner and write the output to a certain stream.
  ///
  /// Processing input is a common task.  Often, this input originates from a
  /// stream and needs to be written to another stream after processing it.
  /// The traits [`PatternReader`][PatternReader] and
  /// [`PatternWriter`][PatternWriter] offer semantics to handle reading from
  /// and writing to streams.  This method now adds a convenient connection
  /// between them as it is designed to be applied on input processing functions
  /// and closures.
  ///
  /// The result of the operation is indicated by the return value which
  /// originates from the reading and writing processes and can be propagated to
  /// the main function.
  fn process(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;
}

impl<T: Fn(String) -> String> PatternIOProcessor for T {
  fn process(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    match input.read_string(show_error_messages) {
      Ok(lines) => {
        output.write_string(&self(lines), append, show_error_messages)
      }
      Err(code) => code,
    }
  }
}

/******************************************************************************/
