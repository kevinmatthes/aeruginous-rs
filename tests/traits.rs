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

use aeruginous::{
    AppendAsLine, ColourMessage, ConvertBuffer, FromRon, FromXml, Prefer,
    ToRon, ToStderr, ToXml,
};
use anstyle::AnsiColor::Red;
use std::io::{stderr, Error, ErrorKind};
use sysexits::ExitCode;

#[test]
fn append_as_line_character_empty_string() {
    let mut buffer = String::new();
    buffer.append_as_line('x');

    assert_eq!("x\n".to_string(), buffer);
}

#[test]
fn append_as_line_character_one_line() {
    let mut buffer = "line\n".to_string();
    buffer.append_as_line('x');

    assert_eq!("line\nx\n".to_string(), buffer);
}

#[test]
fn append_as_line_character_text_without_newline() {
    let mut buffer = "text".to_string();
    buffer.append_as_line('x');

    assert_eq!("textx\n".to_string(), buffer);
}

#[test]
fn append_as_line_str_empty_string() {
    let mut buffer = String::new();
    buffer.append_as_line("test");

    assert_eq!("test\n".to_string(), buffer);
}

#[test]
fn append_as_line_str_one_line() {
    let mut buffer = "line\n".to_string();
    buffer.append_as_line("test");

    assert_eq!("line\ntest\n".to_string(), buffer);
}

#[test]
fn append_as_line_str_text_without_newline() {
    let mut buffer = "text".to_string();
    buffer.append_as_line("test");

    assert_eq!("texttest\n".to_string(), buffer);
}

#[test]
fn append_as_line_string_empty_string() {
    let mut buffer = String::new();
    buffer.append_as_line("test".to_string());

    assert_eq!("test\n".to_string(), buffer);
}

#[test]
fn append_as_line_string_one_line() {
    let mut buffer = "line\n".to_string();
    buffer.append_as_line("test".to_string());

    assert_eq!("line\ntest\n".to_string(), buffer);
}

#[test]
fn append_as_line_string_text_without_newline() {
    let mut buffer = "text".to_string();
    buffer.append_as_line("test".to_string());

    assert_eq!("texttest\n".to_string(), buffer);
}

#[test]
fn colour_message_string() {
    assert_eq!(
        "Error!".to_string().colour_message(Red, &mut stderr()),
        Ok(())
    );
}

#[test]
fn colour_message_string_slice() {
    assert_eq!("Error!".colour_message(Red, &mut stderr()), Ok(()));
}

#[test]
fn convert_buffer_string_string() {
    let buffer = "buffer".to_string();
    let mut string = "string".to_string();

    assert_eq!(string.convert_into(), Ok("string".to_string()));
    assert_eq!(string.convert_from(buffer), Ok(()));
    assert_eq!(string.convert_into(), Ok("buffer".to_string()));
}

#[test]
fn convert_buffer_string_vecu8() {
    let buffer = b"buffer".to_vec();
    let mut string = "string".to_string();

    assert_eq!(string.convert_into(), Ok(b"string".to_vec()));
    assert_eq!(string.convert_from(buffer), Ok(()));
    assert_eq!(string.convert_into(), Ok(b"buffer".to_vec()));
}

#[test]
fn convert_buffer_vecu8_string() {
    let buffer = "buffer".to_string();
    let mut vecu8 = b"vecu8".to_vec();

    assert_eq!(vecu8.convert_into(), Ok("vecu8".to_string()));
    assert_eq!(vecu8.convert_from(buffer), Ok(()));
    assert_eq!(vecu8.convert_into(), Ok("buffer".to_string()));
}

#[test]
fn convert_buffer_vecu8_vecu8() {
    let buffer = b"buffer".to_vec();
    let mut vecu8 = b"vecu8".to_vec();

    assert_eq!(vecu8.convert_into(), Ok(b"vecu8".to_vec()));
    assert_eq!(vecu8.convert_from(buffer), Ok(()));
    assert_eq!(vecu8.convert_into(), Ok(b"buffer".to_vec()));
}

#[test]
fn extensible_markup_language() {
    #[derive(Debug, Eq, serde::Deserialize, serde::Serialize, PartialEq)]
    struct Number {
        n: i32,
    }

    assert_eq!(
        Number::from_xml(&Number { n: 42 }.to_xml().unwrap()).unwrap(),
        Number { n: 42 }
    );
}

#[test]
fn prefer_option_i32() {
    assert_eq!(Some(23).prefer(Some(42)), Some(42));
    assert_eq!(None.prefer(Some(42)), Some(42));
    assert_eq!(Some(23).prefer(None), Some(23));
    assert_eq!(None::<i32>.prefer(None), None);
}

#[test]
fn prefer_option_string() {
    assert_eq!(
        Some(23.to_string()).prefer(Some(42.to_string())),
        Some(42.to_string())
    );
    assert_eq!(None.prefer(Some(42.to_string())), Some(42.to_string()));
    assert_eq!(Some(23.to_string()).prefer(None), Some(23.to_string()));
    assert_eq!(None::<String>.prefer(None), None);
}

#[test]
fn rusty_object_notation_i32_number_struct() {
    #[derive(Debug, Eq, serde::Deserialize, serde::Serialize, PartialEq)]
    struct Number {
        n: i32,
    }

    assert_eq!(
        Number::from_ron(&Number { n: 42 }.to_ron(2).unwrap()).unwrap(),
        Number { n: 42 }
    );
}

#[test]
fn to_stderr_no() {
    assert_eq!(
        Error::from(ErrorKind::InvalidData).to_stderr(false),
        Err::<(), ExitCode>(ExitCode::DataErr)
    );
}

#[test]
fn to_stderr_yes() {
    assert_eq!(
        Error::from(ErrorKind::InvalidData).to_stderr(true),
        Err::<(), ExitCode>(ExitCode::DataErr)
    );
}

/******************************************************************************/
