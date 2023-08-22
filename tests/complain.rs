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

use aeruginous::Complain;

macro_rules! make_test {
    ( $( $name:ident : $text:literal -> $n:tt ),+ ) => {
        $(
            #[test]
            fn $name() {
                let file = concat!(stringify!($name), ".txt");
                let aercom =
                    Complain::new(vec![std::path::PathBuf::from(file)]);

                std::fs::write(file, $text).unwrap();

                assert!(aercom.main().is_err());
                assert_eq!($n, aercom.process().unwrap());

                std::fs::remove_file(file).unwrap();
            }
        )+
    };
}

make_test!(
    aercom_0001_1: "" -> 1,
    aercom_0001_2: "abc" -> 1,
    aercom_0002_1: "\r\n" -> 1,
    aercom_0002_2: "abc\r\n" -> 1,
    aercom_0004_1: "abc \n" -> 1,
    aercom_0004_2: "abc\t\n" -> 1,
    aercom_0004_3: "abc \t\n" -> 1,
    aercom_0004_4: "abc\t \n" -> 1,
    aercom_0005: "\tabc\n" -> 1,
    aercom_0006_1: " \t\n" -> 1,
    aercom_0006_2: " \tabc\n" -> 1,
    aercom_0007: "abc\tabc\n" -> 1
);

/******************************************************************************/
