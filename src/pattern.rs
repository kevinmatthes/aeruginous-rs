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
    match input.read() {
      Ok(boxed_value) => match Box::leak(boxed_value).try_into_string() {
        Ok(lines) => {
          output.write_string(&self(lines), append, show_error_messages)
        }
        Err(code) => code,
      },
      Err(code) => code,
    }
  }
}

/// Read from common sources of input.
pub trait Reader {
  /// Read the input stream(s) and write error messages to `stderr`.
  ///
  /// This method will process the given input streams and return a struct
  /// implementing the [`PatternBuffer`][Buffer] trait.  Errors are indicated
  /// by `sysexits::ExitCode`s to be propagated to the main function.
  ///
  /// # Errors
  ///
  /// ## `sysexits::ExitCode::IoErr`
  ///
  /// Reading from the input stream failed.  For instance, the stream might have
  /// contained invalid UTF-8 characters.
  ///
  /// ## `sysexits::ExitCode::NoInput`
  ///
  /// The given input stream did not exist or the permissions were insufficent.
  /// This is especially in case of files a common error cause.
  fn read(&self) -> Result<Box<dyn Buffer>, ExitCode>;

  /// Read the input stream(s) **without** writing error messages to `stderr`.
  ///
  /// Except the missing error messages, this method behaves just like
  /// [`read`][Reader::read].
  ///
  /// # Errors
  ///
  /// See [`read`][Reader::read].
  fn read_silently(&self) -> Result<Box<dyn Buffer>, ExitCode>;

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
  /// describing the exact cause in further detail.
  ///
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// The conversion to the target type failed.
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
  #[deprecated(note = "Use `read` and `read_silently` instead.")]
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

  /// Fill a string buffer with the information from the given stream.
  ///
  /// This method behaves just like [`read_bytes`][Reader::read_bytes] but
  /// returns a `String` instead of a `Vec<u8>`.
  ///
  /// # Errors
  ///
  /// See [`read_bytes`][Reader::read_bytes].
  #[deprecated(note = "Use `read` and `read_silently` instead.")]
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
  fn read(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    match self {
      Some(path) => match File::open(path) {
        Ok(file) => match BufReader::new(file).fill_buf() {
          Ok(bytes) => Ok(Box::new(bytes.to_vec())),
          Err(error) => {
            eprintln!("{error}");
            Err(ExitCode::IoErr)
          }
        },
        Err(error) => {
          eprintln!("{error}");
          Err(ExitCode::NoInput)
        }
      },
      None => {
        let mut result = String::new();

        for line in stdin().lines() {
          match line {
            Ok(string) => result.push_str(&string),
            Err(error) => {
              eprintln!("{error}");
              return Err(ExitCode::IoErr);
            }
          }
        }

        Ok(Box::new(result))
      }
    }
  }

  fn read_silently(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    match self {
      Some(path) => File::open(path).map_or(Err(ExitCode::NoInput), |file| {
        BufReader::new(file)
          .fill_buf()
          .map_or(Err(ExitCode::IoErr), |bytes| Ok(Box::new(bytes.to_vec())))
      }),
      None => {
        let mut result = String::new();

        for line in stdin().lines() {
          match line {
            Ok(string) => result.push_str(&string),
            Err(_) => return Err(ExitCode::IoErr),
          }
        }

        Ok(Box::new(result))
      }
    }
  }
}

impl Reader for &Vec<PathBuf> {
  fn read(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    if self.is_empty() {
      let mut result = String::new();

      for line in stdin().lines() {
        match line {
          Ok(string) => result.push_str(&string),
          Err(error) => {
            eprintln!("{error}");
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
              eprintln!("{error}");
              return Err(ExitCode::IoErr);
            }
          },
          Err(error) => {
            eprintln!("{error}");
            return Err(ExitCode::NoInput);
          }
        }
      }

      Ok(Box::new(result))
    }
  }

  fn read_silently(&self) -> Result<Box<dyn Buffer>, ExitCode> {
    if self.is_empty() {
      let mut result = String::new();

      for line in stdin().lines() {
        match line {
          Ok(string) => result.push_str(&string),
          Err(_) => return Err(ExitCode::IoErr),
        }
      }

      Ok(Box::new(result))
    } else {
      let mut result = Vec::<u8>::new();

      for file in *self {
        match File::open(file) {
          Ok(file) => match BufReader::new(file).fill_buf() {
            Ok(bytes) => result.append(&mut bytes.to_vec()),
            Err(_) => return Err(ExitCode::IoErr),
          },
          Err(_) => return Err(ExitCode::NoInput),
        }
      }

      Ok(Box::new(result))
    }
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
