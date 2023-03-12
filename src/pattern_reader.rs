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

//! Convenient implementations of common IO reading patterns.

use std::{
  fs::File,
  io::{stdin, BufRead, BufReader},
  path::PathBuf,
};
use sysexits::ExitCode;

/// Read from common sources of input.
pub trait PatternReader {
  /// Read bytes of information from the given stream.
  ///
  /// The given input stream contains the information to be read as a sequence
  /// of UTF-8 encoded characters, represented as combinations of `u8` bytes.
  /// This method tries to retrieve the contents of a given input stream to
  /// return the contained bytes as a vector:  `Vec<u8>`.  As an input stream
  /// is not limited to be a file, implementations should also consider possible
  /// input from `stdin`.
  ///
  /// In case of an error, the second parameter controls whether to write an
  /// appropriate error message to `stderr`.
  ///
  /// # Errors
  ///
  /// As with all IO actions, errors may occur.  In order to handle them
  /// appropriately, this method will return a `sysexits::ExitCode` then,
  /// describing the exact cause in further detail.  Implementations of this
  /// trait should use the following conventions.
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// An error occured while reading from the stream.  This might have happened
  /// due to invalid UTF-8 characters, for example.
  ///
  /// ## `sysexits::ExitCode::NoInput`
  ///
  /// Reading from the stream was not possible.  For example, the resource did
  /// not exist or the permissions were insufficient.
  fn read_bytes(&self, show_error_messages: bool) -> Result<Vec<u8>, ExitCode>;

  /// Fill a string buffer with the information from the given stream.
  ///
  /// This method behaves just like `read_bytes` but returns a `String` instead
  /// of a `Vec<u8>`.  In addition, there are further recommendations for error
  /// conditions.
  ///
  /// # Errors
  ///
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// The conversion to an UTF-8 encoded string failed.  One reason might be
  /// that the read bytes describe invalid UTF-8 characters.
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// See [`read_bytes`][PatternReader::read_bytes].
  ///
  /// ## `sysexits::ExitCode::NoInput`
  ///
  /// See [`read_bytes`][PatternReader::read_bytes].
  fn read_string(&self, show_error_messages: bool) -> Result<String, ExitCode>;
}

impl PatternReader for &Vec<PathBuf> {
  /// Read from multiple input files or `stdin`, if there is not any file.
  fn read_bytes(&self, show_error_messages: bool) -> Result<Vec<u8>, ExitCode> {
    let mut result = Vec::<u8>::new();

    if self.is_empty() {
      for line in stdin().lines() {
        match line {
          Ok(string) => result.append(&mut string.as_bytes().to_vec()),
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            return Err(ExitCode::IoErr);
          }
        }
      }
    } else {
      for file in *self {
        match File::open(file) {
          Ok(file) => match BufReader::new(file).fill_buf() {
            Ok(buffer) => result.append(&mut buffer.to_vec()),
            Err(error) => {
              if show_error_messages {
                eprintln!("{error}");
              }

              return Err(ExitCode::IoErr);
            }
          },
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            return Err(ExitCode::NoInput);
          }
        }
      }
    }

    Ok(result)
  }

  /// Read from multiple input files or `stdin`, if there is not any file.
  fn read_string(&self, show_error_messages: bool) -> Result<String, ExitCode> {
    let mut result = String::new();

    if self.is_empty() {
      for line in stdin().lines() {
        match line {
          Ok(string) => result.push_str(&string),
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            return Err(ExitCode::IoErr);
          }
        }
      }
    } else {
      for file in *self {
        match File::open(file) {
          Ok(file) => match BufReader::new(file).fill_buf() {
            Ok(buffer) => match String::from_utf8(buffer.to_vec()) {
              Ok(string) => result.push_str(&string),
              Err(error) => {
                if show_error_messages {
                  eprintln!("{error}");
                }

                return Err(ExitCode::DataErr);
              }
            },
            Err(error) => {
              if show_error_messages {
                eprintln!("{error}");
              }

              return Err(ExitCode::IoErr);
            }
          },
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            return Err(ExitCode::NoInput);
          }
        }
      }
    }

    Ok(result)
  }
}

/******************************************************************************/
