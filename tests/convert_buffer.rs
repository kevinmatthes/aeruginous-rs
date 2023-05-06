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

use aeruginous::ConvertBuffer;

#[test]
fn string_string() {
  let buffer = "buffer".to_string();
  let mut string = "string".to_string();

  assert_eq!(string.convert_into(), Ok("string".to_string()));
  assert_eq!(string.convert_from(buffer), Ok(()));
  assert_eq!(string.convert_into(), Ok("buffer".to_string()));
}

#[test]
fn string_vecu8() {
  let buffer = b"buffer".to_vec();
  let mut string = "string".to_string();

  assert_eq!(string.convert_into(), Ok(b"string".to_vec()));
  assert_eq!(string.convert_from(buffer), Ok(()));
  assert_eq!(string.convert_into(), Ok(b"buffer".to_vec()));
}

#[test]
fn vecu8_string() {
  let buffer = "buffer".to_string();
  let mut vecu8 = b"vecu8".to_vec();

  assert_eq!(vecu8.convert_into(), Ok("vecu8".to_string()));
  assert_eq!(vecu8.convert_from(buffer), Ok(()));
  assert_eq!(vecu8.convert_into(), Ok("buffer".to_string()));
}

#[test]
fn vecu8_vecu8() {
  let buffer = b"buffer".to_vec();
  let mut vecu8 = b"vecu8".to_vec();

  assert_eq!(vecu8.convert_into(), Ok(b"vecu8".to_vec()));
  assert_eq!(vecu8.convert_from(buffer), Ok(()));
  assert_eq!(vecu8.convert_into(), Ok(b"buffer".to_vec()));
}

/******************************************************************************/
