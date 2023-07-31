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
    /// - [`sysexits::ExitCode::DataErr`]
    fn try_from_bytes(&mut self, bytes: &[u8]) -> Result<()>;

    /// Fill this buffer with a string.
    ///
    /// This method behaves just like [`Self::try_from_bytes`] for [`String`]
    /// as the respective source type.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn try_from_string(&mut self, string: &str) -> Result<()>;

    /// Convert this buffer into a [`Vec<u8>`].
    ///
    /// Some writing processes require the data to write as a sequence of UTF-8
    /// encoded characters.  If required, this method will take care about the
    /// provision of the data in the required type.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn try_into_bytes(&self) -> Result<Vec<u8>>;

    /// Convert this buffer into a [`String`].
    ///
    /// This method behaves just like [`Self::try_into_bytes`] for [`String`]
    /// as the respective target type.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn try_into_string(&self) -> Result<String>;
}

impl Buffer for String {
    fn try_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        Self::from_utf8(bytes.to_vec()).map_or(
            Err(ExitCode::DataErr),
            |string| {
                *self = string;
                Ok(())
            },
        )
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

/******************************************************************************/
