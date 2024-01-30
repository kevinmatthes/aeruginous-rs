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

#![allow(deprecated)]

use aeruginous::PatternWriter;

macro_rules! make_test {
  ( @stdio $( $function:ident ),+ ) => {
    $(
      #[test]
      fn $function() {
        assert_eq!(
          std::io::$function().append(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
        assert_eq!(
          std::io::$function().append_silently(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
        assert_eq!(
          std::io::$function().truncate(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
        assert_eq!(
          std::io::$function().truncate_silently(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
        assert_eq!(
          std::io::$function().write(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
        assert_eq!(
          std::io::$function().write_silently(
            Box::new(stringify!($function).to_string())
          ),
          Ok(())
        );
      }
    )+
  };
}

make_test!(@stdio stderr, stdout);

/******************************************************************************/
