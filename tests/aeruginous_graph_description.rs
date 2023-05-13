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
  AeruginousGraphDescription,
  AgdTokens::{
    Abbreviate, And, By, Comment, Connect, Declare, FullStop, Identifier,
    LineFeed, Space, StringLiteral, Unexpected,
  },
};
use std::{fs::read_to_string, path::PathBuf};
use sysexits::ExitCode;

macro_rules! make_test {
  ( @line $( $name:ident : $path:literal -> $lines:tt ),+ ) => {
    $(
      #[test]
      fn $name() {
        let agd = AeruginousGraphDescription::new();
        let input = read_to_string($path).unwrap();

        assert_eq!(agd.check_line_width(&input), Ok($lines));
      }
    )+
  };

  ( @main @data-err $( $name:ident : $path:literal ),+ ) => {
    $(
      #[test]
      fn $name() {
        assert_eq!(
          AeruginousGraphDescription::main(&Some(PathBuf::from($path))),
          Err(ExitCode::DataErr)
        );
      }
    )+
  };

  ( @read @fail $( $name:ident : $string:literal ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = AeruginousGraphDescription::new();
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
        let mut agd = AeruginousGraphDescription::new();
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

  ( @syntax $( $name:ident : $path:literal -> $syntax:tt ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = AeruginousGraphDescription::new();
        let input = read_to_string($path).unwrap();
        agd.read(&input).unwrap();

        assert_eq!(agd.check_for_syntax_issues(), Ok($syntax));
      }
    )+
  };

  ( @typos $( $name:ident : $path:literal -> $typos:tt ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = AeruginousGraphDescription::new();
        let input = read_to_string($path).unwrap();
        agd.read(&input).unwrap();

        assert_eq!(agd.check_for_typos(), Ok($typos));
      }
    )+
  };
}

make_test!(@line
  line_comment: "./graphs/examples/comment.agd" -> 0,
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
  read_fail_incomplete_comment_42: "(...(...(...)...)...",

  read_fail_incomplete_string_1: "\"",
  read_fail_incomplete_string_2: "\"..."
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
  read_tokens_valid_sequence_1: " " -> [Space(1)],
  read_tokens_valid_sequence_2: "." -> [FullStop],
  read_tokens_valid_sequence_3: " ." -> [Space(1), FullStop],
  read_tokens_valid_sequence_4: ". " -> [FullStop, Space(1)],
  read_tokens_valid_sequence_5: ".  " -> [FullStop, Space(2)],
  read_tokens_valid_sequence_6: ".\n." -> [FullStop, LineFeed(1), FullStop],
  read_tokens_valid_sequence_7: ".\n  " -> [FullStop, LineFeed(1), Space(2)],

  read_tokens_valid_sequence_8: ".  \n." -> [
    FullStop,
    Space(2),
    LineFeed(1),
    FullStop,
  ],

  read_tokens_valid_sequence_9: ". ()\n ." -> [
    FullStop,
    Space(1),
    Comment,
    LineFeed(1),
    Space(1),
    FullStop,
  ],

  read_tokens_valid_sequence_10: "\"\"" -> [StringLiteral(0)],
  read_tokens_valid_sequence_11: "\"...\"" -> [StringLiteral(0)],

  read_tokens_valid_sequence_12: "(...) \"...\" (...)" -> [
    Comment,
    Space(1),
    StringLiteral(0),
    Space(1),
    Comment,
  ],

  read_tokens_valid_sequence_13: "(...) \"...\" (...) \"\" (...)" -> [
    Comment,
    Space(1),
    StringLiteral(0),
    Space(1),
    Comment,
    Space(1),
    StringLiteral(1),
    Space(1),
    Comment,
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
    LineFeed(1),
    FullStop,
  ],

  read_tokens_unexpected_3: " \r\n.\r\n ()" -> [
    Space(1),
    Unexpected {
      character: '\r',
      line: 1,
      position: 2,
    },
    LineFeed(1),
    FullStop,
    Unexpected {
      character: '\r',
      line: 2,
      position: 2,
    },
    LineFeed(1),
    Space(1),
    Comment,
  ],

  read_tokens_unexpected_4: "(...)\r\n.  (... (...) ...) \r\n." -> [
    Comment,
    Unexpected {
      character: '\r',
      line: 1,
      position: 6,
    },
    LineFeed(1),
    FullStop,
    Space(2),
    Comment,
    Space(1),
    Unexpected {
      character: '\r',
      line: 2,
      position: 20,
    },
    LineFeed(1),
    FullStop,
  ]
);

make_test!(@syntax
  syntax_missing_newline: "./graphs/invalid/missing_line_feed.agd" -> 1
);

make_test!(@typos
  typos_comment: "./graphs/examples/comment.agd" -> 0,
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
    AeruginousGraphDescription::main(&Some(PathBuf::from(
      "./graphs/examples/comment.agd"
    ))),
    Ok(())
  );
}

#[test]
fn main_etc() {
  let mut agd = AeruginousGraphDescription::new();
  let input = read_to_string("./graphs/examples/etc.agd").unwrap();
  agd.read(&input).unwrap();

  assert_eq!(
    agd.tokens(),
    &[
      Comment,
      LineFeed(2),
      Abbreviate,
      Space(1),
      StringLiteral(0),
      Space(1),
      By,
      Space(1),
      Identifier(0),
      FullStop,
      LineFeed(1),
      Declare,
      Space(1),
      StringLiteral(1),
      FullStop,
      LineFeed(1),
      Connect,
      Space(1),
      StringLiteral(2),
      Space(1),
      And,
      Space(1),
      Identifier(1),
      FullStop,
      LineFeed(1),
    ]
  );
}

#[test]
fn method_equality() {
  assert_eq!(
    AeruginousGraphDescription::default(),
    AeruginousGraphDescription::new()
  );
}

#[test]
fn read_string_literals() {
  let mut agd = AeruginousGraphDescription::new();
  agd.read("(...) \"abc ...\" (...) \"def\" (...)").unwrap();

  assert_eq!(
    agd.tokens(),
    &[
      Comment,
      Space(1),
      StringLiteral(0),
      Space(1),
      Comment,
      Space(1),
      StringLiteral(1),
      Space(1),
      Comment
    ]
  );
  assert_eq!(
    agd.string_literals(),
    &["abc ...".to_string(), "def".to_string()]
  );
}

/******************************************************************************/
