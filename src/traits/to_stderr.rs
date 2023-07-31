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

use sysexits::Result;

/// Convert an instance into both a [`sysexits::ExitCode`] and an error message.
pub trait ToStderr<T> {
    /// Convert this instance.
    ///
    /// If the boolean parameter is set to `true`, an appropriate error message
    /// for this instance will be written to [`std::io::Stderr`].  Furthermore,
    /// this instance will be converted into a variant of
    /// [`sysexits::ExitCode`].
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    fn to_stderr(self, message: bool) -> Result<T>;
}

impl<T> ToStderr<T> for std::io::Error {
    fn to_stderr(self, message: bool) -> Result<T> {
        if message {
            eprintln!("{self}");
        }

        Err(self.into())
    }
}

/******************************************************************************/
