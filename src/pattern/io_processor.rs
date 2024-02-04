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

#[allow(deprecated)]
use crate::{PatternWriter, ReadFile};
use sysexits::Result;

/// Read some input, process it, and write it to the intended destination.
///
/// Many CLIs need to interact with streams, such as files on a file system and
/// [`std::io::Stdout`] as well as [`std::io::Stdin`].  The tasks are usually to
/// read from one or more input stream(s) and to write the processed data to
/// another stream.  Often, only the processing logic differs but the actual IO
/// steps to execute are always the same.  This leads to redundant boilerplate
/// code which is hard to maintain.
///
/// This trait shall provide a uniform and convenient interface for processing
/// logic such that the user can focus on the implementation of the way the read
/// data shall be processed.  The IO is handled by [`ReadFile`]s and
/// [`PatternWriter`]s such that one can rely on their semantics.
pub trait IOProcessor {
    /// The shared logic of all methods.
    ///
    /// This method defines the common behaviour of all methods this trait
    /// provides.
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
    /// The return value shall indicate whether the operation succeeded.
    /// Implementations should rely on the semantics of [`ReadFile`] and
    /// [`PatternWriter`] instead of introducing further exit codes.
    #[allow(deprecated)]
    fn behaviour(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
    ) -> Result<()>;

    /// Truncate the output stream and write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    #[deprecated(since = "3.7.7", note = "use `aeruginous_io` instead")]
    fn io(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, false, true, true)
    }

    /// Do not truncate the output stream but write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    fn io_append(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, true, true, false)
    }

    /// Neither truncate the output stream nor write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    fn io_append_silently(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, true, false, false)
    }

    /// Truncate the output stream but do not write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    #[deprecated(since = "3.7.7", note = "use `aeruginous_io` instead")]
    fn io_silent(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, false, false, true)
    }

    /// Edit the output stream and write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    fn io_write(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, false, true, false)
    }

    /// Edit the output stream but do not write error messages.
    ///
    /// # Errors
    ///
    /// See [`Self::behaviour`].
    #[allow(deprecated)]
    fn io_write_silently(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
    ) -> Result<()> {
        self.behaviour(input, output, false, false, false)
    }
}

#[allow(deprecated)]
impl<T: Fn(String) -> String> IOProcessor for T {
    fn behaviour(
        &self,
        input: impl ReadFile,
        output: impl PatternWriter,
        append: bool,
        show_error_messages: bool,
        truncate: bool,
    ) -> Result<()> {
        output.behaviour(
            Box::new(self(input.read()?)),
            append,
            show_error_messages,
            truncate,
        )
    }
}

/******************************************************************************/
