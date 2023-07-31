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

/// Append a buffer as a line to another buffer.
pub trait AppendAsLine<T> {
    /// Append a buffer as a line to another buffer.
    ///
    /// This method will take ownership of a given buffer in order to append it
    /// to the instance this method is called on as a new line.
    fn append_as_line(&mut self, data: T);
}

impl AppendAsLine<char> for String {
    fn append_as_line(&mut self, data: char) {
        self.push(data);
        self.push('\n');
    }
}

impl AppendAsLine<Self> for String {
    fn append_as_line(&mut self, data: Self) {
        self.push_str(data.as_str());
        self.push('\n');
    }
}

impl AppendAsLine<&str> for String {
    fn append_as_line(&mut self, data: &str) {
        self.push_str(data);
        self.push('\n');
    }
}

/******************************************************************************/
