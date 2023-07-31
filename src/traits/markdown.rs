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

/// Create an instance from a Markdown string.
pub trait FromMd: Sized {
    /// Create an instance from valid Markdown.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn from_md(md: &str) -> Result<Self>;
}

/// Convert this instance into a Markdown string.
pub trait ToMd: Sized {
    /// Convert an instance to valid Markdown.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn to_md(&self, header_level: u8) -> Result<String>;
}

/******************************************************************************/
