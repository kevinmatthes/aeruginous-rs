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

use anstyle::{AnsiColor, Style};
use std::io::Write;
use sysexits::Result;

/// Write an instance to a stream using [`AnsiColor`]s for colouring.
pub trait ColourMessage {
    /// Write this instance to the given stream using an [`AnsiColor`].
    ///
    /// The instance this method is called on will be written to the given
    /// stream.  Before the instance is written, it is attempted to set the
    /// output colour to the given [`AnsiColor`].  After the instance was
    /// written, the colour will be reset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aeruginous::ColourMessage;
    /// use anstyle::AnsiColor::Red;
    /// use std::io::stderr;
    ///
    /// assert_eq!("Error!".colour_message(Red, &mut stderr()), Ok(()));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`sysexits::ExitCode::IoErr`]
    fn colour_message(
        &self,
        colour: AnsiColor,
        stream: &mut dyn Write,
    ) -> Result<()>;
}

impl ColourMessage for str {
    fn colour_message(
        &self,
        colour: AnsiColor,
        stream: &mut dyn Write,
    ) -> Result<()> {
        let colour = Style::new().fg_color(Some(colour.into()));

        colour.write_to(stream)?;
        write!(stream, "{self}")?;
        colour.write_reset_to(stream)?;

        Ok(())
    }
}

impl ColourMessage for String {
    fn colour_message(
        &self,
        colour: AnsiColor,
        stream: &mut dyn Write,
    ) -> Result<()> {
        self.as_str().colour_message(colour, stream)
    }
}

/******************************************************************************/
