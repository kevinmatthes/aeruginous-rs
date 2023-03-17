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

use std::{
  fs::File,
  io::{stdin, BufRead, BufReader, Write},
  path::PathBuf,
};
use sysexits::ExitCode;

/// Read some input, process it, and write it to the intended destination.
pub trait IOProcessor {
  /// Process input in a given manner and write the output to a certain stream.
  ///
  /// Processing input is a common task.  Often, this input originates from a
  /// stream and needs to be written to another stream after processing it.
  /// The traits [`PatternReader`][Reader] and [`PatternWriter`][Writer] offer
  /// semantics to handle reading from and writing to streams.  This method now
  /// adds a convenient connection between them as it is designed to be applied
  /// on input processing functions and closures.
  ///
  /// The result of the operation is indicated by the return value which
  /// originates from the reading and writing processes and can be propagated to
  /// the main function.
  fn process(
    &self,
    input: impl Reader,
    output: impl Writer,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;
}

impl<T: Fn(String) -> String> IOProcessor for T {
  fn process(
    &self,
    input: impl Reader,
    output: impl Writer,
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

/// Read from common sources of input.
pub trait Reader {
  /// Read bytes of information from the given stream.
  ///
  /// The given input stream contains the information to be read as a sequence
  /// of UTF-8 encoded characters, represented as combinations of `u8` bytes.
  /// This method tries to retrieve the contents of a given input stream in
  /// order to return the contained bytes as a vector:  `Vec<u8>`.  As an input
  /// stream is not limited to be a file, implementations should also consider
  /// possible input from `stdin`.
  ///
  /// In case of an error, the Boolean parameter controls whether to write an
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
  /// This method behaves just like [`read_bytes`][Reader::read_bytes] but
  /// returns a `String` instead of a `Vec<u8>`.  In addition, there are further
  /// recommendations for error conditions.
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
  /// See [`read_bytes`][Reader::read_bytes].
  ///
  /// ## `sysexits::ExitCode::NoInput`
  ///
  /// See [`read_bytes`][Reader::read_bytes].
  fn read_string(&self, show_error_messages: bool) -> Result<String, ExitCode>;
}

impl Reader for &Option<PathBuf> {
  fn read_bytes(&self, show_error_messages: bool) -> Result<Vec<u8>, ExitCode> {
    self.as_ref().map_or_else(
      || {
        let mut result = Vec::<u8>::new();

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

        Ok(result)
      },
      |path| match File::open(path) {
        Ok(file) => match BufReader::new(file).fill_buf() {
          Ok(buffer) => Ok(buffer.to_vec()),
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            Err(ExitCode::IoErr)
          }
        },
        Err(error) => {
          if show_error_messages {
            eprintln!("{error}");
          }

          Err(ExitCode::NoInput)
        }
      },
    )
  }

  fn read_string(&self, show_error_messages: bool) -> Result<String, ExitCode> {
    self.as_ref().map_or_else(
      || {
        let mut result = String::new();

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

        Ok(result)
      },
      |path| match File::open(path) {
        Ok(file) => match BufReader::new(file).fill_buf() {
          Ok(buffer) => match String::from_utf8(buffer.to_vec()) {
            Ok(string) => Ok(string),
            Err(error) => {
              if show_error_messages {
                eprintln!("{error}");
              }

              Err(ExitCode::DataErr)
            }
          },
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            Err(ExitCode::IoErr)
          }
        },
        Err(error) => {
          if show_error_messages {
            eprintln!("{error}");
          }

          Err(ExitCode::NoInput)
        }
      },
    )
  }
}

impl Reader for &Vec<PathBuf> {
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

/// Write to common destinations for output.
pub trait Writer {
  /// Write the given buffer's contents to the given output stream.
  ///
  /// The content of the given buffer will be written to the instance this
  /// method is called on.  As the output is not limited to be a file,
  /// implementations should also consider the possibility to write to `stdout`.
  ///
  /// In case of a file to write to, the parameter `append` controls whether to
  /// add the buffer's contents at the end of the file or to truncate the file
  /// before writing to it.
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
  /// might be lost or contained invalid UTF-8 characters which caused the
  /// operation to fail.
  fn write_bytes(
    &self,
    buffer: &[u8],
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;

  /// Write the given buffer's contents to the given output stream.
  ///
  /// See [`write_bytes`][Writer::write_bytes] for details.
  fn write_string(
    &self,
    buffer: &str,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;
}

impl Writer for &Option<PathBuf> {
  fn write_bytes(
    &self,
    buffer: &[u8],
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
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
      |path| match File::options()
        .append(append)
        .create(true)
        .write(true)
        .open(path)
      {
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

  fn write_string(
    &self,
    buffer: &str,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    self.as_ref().map_or_else(
      || {
        print!("{buffer}");
        ExitCode::Ok
      },
      |path| match File::options()
        .append(append)
        .create(true)
        .write(true)
        .open(path)
      {
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
