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

use aeruginous::Prefer;

#[test]
fn option_t() {
  assert_eq!(Some(23).prefer(Some(42)), Some(42));
  assert_eq!(None.prefer(Some(42)), Some(42));
  assert_eq!(Some(23).prefer(None), Some(23));
  assert_eq!(None::<i32>.prefer(None), None);
}

#[test]
fn option_string() {
  assert_eq!(
    Some(23.to_string()).prefer(Some(42.to_string())),
    Some(42.to_string())
  );
  assert_eq!(None.prefer(Some(42.to_string())), Some(42.to_string()));
  assert_eq!(Some(23.to_string()).prefer(None), Some(23.to_string()));
  assert_eq!(None::<String>.prefer(None), None);
}

/******************************************************************************/
