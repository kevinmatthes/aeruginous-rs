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
    ( @copy $name:ident : $( $file:literal ),+ ) => {
        $(
            std::fs::copy($file, make_test! { @dir $name, $file }).unwrap();
        )+
    };

    ( @dir $name:ident ) => {
        concat!("tests/", stringify!($name), "/")
    };
    ( @dir $name:ident , $file:literal ) => {
        concat!(make_test! { @dir $name }, $file)
    };

    ( @success $( $name:ident : $level:tt ),+ ) => {
        $(
            #[test]
            fn $name() {
                std::fs::create_dir(make_test! { @dir $name }).unwrap();

                make_test!(
                    @copy $name:
                        ".version",
                        "Cargo.lock",
                        "Cargo.toml",
                        "CITATION.cff"
                );

                assert!(aeruginous::IncrementVersion::new(
                    vec![
                        make_test! { @dir $name, ".version" }.into(),
                        make_test! { @dir $name, "Cargo.lock" }.into(),
                        make_test! { @dir $name, "Cargo.toml" }.into(),
                        make_test! { @dir $name, "CITATION.cff" }.into()
                    ],
                    vec![make_test! { @dir $name, "Cargo.toml" }.into()],
                    aeruginous::VERSION.to_string(),
                    Some("aeruginous".to_string()),
                    aeruginous::VersionRange::$level
                )
                .main()
                .is_ok());

                std::fs::remove_dir_all(make_test! { @dir $name }).unwrap();
            }
        )+
    };
}

make_test!(@success success_major: Major);
make_test!(@success success_minor: Minor);
make_test!(@success success_patch: Patch);

/******************************************************************************/
