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

use crate::{PatternBuffer, ToStderr};
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
    /// provides.  Implementations should also consider the possibilities to
    /// write to both (a) file(s) and [`std::io::Stdout`].
    ///
    /// In case of a file,
    ///
    /// - `append` shall control whether to edit it solely by pasting the
    ///   buffer's contents at the file's end.
    /// - `truncate` shall control whether to clear the file before writing to
    ///   it.
    /// - the output file shall be created, in case that it should not already
    ///   exist.
    ///
    /// `show_error_messages` shall control whether to write error messages to
    /// [`std::io::Stderr`], if appropriate.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
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
}

/// Add an implementation of the trait for a certain type.
#[macro_export]
macro_rules! impl_pattern_writer_for {
  ( @all $T:ty ) => {
    impl_pattern_writer_for!(@option $T);
    impl_pattern_writer_for!(@ref $T);
  };
  ( @option $T:ty ) => {
    impl $crate::PatternWriter for $T {
      fn behaviour(
        &self,
        buffer: Box<dyn PatternBuffer>,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
      ) -> Result<()> {
        match self {
          Some(thing) => Writer::behaviour(
            thing,
            buffer,
            append,
            show_error_messages,
            truncate,
          ),
          None => std::io::stdout().behaviour(
            buffer,
            append,
            show_error_messages,
            truncate,
          ),
        }
      }
    }
  };
  ( @ref $T:ty ) => {
    impl $crate::PatternWriter for &$T {
      fn behaviour(
        &self,
        buffer: Box<dyn PatternBuffer>,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
      ) -> Result<()> {
        (*self).behaviour(buffer, append, show_error_messages, truncate)
      }
    }
  };
}

impl_pattern_writer_for!(@all Option<PathBuf>);
impl_pattern_writer_for!(@option Option<&PathBuf>);
impl_pattern_writer_for!(@all Option<String>);
impl_pattern_writer_for!(@option Option<&str>);
impl_pattern_writer_for!(@ref PathBuf);

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
                                eprintln!(
                                    "Creating an exact copy was not possible."
                                );
                            }

                            Err(ExitCode::IoErr)
                        }
                    }
                    Err(error) => error.to_stderr(show_error_messages),
                }
            }
            Err(error) => error.to_stderr(show_error_messages),
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
        eprint!("{}", buffer.as_ref().try_into_string()?);
        Ok(())
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
        print!("{}", buffer.as_ref().try_into_string()?);
        Ok(())
    }
}

impl Writer for String {
    fn behaviour(
        &self,
        buffer: Box<dyn PatternBuffer>,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
    ) -> Result<()> {
        PathBuf::from(&self).behaviour(
            buffer,
            append,
            show_error_messages,
            truncate,
        )
    }
}

impl Writer for &str {
    fn behaviour(
        &self,
        buffer: Box<dyn PatternBuffer>,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
    ) -> Result<()> {
        PathBuf::from(&self).behaviour(
            buffer,
            append,
            show_error_messages,
            truncate,
        )
    }
}

/******************************************************************************/
