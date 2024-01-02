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

macro_rules! make_test {
    ( @rs2md @none $( $n:ident -> $i:expr , $o:expr ),+ ) => {
        $(
            #[cfg(feature = "rs2md")]
            #[test]
            fn $n() {
                use aeruginous::ReadFile;

                assert!(aeruginous::Rs2md::new(
                    vec![
                        "tests/assets/gpl-3.0-inner.rs",
                        "tests/assets/gpl-3.0-outer.rs"
                    ],
                    Some(concat!(stringify!($n), ".md")),
                    $i,
                    $o
                ).main().is_ok());

                assert!(
                    concat!(stringify!($n), ".md")
                        .read()
                        .unwrap()
                        .is_empty()
                );

                std::fs::remove_file(concat!(stringify!($n), ".md")).unwrap();
            }
        )+
    };

    ( @rs2md @once $( $n:ident -> $i:expr , $o:expr ),+ ) => {
        $(
            #[cfg(feature = "rs2md")]
            #[test]
            fn $n() {
                use aeruginous::ReadFile;

                assert!(aeruginous::Rs2md::new(
                    vec![
                        "tests/assets/gpl-3.0-inner.rs",
                        "tests/assets/gpl-3.0-outer.rs"
                    ],
                    Some(concat!(stringify!($n), ".md")),
                    $i,
                    $o
                ).main().is_ok());

                assert_eq!(
                    concat!(stringify!($n), ".md").read().unwrap(),
                    "\
Copyright (C) 2023 Kevin Matthes

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"
                );

                std::fs::remove_file(concat!(stringify!($n), ".md")).unwrap();
            }
        )+
    };

    ( @rs2md @twice $( $n:ident -> $i:expr , $o:expr ),+ ) => {
        $(
            #[cfg(feature = "rs2md")]
            #[test]
            fn $n() {
                use aeruginous::ReadFile;

                assert!(aeruginous::Rs2md::new(
                    vec![
                        "tests/assets/gpl-3.0-inner.rs",
                        "tests/assets/gpl-3.0-outer.rs"
                    ],
                    Some(concat!(stringify!($n), ".md")),
                    $i,
                    $o
                ).main().is_ok());

                assert_eq!(
                    concat!(stringify!($n), ".md").read().unwrap(),
                    "\
Copyright (C) 2023 Kevin Matthes

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
Copyright (C) 2023 Kevin Matthes

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"
                );

                std::fs::remove_file(concat!(stringify!($n), ".md")).unwrap();
            }
        )+
    };
}

make_test!(@rs2md @none
    rs2md_neither_inner_nor_outer -> false, false
);

make_test!(@rs2md @once
    rs2md_only_inner -> true, false,
    rs2md_only_outer -> false, true
);

make_test!(@rs2md @twice
    rs2md_both_inner_and_outer -> true, true
);

#[cfg(feature = "mkcws")]
#[test]
fn mkcws() {
    use aeruginous::ReadFile;

    assert!(aeruginous::Mkcws::new(".", Some("cwd.code-workspace"))
        .main()
        .is_ok());

    assert_eq!(
        "cwd.code-workspace".read().unwrap(),
        "{ \"folders\" : [ { \"path\" : \".\" } ] }\n"
    );

    std::fs::remove_file("cwd.code-workspace").unwrap();
}

#[cfg(feature = "uncrlf")]
#[test]
fn uncrlf() {
    use aeruginous::{PatternWriter, ReadFile};

    "uncrlf.txt"
        .truncate(Box::new("TEST\r\nTest\ntest\r\n".to_string()))
        .unwrap();

    assert!(aeruginous::Uncrlf::new(None, None, Some("uncrlf.txt"))
        .main()
        .is_ok());

    assert_eq!("uncrlf.txt".read().unwrap(), "TEST\nTest\ntest\n");
    std::fs::remove_file("uncrlf.txt").unwrap();
}

/******************************************************************************/
