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
  /// The buffer could not be converted into the target type.
  fn try_from_bytes(&mut self, bytes: &[u8]) -> Result<()>;

  /// Fill this buffer with a string.
  ///
  /// This method behaves just like [`try_from_bytes`][Buffer::try_from_bytes]
  /// but for `String` as the respective source type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_from_string(&mut self, string: &str) -> Result<()>;

  /// Convert this buffer into a `Vec<u8>`.
  ///
  /// Some writing processes require the data to write as a sequence of UTF-8
  /// encoded characters.  If required, this method will take care about the
  /// provision of the data in the required type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_into_bytes(&self) -> Result<Vec<u8>>;

  /// Convert this buffer into a `String`.
  ///
  /// This method behaves just like [`try_into_bytes`][Buffer::try_into_bytes]
  /// but for `String` as the respective target type.
  ///
  /// # Errors
  ///
  /// See [`try_from_bytes`][Buffer::try_from_bytes].
  fn try_into_string(&self) -> Result<String>;
}

impl Buffer for String {
  fn try_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
    Self::from_utf8(bytes.to_vec()).map_or(Err(ExitCode::DataErr), |string| {
      *self = string;
      Ok(())
    })
  }

  fn try_from_string(&mut self, string: &str) -> Result<()> {
    *self = string.to_string();
    Ok(())
  }

  fn try_into_bytes(&self) -> Result<Vec<u8>> {
    Ok(self.as_bytes().to_vec())
  }

  fn try_into_string(&self) -> Result<String> {
    Ok(self.clone())
  }
}

#[cfg(test)]
mod string {
  use crate::PatternBuffer;

  #[test]
  fn try_from_bytes() {
    let data = "bytes".as_bytes();
    let mut string = String::new();

    assert_eq!(string.try_from_bytes(data), Ok(()));
    assert_eq!(string, "bytes".to_string());
  }

  #[test]
  fn try_from_string() {
    let data = "string";
    let mut string = String::new();

    assert_eq!(string.try_from_string(data), Ok(()));
    assert_eq!(string, "string".to_string());
  }

  #[test]
  fn try_into_bytes() {
    assert_eq!(
      "string".to_string().try_into_bytes(),
      Ok("string".as_bytes().to_vec())
    );
  }

  #[test]
  fn try_into_string() {
    assert_eq!(
      "string".to_string().try_into_string(),
      Ok("string".to_string())
    );
  }
}

impl Buffer for Vec<u8> {
  fn try_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
    *self = bytes.to_vec();
    Ok(())
  }

  fn try_from_string(&mut self, string: &str) -> Result<()> {
    *self = string.as_bytes().to_vec();
    Ok(())
  }

  fn try_into_bytes(&self) -> Result<Vec<u8>> {
    Ok(self.clone())
  }

  fn try_into_string(&self) -> Result<String> {
    String::from_utf8(self.clone()).map_or(Err(ExitCode::DataErr), Ok)
  }
}

#[cfg(test)]
mod vec_u8 {
  use crate::PatternBuffer;

  #[test]
  fn try_from_bytes() {
    let data = "bytes".as_bytes();
    let mut bytes = Vec::<u8>::new();

    assert_eq!(bytes.try_from_bytes(data), Ok(()));
    assert_eq!(bytes, "bytes".as_bytes().to_vec());
  }

  #[test]
  fn try_from_string() {
    let data = "string";
    let mut bytes = Vec::<u8>::new();

    assert_eq!(bytes.try_from_string(data), Ok(()));
    assert_eq!(bytes, "string".as_bytes().to_vec());
  }

  #[test]
  fn try_into_bytes() {
    assert_eq!(
      "bytes".as_bytes().to_vec().try_into_bytes(),
      Ok("bytes".as_bytes().to_vec())
    );
  }

  #[test]
  fn try_into_string() {
    assert_eq!(
      "bytes".as_bytes().to_vec().try_into_string(),
      Ok("bytes".to_string())
    );
  }
}

/******************************************************************************/
