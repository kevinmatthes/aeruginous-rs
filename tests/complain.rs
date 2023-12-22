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

use aeruginous::{Complain, IndentationUnit};
use std::{
    fs::{remove_file, write},
    path::PathBuf,
};

macro_rules! make_test {
    ( @content $( $name:ident : $text:literal -> $n:tt ),+ ) => {
        $(
            #[test]
            fn $name() {
                let file = concat!(stringify!($name), ".txt");
                let ac = Complain::new(vec![PathBuf::from(file)]);

                write(file, $text).unwrap();

                assert!(ac.main().is_err());
                assert_eq!($n, ac.process().unwrap());

                remove_file(file).unwrap();
            }
        )+
    };

    ( @content @tabs $( $name:ident : $text:literal -> $n:tt ),+ ) => {
        $(
            #[test]
            fn $name() {
                let file = concat!(stringify!($name), ".txt");
                let mut ac = Complain::new(vec![PathBuf::from(file)]);

                ac.indent_by(IndentationUnit::Tabs);
                write(file, $text).unwrap();

                assert!(ac.main().is_err());
                assert_eq!($n, ac.process().unwrap());

                remove_file(file).unwrap();
            }
        )+
    };

    ( @path @failure $( $name:ident : $file:literal -> $n:tt ),+ ) => {
        $(
            #[test]
            fn $name() {
                let ac = Complain::new(vec![PathBuf::from($file)]);

                assert!(ac.main().is_err());
                assert_eq!($n, ac.process().unwrap());
            }
        )+
    };

    ( @path @success $( $name:ident : $file:literal ),+ ) => {
        $(
            #[test]
            fn $name() {
                let ac = Complain::new(vec![PathBuf::from($file)]);

                assert!(ac.main().is_ok());
                assert_eq!(0, ac.process().unwrap());
            }
        )+
    };
}

make_test!(@content
    ac_0001_1: "" -> 1,
    ac_0001_2: "abc" -> 1,
    ac_0002_1: "\r\n" -> 1,
    ac_0002_2: "abc\r\n" -> 1,
    ac_0004_1: "abc \n" -> 1,
    ac_0004_2: "abc\t\n" -> 1,
    ac_0004_3: "abc \t\n" -> 1,
    ac_0004_4: "abc\t \n" -> 1,
    ac_0005_1: "\tabc\n" -> 1,
    ac_0006_1: " \t\n" -> 1,
    ac_0006_2: " \tabc\n" -> 1,
    ac_0007_1: "abc\tabc\n" -> 1
);

make_test!(@content @tabs
    ac_0001_3: "" -> 1,
    ac_0001_4: "abc" -> 1,
    ac_0002_3: "\r\n" -> 1,
    ac_0002_4: "abc\r\n" -> 1,
    ac_0004_5: "abc \n" -> 1,
    ac_0004_6: "abc\t\n" -> 1,
    ac_0004_7: "abc \t\n" -> 1,
    ac_0004_8: "abc\t \n" -> 1,
    ac_0005_2: " abc\n" -> 1,
    ac_0006_3: "\t \n" -> 1,
    ac_0006_4: "\t abc\n" -> 1,
    ac_0007_2: "abc\tabc\n" -> 1
);

make_test!(@path @failure
    ac_0003: "graphs/invalid/too_long_comments.agd" -> 2
);

make_test!(@path @success
    ac_success_license_file: "LICENSE"
);

#[test]
fn default() {
    assert_eq!(
        Complain::default().state(),
        (&Vec::new(), [false; 7], IndentationUnit::Spaces, 80)
    );
}

#[test]
fn ignore_all_lints() {
    let mut ac = Complain::new(vec![PathBuf::from(
        "graphs/invalid/too_long_comments.agd",
    )]);

    ac.ignore_carriage_return_line_feeds();
    ac.ignore_line_width_issues();
    ac.ignore_missing_final_line_feed();
    ac.ignore_mixed_indentation();
    ac.ignore_tabs_within_lines();
    ac.ignore_trailing_white_space_characters();
    ac.ignore_wrong_indentation();

    assert!(ac.main().is_ok());
    assert_eq!(
        ac.state(),
        (
            &vec![PathBuf::from("graphs/invalid/too_long_comments.agd")],
            [true; 7],
            IndentationUnit::Spaces,
            80
        )
    );
}

/******************************************************************************/
