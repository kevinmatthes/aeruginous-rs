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

use aeruginous::PatternBuffer;

#[test]
fn string_try_from_bytes() {
  let data = b"bytes";
  let mut string = String::new();

  assert_eq!(string.try_from_bytes(data), Ok(()));
  assert_eq!(string, "bytes".to_string());
}

#[test]
fn string_try_from_string() {
  let data = "string";
  let mut string = String::new();

  assert_eq!(string.try_from_string(data), Ok(()));
  assert_eq!(string, "string".to_string());
}

#[test]
fn string_try_into_bytes() {
  assert_eq!(
    "string".to_string().try_into_bytes(),
    Ok(b"string".to_vec())
  );
}

#[test]
fn string_try_into_string() {
  assert_eq!(
    "string".to_string().try_into_string(),
    Ok("string".to_string())
  );
}

#[test]
fn vecu8_try_from_bytes() {
  let data = b"bytes";
  let mut bytes = Vec::<u8>::new();

  assert_eq!(bytes.try_from_bytes(data), Ok(()));
  assert_eq!(bytes, b"bytes".to_vec());
}

#[test]
fn vecu8_try_from_string() {
  let data = "string";
  let mut bytes = Vec::<u8>::new();

  assert_eq!(bytes.try_from_string(data), Ok(()));
  assert_eq!(bytes, b"string".to_vec());
}

#[test]
fn vecu8_try_into_bytes() {
  assert_eq!(b"bytes".to_vec().try_into_bytes(), Ok(b"bytes".to_vec()));
}

#[test]
fn vecu8_try_into_string() {
  assert_eq!(b"bytes".to_vec().try_into_string(), Ok("bytes".to_string()));
}

/******************************************************************************/
