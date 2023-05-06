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

use sysexits::{ExitCode, Result};

/// A buffer that can be converted from and into another buffer type.
pub trait ConvertBuffer<T> {
  /// Fill this instance with the data of the given buffer, if possible.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use aeruginous::ConvertBuffer;
  ///
  /// let buffer = "buffer".to_string();
  /// let mut string = "string".to_string();
  ///
  /// assert_eq!(string, "string".to_string());
  /// assert_eq!(string.convert_from(buffer), Ok(()));
  /// assert_eq!(string, "buffer".to_string());
  /// ```
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  fn convert_from(&mut self, buffer: T) -> Result<()>;

  /// Convert this buffer into another buffer type for a moment, if possible.
  ///
  /// This method will attempt to convert this instance into the requested type.
  /// In case of success, the resulting data will be returned.  This instance
  /// will remain unchanged in either case.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use aeruginous::ConvertBuffer;
  ///
  /// let string = "string".to_string();
  ///
  /// assert_eq!(string, "string".to_string());
  /// assert_eq!(string.convert_into(), Ok(b"string".to_vec()));
  /// assert_eq!(string, "string".to_string());
  /// ```
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  fn convert_into(&self) -> Result<T>;
}

impl ConvertBuffer<Self> for String {
  fn convert_from(&mut self, buffer: Self) -> Result<()> {
    *self = buffer;
    Ok(())
  }

  fn convert_into(&self) -> Result<Self> {
    Ok(self.clone())
  }
}

impl ConvertBuffer<Vec<u8>> for String {
  fn convert_from(&mut self, buffer: Vec<u8>) -> Result<()> {
    Self::from_utf8(buffer).map_or(Err(ExitCode::DataErr), |s| {
      *self = s;
      Ok(())
    })
  }

  fn convert_into(&self) -> Result<Vec<u8>> {
    Ok(self.as_bytes().to_vec())
  }
}

impl ConvertBuffer<Self> for Vec<u8> {
  fn convert_from(&mut self, buffer: Self) -> Result<()> {
    *self = buffer;
    Ok(())
  }

  fn convert_into(&self) -> Result<Self> {
    Ok(self.clone())
  }
}

impl ConvertBuffer<String> for Vec<u8> {
  fn convert_from(&mut self, buffer: String) -> Result<()> {
    *self = buffer.as_bytes().to_vec();
    Ok(())
  }

  fn convert_into(&self) -> Result<String> {
    String::from_utf8(self.clone()).map_or(Err(ExitCode::DataErr), Ok)
  }
}

/******************************************************************************/
