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

use aeruginous::{
  AgdTokens::{Comment, FullStop, LineFeed, Space, Unexpected},
  GraphDescription, PatternReader,
};
use std::path::PathBuf;
use sysexits::ExitCode;

macro_rules! make_test {
  ( @line $( $name:ident : $path:literal -> $lines:tt ),+ ) => {
    $(
      #[test]
      fn $name() {
        let agd = GraphDescription::new();
        let input = PathBuf::from($path)
          .read()
          .unwrap()
          .try_into_string()
          .unwrap();

        assert_eq!(agd.line_width(&input), Ok($lines));
      }
    )+
  };

  ( @main @data-err $( $name:ident : $path:literal ),+ ) => {
    $(
      #[test]
      fn $name() {
        assert_eq!(
          GraphDescription::main(&Some(PathBuf::from($path))),
          Err(ExitCode::DataErr)
        );
      }
    )+
  };

  ( @read @fail $( $name:ident : $string:literal ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = GraphDescription::new();
        assert_eq!(agd.read($string), Err(sysexits::ExitCode::DataErr));
      }
    )+
  };

  (
    @read @tokens $(
      $name:ident : $string:literal -> $expectation:tt
    ),+
  ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = GraphDescription::new();
        agd.read($string).unwrap();

        assert_eq!(agd.tokens(), &$expectation);
      }
    )+
  };

  ( @read @tokens @comment $( $name:ident : $string:literal ),+ ) => {
    make_test!(
      @read @tokens $(
        $name : $string -> [Comment]
      ),+
    );
  };

  ( @typos $($name:ident : $path:literal -> $typos:tt ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = GraphDescription::new();
        let input = PathBuf::from($path)
          .read()
          .unwrap()
          .try_into_string()
          .unwrap();
        agd.read(&input).unwrap();

        assert_eq!(agd.typos(), Ok($typos));
      }
    )+
  };
}

make_test!(@line
  line_comment: "./graphs/testing/comment.agd" -> 0,
  line_delimiters: "./graphs/invalid/delimiters.agd" -> 0,
  line_more_delimiters: "./graphs/invalid/more_delimiters.agd" -> 0,
  line_question_mark: "./graphs/invalid/question_mark.agd" -> 0,
  line_too_long_comments: "./graphs/invalid/too_long_comments.agd" -> 2,

  line_too_long_comments_and_typos:
    "./graphs/invalid/too_long_comments_and_typos.agd" -> 2
);

make_test!(@main @data-err
  main_delimiters: "./graphs/invalid/delimiters.agd",
  main_more_delimiters: "./graphs/invalid/more_delimiters.agd",
  main_question_mark: "./graphs/invalid/question_mark.agd",
  main_too_long_comments: "./graphs/invalid/too_long_comments.agd",

  main_too_long_comments_and_typos:
    "./graphs/invalid/too_long_comments_and_typos.agd"
);

make_test!(@read @fail
  read_fail_incomplete_comment_1: "(",
  read_fail_incomplete_comment_2: "(...",

  read_fail_incomplete_comment_3: "(()",
  read_fail_incomplete_comment_4: "(...()",
  read_fail_incomplete_comment_5: "((...)",
  read_fail_incomplete_comment_6: "(()...",
  read_fail_incomplete_comment_7: "(...(...)",
  read_fail_incomplete_comment_8: "(...()...",
  read_fail_incomplete_comment_9: "((...)...",
  read_fail_incomplete_comment_10: "(...(...)...",

  read_fail_incomplete_comment_11: "((())",
  read_fail_incomplete_comment_12: "(...(())",
  read_fail_incomplete_comment_13: "((...())",
  read_fail_incomplete_comment_14: "(((...))",
  read_fail_incomplete_comment_15: "((()...)",
  read_fail_incomplete_comment_16: "((())...",
  read_fail_incomplete_comment_17: "(...(...())",
  read_fail_incomplete_comment_18: "(...((...))",
  read_fail_incomplete_comment_19: "(...(()...)",
  read_fail_incomplete_comment_20: "(...(())...",
  read_fail_incomplete_comment_21: "((...(...))",
  read_fail_incomplete_comment_22: "((...()...)",
  read_fail_incomplete_comment_23: "((...())...",
  read_fail_incomplete_comment_24: "(((...)...)",
  read_fail_incomplete_comment_25: "(((...))...",
  read_fail_incomplete_comment_26: "((()...)...",
  read_fail_incomplete_comment_27: "(...(...(...))",
  read_fail_incomplete_comment_28: "(...(...()...)",
  read_fail_incomplete_comment_29: "(...(...())...",
  read_fail_incomplete_comment_30: "(...((...)...)",
  read_fail_incomplete_comment_31: "(...((...))...",
  read_fail_incomplete_comment_32: "(...(()...)...",
  read_fail_incomplete_comment_33: "((...(...)...)",
  read_fail_incomplete_comment_34: "((...(...))...",
  read_fail_incomplete_comment_35: "((...()...)...",
  read_fail_incomplete_comment_36: "(((...)...)...",
  read_fail_incomplete_comment_37: "(...(...(...)...)",
  read_fail_incomplete_comment_38: "(...(...(...))...",
  read_fail_incomplete_comment_39: "(...(...()...)...",
  read_fail_incomplete_comment_40: "(...((...)...)...",
  read_fail_incomplete_comment_41: "((...(...)...)...",
  read_fail_incomplete_comment_42: "(...(...(...)...)..."
);

make_test!(@read @tokens @comment
  read_tokens_valid_comment_1: "()",
  read_tokens_valid_comment_2: "(...)",

  read_tokens_valid_comment_3: "(())",
  read_tokens_valid_comment_4: "(...())",
  read_tokens_valid_comment_5: "((...))",
  read_tokens_valid_comment_6: "(()...)",
  read_tokens_valid_comment_7: "(...(...))",
  read_tokens_valid_comment_8: "(...()...)",
  read_tokens_valid_comment_9: "((...)...)",
  read_tokens_valid_comment_10: "(...(...)...)",

  read_tokens_valid_comment_11: "((()))",
  read_tokens_valid_comment_12: "(...(()))",
  read_tokens_valid_comment_13: "((...()))",
  read_tokens_valid_comment_14: "(((...)))",
  read_tokens_valid_comment_15: "((()...))",
  read_tokens_valid_comment_16: "((())...)",
  read_tokens_valid_comment_17: "(...(...()))",
  read_tokens_valid_comment_18: "(...((...)))",
  read_tokens_valid_comment_19: "(...(()...))",
  read_tokens_valid_comment_20: "(...(())...)",
  read_tokens_valid_comment_21: "((...(...)))",
  read_tokens_valid_comment_22: "((...()...))",
  read_tokens_valid_comment_23: "((...())...)",
  read_tokens_valid_comment_24: "(((...)...))",
  read_tokens_valid_comment_25: "(((...))...)",
  read_tokens_valid_comment_26: "((()...)...)",
  read_tokens_valid_comment_27: "(...(...(...)))",
  read_tokens_valid_comment_28: "(...(...()...))",
  read_tokens_valid_comment_29: "(...(...())...)",
  read_tokens_valid_comment_30: "(...((...)...))",
  read_tokens_valid_comment_31: "(...((...))...)",
  read_tokens_valid_comment_32: "(...(()...)...)",
  read_tokens_valid_comment_33: "((...(...)...))",
  read_tokens_valid_comment_34: "((...(...))...)",
  read_tokens_valid_comment_35: "((...()...)...)",
  read_tokens_valid_comment_36: "(((...)...)...)",
  read_tokens_valid_comment_37: "(...(...(...)...))",
  read_tokens_valid_comment_38: "(...(...(...))...)",
  read_tokens_valid_comment_39: "(...(...()...)...)",
  read_tokens_valid_comment_40: "(...((...)...)...)",
  read_tokens_valid_comment_41: "((...(...)...)...)",
  read_tokens_valid_comment_42: "(...(...(...)...)...)"
);

make_test!(@read @tokens
  read_tokens_valid_sequence_1: " " -> [Space],
  read_tokens_valid_sequence_2: "." -> [FullStop],
  read_tokens_valid_sequence_3: " ." -> [Space, FullStop],
  read_tokens_valid_sequence_4: ". " -> [FullStop, Space],
  read_tokens_valid_sequence_5: ".  " -> [FullStop, Space, Space],
  read_tokens_valid_sequence_6: ".\n." -> [FullStop, LineFeed, FullStop],
  read_tokens_valid_sequence_7: ".\n  " -> [FullStop, LineFeed, Space, Space],

  read_tokens_valid_sequence_8: ".  \n." -> [
    FullStop,
    Space,
    Space,
    LineFeed,
    FullStop,
  ],

  read_tokens_valid_sequence_9: ". ()\n ." -> [
    FullStop,
    Space,
    Comment,
    LineFeed,
    Space,
    FullStop,
  ]
);

make_test!(@read @tokens
  read_tokens_unexpected_1: "\r" -> [Unexpected {
    character: '\r',
    line: 1,
    position: 1,
  }],

  read_tokens_unexpected_2: ".\r\n." -> [
    FullStop,
    Unexpected {
      character: '\r',
      line: 1,
      position: 2,
    },
    LineFeed,
    FullStop,
  ],

  read_tokens_unexpected_3: " \r\n.\r\n ()" -> [
    Space,
    Unexpected {
      character: '\r',
      line: 1,
      position: 2,
    },
    LineFeed,
    FullStop,
    Unexpected {
      character: '\r',
      line: 2,
      position: 2,
    },
    LineFeed,
    Space,
    Comment,
  ],

  read_tokens_unexpected_4: "(...)\r\n.  (... (...) ...) \r\n." -> [
    Comment,
    Unexpected {
      character: '\r',
      line: 1,
      position: 6,
    },
    LineFeed,
    FullStop,
    Space,
    Space,
    Comment,
    Space,
    Unexpected {
      character: '\r',
      line: 2,
      position: 20,
    },
    LineFeed,
    FullStop,
  ]
);

make_test!(@typos
  typos_comment: "./graphs/testing/comment.agd" -> 0,
  typos_delimiters: "./graphs/invalid/delimiters.agd" -> 3,
  typos_more_delimiters: "./graphs/invalid/more_delimiters.agd" -> 12,
  typos_question_mark: "./graphs/invalid/question_mark.agd" -> 1,
  typos_too_long_comments: "./graphs/invalid/too_long_comments.agd" -> 0,

  typos_too_long_comments_and_typos:
    "./graphs/invalid/too_long_comments_and_typos.agd" -> 1
);

#[test]
fn main_comment() {
  assert_eq!(
    GraphDescription::main(&Some(PathBuf::from(
      "./graphs/testing/comment.agd"
    ))),
    Ok(())
  );
}

#[test]
fn method_equality() {
  assert_eq!(GraphDescription::default(), GraphDescription::new());
}

/******************************************************************************/
