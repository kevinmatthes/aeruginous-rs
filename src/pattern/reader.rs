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

use crate::PatternBuffer;
use std::{
  fs::File,
  io::{stdin, BufRead, BufReader},
  path::PathBuf,
};
use sysexits::{ExitCode, Result};

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
  /// ## `sysexits::ExitCode::DataErr`
  ///
  /// The buffer could not be converted into the target type.
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
  ) -> Result<Box<dyn PatternBuffer>>;

  /// Read the input stream(s) and write error messages to `stderr`.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  fn read(&self) -> Result<Box<dyn PatternBuffer>> {
    self.behaviour(true)
  }

  /// Read the input stream(s) without writing error messages to `stderr`.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  fn read_silently(&self) -> Result<Box<dyn PatternBuffer>> {
    self.behaviour(false)
  }

  /// Read bytes from this stream.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  #[deprecated(since = "0.2.1")]
  fn read_bytes(&self, show_error_messages: bool) -> Result<Vec<u8>> {
    if show_error_messages {
      self.read()?.as_ref().try_into_bytes()
    } else {
      self.read_silently()?.as_ref().try_into_bytes()
    }
  }

  /// Read a string from this stream.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Reader::behaviour].
  #[deprecated(since = "0.2.1")]
  fn read_string(&self, show_error_messages: bool) -> Result<String> {
    if show_error_messages {
      self.read()?.as_ref().try_into_string()
    } else {
      self.read_silently()?.as_ref().try_into_string()
    }
  }
}

impl Reader for &Option<PathBuf> {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    self.as_ref().map_or_else(
      || stdin().behaviour(show_error_messages),
      |path| Reader::behaviour(path, show_error_messages),
    )
  }
}

impl Reader for PathBuf {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    match File::open(self) {
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
    }
  }
}

impl Reader for std::io::Stdin {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
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

impl Reader for &Vec<PathBuf> {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    if self.is_empty() {
      stdin().behaviour(show_error_messages)
    } else {
      let mut result = Vec::<u8>::new();

      for file in *self {
        match Reader::behaviour(file, show_error_messages) {
          Ok(buffer) => match buffer.as_ref().try_into_bytes() {
            Ok(mut bytes) => result.append(&mut bytes),
            Err(code) => return Err(code),
          },
          Err(code) => return Err(code),
        }
      }

      Ok(Box::new(result))
    }
  }
}

/******************************************************************************/
