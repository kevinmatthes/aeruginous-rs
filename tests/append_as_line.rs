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

use aeruginous::AppendAsLine;

#[test]
fn character_empty_string() {
  let mut buffer = String::new();
  buffer.append_as_line('x');

  assert_eq!("x\n".to_string(), buffer);
}

#[test]
fn character_one_line() {
  let mut buffer = "line\n".to_string();
  buffer.append_as_line('x');

  assert_eq!("line\nx\n".to_string(), buffer);
}

#[test]
fn character_text_without_newline() {
  let mut buffer = "text".to_string();
  buffer.append_as_line('x');

  assert_eq!("textx\n".to_string(), buffer);
}

#[test]
fn str_empty_string() {
  let mut buffer = String::new();
  buffer.append_as_line("test");

  assert_eq!("test\n".to_string(), buffer);
}

#[test]
fn str_one_line() {
  let mut buffer = "line\n".to_string();
  buffer.append_as_line("test");

  assert_eq!("line\ntest\n".to_string(), buffer);
}

#[test]
fn str_text_without_newline() {
  let mut buffer = "text".to_string();
  buffer.append_as_line("test");

  assert_eq!("texttest\n".to_string(), buffer);
}

#[test]
fn string_empty_string() {
  let mut buffer = String::new();
  buffer.append_as_line("test".to_string());

  assert_eq!("test\n".to_string(), buffer);
}

#[test]
fn string_one_line() {
  let mut buffer = "line\n".to_string();
  buffer.append_as_line("test".to_string());

  assert_eq!("line\ntest\n".to_string(), buffer);
}

#[test]
fn string_text_without_newline() {
  let mut buffer = "text".to_string();
  buffer.append_as_line("test".to_string());

  assert_eq!("texttest\n".to_string(), buffer);
}

/******************************************************************************/
