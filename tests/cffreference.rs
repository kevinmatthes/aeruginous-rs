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

use aeruginous::Cffreference;
use aeruginous_io::PathBufLikeReader;

macro_rules! make_test {
    ( $( $n:tt -> $name:ident ),+ ) => {
        $(
            #[test]
            fn $name() {
                Cffreference::new(
                    Some(std::path::PathBuf::from(concat!(
                        "tests/assets/CITATION.cff/input_",
                        $n,
                        ".cff"
                    ))),
                    Some(std::path::PathBuf::from(concat!(
                        "tests/assets/CITATION.cff/output_",
                        $n,
                        ".cff"
                    ))),
                )
                .main()
                .unwrap();

                assert_eq!(
                    "tests/assets/CITATION.cff/expectation.yml"
                        .read_silently()
                        .unwrap(),
                    concat!("tests/assets/CITATION.cff/output_", $n, ".cff")
                        .read_silently()
                        .unwrap()
                );

                std::fs::remove_file(concat!(
                    "tests/assets/CITATION.cff/output_",
                    $n,
                    ".cff"
                )).unwrap();
            }
        )+
    };
}

make_test!(
    1 -> classic,
    2 -> data_after_references,
    3 -> has_type,
    4 -> has_preferred_citation,
    5 -> data_after_preferred_citation,
    6 -> has_both_preferred_citation_and_type,
    7 -> has_both_type_and_preferred_citation
);

/******************************************************************************/
