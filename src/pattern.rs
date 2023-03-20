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

/// A buffer for pattern-based IO.
pub trait Buffer {
  /// Fill this buffer with bytes.
  ///
  /// Some reading operations return the read data as a sequence of UTF-8
  /// encoded characters.  This method will convert the given data into the
  /// type of the buffer.  If the conversion of the given data succeeds, the
  /// buffer's contents will be replaced with the converted data.
  ///
  /// # Errors
  ///
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// A conversion was not possible.
  fn try_from_bytes(&mut self, bytes: &[u8]) -> ExitCode;

  /// Fill this buffer with a string.
  ///
  /// This method behaves just like [`try_from_bytes`][Buffer::try_from_bytes]
  /// but for `String` as the respective source type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_from_string(&mut self, string: &str) -> ExitCode;

  /// Convert this buffer into a `Vec<u8>`.
  ///
  /// Some writing processes require the data to write as a sequence of UTF-8
  /// encoded characters.  If required, this method will take care about the
  /// provision of the data in the required type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_into_bytes(&self) -> Result<Vec<u8>, ExitCode>;

  /// Convert this buffer into a `String`.
  ///
  /// This method behaves just like [`try_into_bytes`][Buffer::try_into_bytes]
  /// but for `String` as the respective target type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_into_string(&self) -> Result<String, ExitCode>;
}

impl Buffer for String {
  fn try_from_bytes(&mut self, bytes: &[u8]) -> ExitCode {
    Self::from_utf8(bytes.to_vec()).map_or(ExitCode::DataErr, |string| {
      *self = string;
      ExitCode::Ok
    })
  }

  fn try_from_string(&mut self, string: &str) -> ExitCode {
    *self = string.to_string();
    ExitCode::Ok
  }

  fn try_into_bytes(&self) -> Result<Vec<u8>, ExitCode> {
    Ok(self.as_bytes().to_vec())
  }

  fn try_into_string(&self) -> Result<String, ExitCode> {
    Ok(Self::from(self))
  }
}

impl Buffer for Vec<u8> {
  fn try_from_bytes(&mut self, bytes: &[u8]) -> ExitCode {
    *self = bytes.to_vec();
    ExitCode::Ok
  }

  fn try_from_string(&mut self, string: &str) -> ExitCode {
    *self = string.as_bytes().to_vec();
    ExitCode::Ok
  }

  fn try_into_bytes(&self) -> Result<Vec<u8>, ExitCode> {
    Ok(self.clone())
  }

  fn try_into_string(&self) -> Result<String, ExitCode> {
    String::from_utf8(self.clone()).map_or(Err(ExitCode::DataErr), Ok)
  }
}

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
    input: impl Reader,
    output: impl Writer,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;

  /// Truncate the output stream and write error messages.
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io(&self, input: impl Reader, output: impl Writer) -> ExitCode {
    self.behaviour(input, output, false, true)
  }

  /// Do not truncate the output stream and write error messages.
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_append(&self, input: impl Reader, output: impl Writer) -> ExitCode {
    self.behaviour(input, output, true, true)
  }

  /// Neither truncate the output stream nor write error messages.
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_append_silently(
    &self,
    input: impl Reader,
    output: impl Writer,
  ) -> ExitCode {
    self.behaviour(input, output, true, false)
  }

  /// Truncate the output stream and do not write error messages.
  ///
  /// See [`behaviour`][IOProcessor::behaviour] for details.
  fn io_silent(&self, input: impl Reader, output: impl Writer) -> ExitCode {
    self.behaviour(input, output, false, false)
  }

  /// A deprecated synonym for [`behaviour`][IOProcessor::behaviour].
  #[deprecated(note = "Renamed to `behaviour`.")]
  fn process(
    &self,
    input: impl Reader,
    output: impl Writer,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    self.behaviour(input, output, append, show_error_messages)
  }
}

impl<T: Fn(String) -> String> IOProcessor for T {
  fn behaviour(
    &self,
    input: impl Reader,
    output: impl Writer,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    match input.read() {
      Ok(buffer) => match Box::leak(buffer).try_into_string() {
        Ok(lines) => {
          output.behaviour(Box::new(self(lines)), append, show_error_messages)
        }
        Err(code) => code,
      },
      Err(code) => code,
    }
  }
}

/// Read from common sources of input.
pub trait Reader {
  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.  Implementations should consider the possibilities to read from
  /// both (a) file(s) and `stdin`.
  ///
  /// `show_error_messages` shall control whether to write error messages to
  /// `stderr`, if appropriate.
  ///
  /// # Errors
  ///
  /// Implementations shall follow these conventions for error cases.
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// Reading from the input stream(s) failed.  For instance, the stream(s)
  /// might have contained invalid UTF-8 characters.
  ///
  /// ## `sysexits::ExitCode::NoInput`
  ///
  /// This input stream did not exist or the permissions were insufficent.  This
  /// is especially in case of files a common error cause.
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn Buffer>, ExitCode>;

  /// Read the input stream(s) and write error messages to `stderr`.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  fn read(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    self.behaviour(true)
  }

  /// Read the input stream(s) without writing error messages to `stderr`.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  fn read_silently(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    self.behaviour(false)
  }

  /// Read bytes from this stream.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  #[deprecated]
  fn read_bytes(&self, show_error_messages: bool) -> Result<Vec<u8>, ExitCode> {
    if show_error_messages {
      match self.read() {
        Ok(buffer) => Box::<dyn Buffer>::leak(buffer).try_into_bytes(),
        Err(code) => Err(code),
      }
    } else {
      match self.read_silently() {
        Ok(buffer) => Box::<dyn Buffer>::leak(buffer).try_into_bytes(),
        Err(code) => Err(code),
      }
    }
  }

  /// Read a string from this stream.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  #[deprecated]
  fn read_string(&self, show_error_messages: bool) -> Result<String, ExitCode> {
    if show_error_messages {
      match self.read() {
        Ok(buffer) => Box::<dyn Buffer>::leak(buffer).try_into_string(),
        Err(code) => Err(code),
      }
    } else {
      match self.read_silently() {
        Ok(buffer) => Box::<dyn Buffer>::leak(buffer).try_into_string(),
        Err(code) => Err(code),
      }
    }
  }
}

impl Reader for &Option<PathBuf> {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn Buffer>, ExitCode> {
    match self {
      Some(path) => match File::open(path) {
        Ok(file) => match BufReader::new(file).fill_buf() {
          Ok(bytes) => Ok(Box::new(bytes.to_vec())),
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
      None => {
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

        Ok(Box::new(result))
      }
    }
  }
}

impl Reader for &Vec<PathBuf> {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn Buffer>, ExitCode> {
    if self.is_empty() {
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

      Ok(Box::new(result))
    } else {
      let mut result = Vec::<u8>::new();

      for file in *self {
        match File::open(file) {
          Ok(file) => match BufReader::new(file).fill_buf() {
            Ok(bytes) => result.append(&mut bytes.to_vec()),
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

      Ok(Box::new(result))
    }
  }
}

/// Write to common destinations for output.
pub trait Writer {
  /// Append the buffer's contents to a stream and print error messages.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn append(&self, buffer: Box<dyn Buffer>) -> ExitCode {
    self.behaviour(buffer, true, true)
  }

  /// Append the buffer's contents without printing error messages.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn append_silently(&self, buffer: Box<dyn Buffer>) -> ExitCode {
    self.behaviour(buffer, true, false)
  }

  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.  Implementations should consider the possibilities to write to
  /// both (a) file(s) and `stdout`.
  ///
  /// In case of a file, `append` shall control whether to *not* truncating it
  /// before writing to it.  If the file should not already exist, it shall be
  /// created.  Contents shall be pasted at the file's end.
  ///
  /// `show_error_messages` shall control whether to write error messages to
  /// `stderr`, if appropriate.
  ///
  /// # Errors
  ///
  /// The return value shall indicate whether the operation succeeded.  In case
  /// of success, `sysexits::ExitCode::Ok` shall be returned.  Implementations
  /// shall follow these conventions for error cases.
  ///
  /// ## `sysexits::ExitCode::CantCreat`
  ///
  /// In case of a file, the target could not be created.
  ///
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// The buffer could not be converted into the target type.
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// The data could not be written (completely).  Loss of information is
  /// possible.  The buffer might also have contained invalid UTF-8 characters.
  fn behaviour(
    &self,
    buffer: Box<dyn Buffer>,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode;

  /// Truncate the stream, write the buffer's data, and print error messages.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn write(&self, buffer: Box<dyn Buffer>) -> ExitCode {
    self.behaviour(buffer, false, true)
  }

  /// Truncate the stream and write the buffer's data without error messages.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn write_silently(&self, buffer: Box<dyn Buffer>) -> ExitCode {
    self.behaviour(buffer, false, false)
  }

  /// Write bytes to this stream.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  #[deprecated]
  fn write_bytes(
    &self,
    buffer: &[u8],
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    if show_error_messages {
      if append {
        self.append(Box::new(buffer.to_vec()))
      } else {
        self.write(Box::new(buffer.to_vec()))
      }
    } else if append {
      self.append_silently(Box::new(buffer.to_vec()))
    } else {
      self.write_silently(Box::new(buffer.to_vec()))
    }
  }

  /// Write a string to this stream.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  #[deprecated]
  fn write_string(
    &self,
    buffer: &str,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    if show_error_messages {
      if append {
        self.append(Box::new(buffer.to_string()))
      } else {
        self.write(Box::new(buffer.to_string()))
      }
    } else if append {
      self.append_silently(Box::new(buffer.to_string()))
    } else {
      self.write_silently(Box::new(buffer.to_string()))
    }
  }
}

impl Writer for &Option<PathBuf> {
  fn behaviour(
    &self,
    buffer: Box<dyn Buffer>,
    append: bool,
    show_error_messages: bool,
  ) -> ExitCode {
    match self {
      Some(path) => {
        match File::options()
          .append(append)
          .create(true)
          .write(true)
          .open(path)
        {
          Ok(mut file) => match Box::leak(buffer).try_into_bytes() {
            Ok(bytes) => match file.write(&bytes) {
              Ok(count) => {
                if count == bytes.len() {
                  ExitCode::Ok
                } else {
                  if show_error_messages {
                    eprintln!(
                      "Writing the buffer did not create an exact copy!"
                    );
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
            Err(code) => code,
          },
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            ExitCode::CantCreat
          }
        }
      }
      None => match Box::leak(buffer).try_into_string() {
        Ok(string) => {
          print!("{string}");
          ExitCode::Ok
        }
        Err(code) => code,
      },
    }
  }
}

/******************************************************************************/
