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
use std::{fs::File, io::Write, path::PathBuf};
use sysexits::{ExitCode, Result};

/// Write to common destinations for output.
pub trait Writer {
  /// Append the buffer's contents to a stream and print error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn append(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, true, true)
  }

  /// Append the buffer's contents without printing error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn append_silently(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, true, false)
  }

  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.  Implementations should consider the possibilities to write to
  /// both (a) file(s) and `stdout`.
  ///
  /// In case of a file, `append` shall control whether to *not* truncate it
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
    buffer: Box<dyn PatternBuffer>,
    append: bool,
    show_error_messages: bool,
  ) -> Result<()>;

  /// Truncate the stream, write the buffer's data, and print error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn write(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, true)
  }

  /// Truncate the stream and write the buffer's data without error messages.
  ///
  /// # Errors
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  fn write_silently(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, false)
  }

  /// Write bytes to this stream.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  #[cfg(not(tarpaulin_include))]
  #[deprecated(since = "0.2.1")]
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
    .into()
  }

  /// Write a string to this stream.
  ///
  /// See [`behaviour`][Writer::behaviour] for details.
  #[cfg(not(tarpaulin_include))]
  #[deprecated(since = "0.2.1")]
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
    .into()
  }
}

impl Writer for &Option<PathBuf> {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    append: bool,
    show_error_messages: bool,
  ) -> Result<()> {
    match self {
      Some(path) => {
        Writer::behaviour(path, buffer, append, show_error_messages)
      }
      None => std::io::stdout().behaviour(buffer, append, show_error_messages),
    }
  }
}

impl Writer for PathBuf {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    append: bool,
    show_error_messages: bool,
  ) -> Result<()> {
    match File::options()
      .append(append)
      .create(true)
      .write(true)
      .open(self)
    {
      Ok(mut file) => match buffer.as_ref().try_into_bytes() {
        Ok(bytes) => match file.write(&bytes) {
          Ok(count) => {
            if count == bytes.len() {
              Ok(())
            } else {
              if show_error_messages {
                eprintln!("Writing the buffer did not create an exact copy!");
              }

              Err(ExitCode::IoErr)
            }
          }
          Err(error) => {
            if show_error_messages {
              eprintln!("{error}");
            }

            Err(ExitCode::IoErr)
          }
        },
        Err(code) => Err(code),
      },
      Err(error) => {
        if show_error_messages {
          eprintln!("{error}");
        }

        Err(ExitCode::CantCreat)
      }
    }
  }
}

impl Writer for std::io::Stdout {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    _: bool,
    _: bool,
  ) -> Result<()> {
    let string = buffer.as_ref().try_into_string()?;
    print!("{string}");
    Ok(())
  }
}

#[cfg(test)]
mod stdout {
  use crate::PatternWriter;
  use std::io::stdout;

  #[test]
  fn append() {
    assert_eq!(stdout().append(Box::new("append".to_string())), Ok(()));
  }

  #[test]
  fn append_silently() {
    assert_eq!(
      stdout().append_silently(Box::new("append_silently".to_string())),
      Ok(())
    );
  }

  #[test]
  fn write() {
    assert_eq!(stdout().write(Box::new("write".to_string())), Ok(()));
  }

  #[test]
  fn write_silently() {
    assert_eq!(
      stdout().write_silently(Box::new("write_silently".to_string())),
      Ok(())
    );
  }
}

/******************************************************************************/
