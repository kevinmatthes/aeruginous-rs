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

use serde::{Deserialize, Serialize};
use sysexits::{ExitCode, Result};

/// Create an instance from a Markdown string.
pub trait FromMd: Sized {
    /// Create an instance from valid Markdown.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn from_md(md: &str) -> Result<Self>;
}

/// Create an instance from a RON string.
pub trait FromRon<'a>: Deserialize<'a> {
    /// Create an instance implementing [`serde::Deserialize`] from valid RON.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn from_ron(ron: &'a str) -> Result<Self>;
}

impl<'a, T: Deserialize<'a>> FromRon<'a> for T {
    fn from_ron(ron: &'a str) -> Result<Self> {
        ron::de::from_str(ron).map_or(Err(ExitCode::DataErr), Ok)
    }
}

/// Create an instance from a reStructured Text string.
pub trait FromRst: Sized {
    /// Create an instance from valid reStructured Text.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn from_rst(rst: &str) -> Result<Self>;
}

/// Create an instance from an eXtensible Markup Language string.
pub trait FromXml<'a>: Deserialize<'a> {
    /// Create an instance from valid eXtensible Markup Language.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn from_xml(xml: &'a str) -> Result<Self>;
}

impl<'a, T: Deserialize<'a>> FromXml<'a> for T {
    fn from_xml(xml: &'a str) -> Result<Self> {
        quick_xml::de::from_str(xml).map_or(Err(ExitCode::DataErr), Ok)
    }
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

/// Convert this instance into a RON string.
pub trait ToRon: Serialize {
    /// Convert an instance implementing [`serde::Serialize`] to valid RON.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn to_ron(&self, indentation_width: usize) -> Result<String>;
}

impl<T: Serialize> ToRon for T {
    fn to_ron(&self, indentation_width: usize) -> Result<String> {
        ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::default()
                .indentor(" ".repeat(indentation_width)),
        )
        .map_or(Err(ExitCode::DataErr), |mut s| {
            s.push('\n');
            Ok(s)
        })
    }
}

/// Convert this instance into a reStructured Text string.
pub trait ToRst: Sized {
    /// Convert an instance to valid reStructured Text.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn to_rst(&self, header_level: u8) -> Result<String>;
}

/// Convert this instance into an eXtensible Markup Language string.
pub trait ToXml: Serialize {
    /// Convert an instance to valid eXtensible Markup Language.
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::DataErr`]
    fn to_xml(&self) -> Result<String>;
}

impl<T: Serialize> ToXml for T {
    fn to_xml(&self) -> Result<String> {
        quick_xml::se::to_string(&self).map_or(Err(ExitCode::DataErr), |s| {
            let mut result = String::from("<?xml version=\"1.0\"?>\n");
            result.push_str(&s);
            result.push('\n');
            Ok(result)
        })
    }
}

/******************************************************************************/
