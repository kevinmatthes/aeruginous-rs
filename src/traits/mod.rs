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

mod append_as_line;
mod colour_message;
mod convert_buffer;
mod file_formats;
mod prefer;
mod read_file;
mod to_stderr;

pub use append_as_line::AppendAsLine;
pub use colour_message::ColourMessage;
pub use convert_buffer::ConvertBuffer;
pub use file_formats::{
    FromMd, FromRon, FromRst, FromXml, ToMd, ToRon, ToRst, ToXml,
};
pub use prefer::Prefer;
pub use to_stderr::ToStderr;

#[allow(deprecated)]
pub use read_file::ReadFile;

/******************************************************************************/
