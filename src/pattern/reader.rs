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
use std::{io::stdin, path::PathBuf};
use sysexits::Result;

/// Read from common sources of input.
pub trait Reader {
  /// The shared logic of all methods.
  ///
  /// This method defines the common behaviour of all methods this trait
  /// provides.  Implementations should also consider the possibilities to read
  /// from both (a) file(s) and [`std::io::Stdin`].
  ///
  /// `show_error_messages` shall control whether to write error messages to
  /// [`std::io::Stderr`], if appropriate.
  ///
  /// # Errors
  ///
  /// See [`sysexits::ExitCode`].
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>>;

  /// Read the input stream(s) and write error messages to [`std::io::Stderr`].
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn read(&self) -> Result<Box<dyn PatternBuffer>> {
    self.behaviour(true)
  }

  /// Read the input stream(s) without writing error messages.
  ///
  /// # Errors
  ///
  /// See [`Self::behaviour`].
  fn read_silently(&self) -> Result<Box<dyn PatternBuffer>> {
    self.behaviour(false)
  }
}

impl Reader for Option<PathBuf> {
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

impl Reader for &Option<PathBuf> {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    (*self).behaviour(show_error_messages)
  }
}

impl Reader for PathBuf {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    match std::fs::read_to_string(self) {
      Ok(string) => Ok(Box::new(string)),
      Err(error) => error.to_stderr(show_error_messages),
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
        Err(error) => return error.to_stderr(show_error_messages),
      }
    }

    Ok(Box::new(result))
  }
}

impl Reader for &str {
  fn behaviour(
    &self,
    show_error_messages: bool,
  ) -> Result<Box<dyn PatternBuffer>> {
    PathBuf::from(self).behaviour(show_error_messages)
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
      let mut result = Vec::new();

      for file in *self {
        result.append(
          &mut Reader::behaviour(file, show_error_messages)?
            .as_ref()
            .try_into_bytes()?,
        );
      }

      Ok(Box::new(result))
    }
  }
}

/******************************************************************************/
