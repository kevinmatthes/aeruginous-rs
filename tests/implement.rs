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

use aeruginous::implement;

#[test]
fn one_function() {
  struct Example;

  implement!(Example;
    pub fn function() -> i32 {
      42
    }
  );

  assert_eq!(Example::function(), 42);
}

#[test]
fn two_functions() {
  struct Example;

  implement!(Example;
    pub fn function_one() -> i32 {
      42
    },

    pub fn function_two() -> f64 {
      23.0
    }
  );

  assert_eq!(Example::function_one(), 42);
  assert_eq!(Example::function_two(), 23.0);
}

/******************************************************************************/
