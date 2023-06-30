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

use aeruginous::{Cffreference, ReadFile};
use std::{fs::remove_file, path::PathBuf};

macro_rules! make_test {
  ( $n:tt -> $name:ident ) => {
    #[test]
    fn $name() {
      Cffreference::new(
        Some(PathBuf::from(concat!("./cffs/input_", $n, ".cff"))),
        Some(PathBuf::from(concat!("./cffs/output_", $n, ".cff"))),
      )
      .main()
      .unwrap();

      assert_eq!(
        "./cffs/expectation.yml".read().unwrap(),
        concat!("./cffs/output_", $n, ".cff").read().unwrap()
      );

      remove_file(concat!("./cffs/output_", $n, ".cff")).unwrap();
    }
  };
}

make_test!(1 -> classic);
make_test!(2 -> data_after_references);
make_test!(3 -> has_type);
make_test!(4 -> has_preferred_citation);
make_test!(5 -> data_after_preferred_citation);
make_test!(6 -> has_both_preferred_citation_and_type);
make_test!(7 -> has_both_type_and_preferred_citation);

/******************************************************************************/
