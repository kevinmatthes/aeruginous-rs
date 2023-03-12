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

//! Convenient implementations of common IO writing patterns.

use std::{fs::File, io::Write, path::PathBuf};
use sysexits::ExitCode;

/// Write to common destinations for output.
pub trait PatternWriter {
  /// Write the given buffer's contents to the given output stream.
  ///
  /// The content of the given buffer will be written to the instance this
  /// method is called on.  As the output is not limited to be a file,
  /// implementations should also consider the possibility to write to `stdout`.
  ///
  /// As errors might occur during IO actions, the returned `sysexits::ExitCode`
  /// indicates whether the operation succeeded.  Implementations should return
  /// values according to the following conventions.  Furthermore, an
  /// appropriate error message can be written to `stderr`, if activated by the
  /// last parameter.
  ///
  /// # Errors
  ///
  /// ## `sysexits::ExitCode::CantCreat`
  ///
  /// In case of an output file to write to, this value indicates that it could
  /// not be created.
  ///
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// Sometimes, a conversion of the data might be required before writing it
  /// to the destination.  This conversion might fail due to the data containing
  /// invalid UTF-8 characters.
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// The data could not be written to the intended destination.  Information
  /// might be lost or were invalid UTF-8 characters which caused the operation
  /// to fail.
  fn write_bytes(&self, buffer: &[u8], show_error_messages: bool) -> ExitCode;

  /// Write the given buffer's contents to the given output stream.
  ///
  /// See [`write_bytes`][PatternWriter::write_bytes] for details.
  fn write_string(&self, buffer: &str, show_error_messages: bool) -> ExitCode;
}

impl PatternWriter for &Option<PathBuf> {
  fn write_bytes(&self, buffer: &[u8], show_error_messages: bool) -> ExitCode {
    self.as_ref().map_or_else(
      || match String::from_utf8(buffer.to_vec()) {
        Ok(string) => {
          print!("{string}");
          ExitCode::Ok
        }
        Err(error) => {
          if show_error_messages {
            eprintln!("{error}");
          }

          ExitCode::DataErr
        }
      },
      |path| match File::create(path) {
        Ok(mut file) => match file.write(buffer) {
          Ok(count) => {
            if count == buffer.len() {
              ExitCode::Ok
            } else {
              if show_error_messages {
                eprintln!("Writing the buffer did not create an exact copy!");
              }

              ExitCode::IoErr
            }
          }
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            ExitCode::IoErr
          }
        },
        Err(error) => {
          if show_error_messages {
            eprintln!("{error}");
          }

          ExitCode::CantCreat
        }
      },
    )
  }

  fn write_string(&self, buffer: &str, show_error_messages: bool) -> ExitCode {
    self.as_ref().map_or_else(
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
                if show_error_messages {
                  eprintln!("Writing the buffer did not create an exact copy!");
                }

                ExitCode::IoErr
              }
            }
            Err(error) => {
              if show_error_messages {
                eprintln!("{error}");
              }

              ExitCode::IoErr
            }
          }
        }
        Err(error) => {
          if show_error_messages {
            eprintln!("{error}");
          }

          ExitCode::CantCreat
        }
      },
    )
  }
}

/******************************************************************************/
