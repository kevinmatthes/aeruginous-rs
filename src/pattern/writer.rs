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
  /// See [`Self::behaviour`].
  fn append(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, true, true, false)
  }

  /// Append the buffer's contents without printing error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn append_silently(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, true, false, false)
  }

  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.  Implementations should also consider the possibilities to write
  /// to both (a) file(s) and [`std::io::Stdout`].
  ///
  /// In case of a file,
  ///
  /// - `append` shall control whether to edit it solely by pasting the buffer's
  ///   contents at the file's end.
  /// - `truncate` shall control whether to clear the file before writing to it.
  /// - the output file shall be created, in case that it should not already
  ///   exist.
  ///
  /// `show_error_messages` shall control whether to write error messages to
  /// [`std::io::Stderr`], if appropriate.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::CantCreat`]
  /// - [`sysexits::ExitCode::DataErr`]
  /// - [`sysexits::ExitCode::IoErr`]
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    append: bool,
    show_error_messages: bool,
    truncate: bool,
  ) -> Result<()>;

  /// Truncate the stream, write the buffer's data, and print error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn truncate(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, true, true)
  }

  /// Truncate the stream and write the buffer's data without error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn truncate_silently(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, false, true)
  }

  /// Edit the stream, write the buffer's data, and print error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn write(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, true, false)
  }

  /// Edit the stream and write the buffer's data without error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn write_silently(&self, buffer: Box<dyn PatternBuffer>) -> Result<()> {
    self.behaviour(buffer, false, false, false)
  }

  /// Write bytes to this stream.
  ///
  /// See [`Self::behaviour`].
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
  /// See [`Self::behaviour`].
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
    truncate: bool,
  ) -> Result<()> {
    match self {
      Some(path) => {
        Writer::behaviour(path, buffer, append, show_error_messages, truncate)
      }
      None => std::io::stdout().behaviour(
        buffer,
        append,
        show_error_messages,
        truncate,
      ),
    }
  }
}

impl Writer for PathBuf {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    append: bool,
    show_error_messages: bool,
    truncate: bool,
  ) -> Result<()> {
    match File::options()
      .append(append)
      .create(true)
      .write(true)
      .truncate(truncate)
      .open(self)
    {
      Ok(mut file) => {
        let bytes = buffer.as_ref().try_into_bytes()?;

        match file.write(&bytes) {
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
        }
      }
      Err(error) => {
        if show_error_messages {
          eprintln!("{error}");
        }

        Err(ExitCode::CantCreat)
      }
    }
  }
}

impl Writer for std::io::Stderr {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    _: bool,
    _: bool,
    _: bool,
  ) -> Result<()> {
    let string = buffer.as_ref().try_into_string()?;
    eprint!("{string}");
    Ok(())
  }
}

#[cfg(test)]
mod stderr {
  use crate::PatternWriter;
  use std::io::stderr;

  #[test]
  fn append() {
    assert_eq!(stderr().append(Box::new("append".to_string())), Ok(()));
  }

  #[test]
  fn append_silently() {
    assert_eq!(
      stderr().append_silently(Box::new("append_silently".to_string())),
      Ok(())
    );
  }

  #[test]
  fn write() {
    assert_eq!(stderr().write(Box::new("write".to_string())), Ok(()));
  }

  #[test]
  fn write_silently() {
    assert_eq!(
      stderr().write_silently(Box::new("write_silently".to_string())),
      Ok(())
    );
  }

  #[test]
  fn truncate() {
    assert_eq!(stderr().truncate(Box::new("truncate".to_string())), Ok(()));
  }

  #[test]
  fn truncate_silently() {
    assert_eq!(
      stderr().truncate_silently(Box::new("truncate_silently".to_string())),
      Ok(())
    );
  }
}

impl Writer for std::io::Stdout {
  fn behaviour(
    &self,
    buffer: Box<dyn PatternBuffer>,
    _: bool,
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

  #[test]
  fn truncate() {
    assert_eq!(stdout().truncate(Box::new("truncate".to_string())), Ok(()));
  }

  #[test]
  fn truncate_silently() {
    assert_eq!(
      stdout().truncate_silently(Box::new("truncate_silently".to_string())),
      Ok(())
    );
  }
}

/******************************************************************************/
