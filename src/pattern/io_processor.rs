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

use crate::{PatternReader, PatternWriter, Result};
use sysexits::ExitCode;

/// Read some input, process it, and write it to the intended destination.
pub trait IOProcessor {
  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.
  ///
  /// In case of a file, `append` shall control whether to *not* truncate it
  /// before writing to it.  `show_error_messages` shall control whether to
  /// write error messages to `stderr`, if appropriate.
  ///
  /// # Errors
  ///
  /// The return value shall indicate whether the operation succeeded.
  /// Implementations should rely on the semantics of [`PatternReader`][Reader]
  /// and [`PatternWriter`][Writer] instead of introducing further exit codes.
  fn behaviour(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
    append: bool,
    show_error_messages: bool,
  ) -> Result<()>;

  /// Truncate the output stream and write error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
  ) -> Result<()> {
    self.behaviour(input, output, false, true)
  }

  /// Do not truncate the output stream and write error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_append(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
  ) -> Result<()> {
    self.behaviour(input, output, true, true)
  }

  /// Neither truncate the output stream nor write error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_append_silently(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
  ) -> Result<()> {
    self.behaviour(input, output, true, false)
  }

  /// Truncate the output stream and do not write error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_silent(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
  ) -> Result<()> {
    self.behaviour(input, output, false, false)
  }

  /// A deprecated synonym for [`behaviour`][IOProcessor::behaviour].
  #[deprecated(note = "Renamed to `behaviour`.", since = "0.2.1")]
  fn process(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    match self.behaviour(input, output, append, show_error_messages) {
      Ok(()) => ExitCode::Ok,
      Err(code) => code,
    }
  }
}

impl<T: Fn(String) -> String> IOProcessor for T {
  fn behaviour(
    &self,
    input: impl PatternReader,
    output: impl PatternWriter,
    append: bool,
    show_error_messages: bool,
  ) -> Result<()> {
    match input.read() {
      Ok(buffer) => match buffer.as_ref().try_into_string() {
        Ok(lines) => {
          output.behaviour(Box::new(self(lines)), append, show_error_messages)
        }
        Err(code) => Err(code),
      },
      Err(code) => Err(code),
    }
  }
}

/******************************************************************************/
