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
  GraphDescription,
};

macro_rules! make_test {
  ( @parse @fail $( $name:ident : $string:literal ),+ ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = GraphDescription::new();
        assert_eq!(agd.parse($string), Err(sysexits::ExitCode::DataErr));
      }
    )+
  };

  (
    @parse @tokens $(
      $name:ident : $string:literal -> $expectation:tt
    ),+
  ) => {
    $(
      #[test]
      fn $name() {
        let mut agd = GraphDescription::new();
        agd.parse($string).unwrap();

        assert_eq!(agd.tokens(), &$expectation);
      }
    )+
  };

  ( @parse @tokens @comment $( $name:ident : $string:literal ),+ ) => {
    make_test!(
      @parse @tokens $(
        $name : $string -> [Comment]
      ),+
    );
  };
}

make_test!(@parse @fail
  parse_fail_incomplete_comment_1: "(",
  parse_fail_incomplete_comment_2: "(...",

  parse_fail_incomplete_comment_3: "(()",
  parse_fail_incomplete_comment_4: "(...()",
  parse_fail_incomplete_comment_5: "((...)",
  parse_fail_incomplete_comment_6: "(()...",
  parse_fail_incomplete_comment_7: "(...(...)",
  parse_fail_incomplete_comment_8: "(...()...",
  parse_fail_incomplete_comment_9: "((...)...",
  parse_fail_incomplete_comment_10: "(...(...)...",

  parse_fail_incomplete_comment_11: "((())",
  parse_fail_incomplete_comment_12: "(...(())",
  parse_fail_incomplete_comment_13: "((...())",
  parse_fail_incomplete_comment_14: "(((...))",
  parse_fail_incomplete_comment_15: "((()...)",
  parse_fail_incomplete_comment_16: "((())...",
  parse_fail_incomplete_comment_17: "(...(...())",
  parse_fail_incomplete_comment_18: "(...((...))",
  parse_fail_incomplete_comment_19: "(...(()...)",
  parse_fail_incomplete_comment_20: "(...(())...",
  parse_fail_incomplete_comment_21: "((...(...))",
  parse_fail_incomplete_comment_22: "((...()...)",
  parse_fail_incomplete_comment_23: "((...())...",
  parse_fail_incomplete_comment_24: "(((...)...)",
  parse_fail_incomplete_comment_25: "(((...))...",
  parse_fail_incomplete_comment_26: "((()...)...",
  parse_fail_incomplete_comment_27: "(...(...(...))",
  parse_fail_incomplete_comment_28: "(...(...()...)",
  parse_fail_incomplete_comment_29: "(...(...())...",
  parse_fail_incomplete_comment_30: "(...((...)...)",
  parse_fail_incomplete_comment_31: "(...((...))...",
  parse_fail_incomplete_comment_32: "(...(()...)...",
  parse_fail_incomplete_comment_33: "((...(...)...)",
  parse_fail_incomplete_comment_34: "((...(...))...",
  parse_fail_incomplete_comment_35: "((...()...)...",
  parse_fail_incomplete_comment_36: "(((...)...)...",
  parse_fail_incomplete_comment_37: "(...(...(...)...)",
  parse_fail_incomplete_comment_38: "(...(...(...))...",
  parse_fail_incomplete_comment_39: "(...(...()...)...",
  parse_fail_incomplete_comment_40: "(...((...)...)...",
  parse_fail_incomplete_comment_41: "((...(...)...)...",
  parse_fail_incomplete_comment_42: "(...(...(...)...)..."
);

make_test!(@parse @tokens @comment
  parse_tokens_valid_comment_1: "()",
  parse_tokens_valid_comment_2: "(...)",

  parse_tokens_valid_comment_3: "(())",
  parse_tokens_valid_comment_4: "(...())",
  parse_tokens_valid_comment_5: "((...))",
  parse_tokens_valid_comment_6: "(()...)",
  parse_tokens_valid_comment_7: "(...(...))",
  parse_tokens_valid_comment_8: "(...()...)",
  parse_tokens_valid_comment_9: "((...)...)",
  parse_tokens_valid_comment_10: "(...(...)...)",

  parse_tokens_valid_comment_11: "((()))",
  parse_tokens_valid_comment_12: "(...(()))",
  parse_tokens_valid_comment_13: "((...()))",
  parse_tokens_valid_comment_14: "(((...)))",
  parse_tokens_valid_comment_15: "((()...))",
  parse_tokens_valid_comment_16: "((())...)",
  parse_tokens_valid_comment_17: "(...(...()))",
  parse_tokens_valid_comment_18: "(...((...)))",
  parse_tokens_valid_comment_19: "(...(()...))",
  parse_tokens_valid_comment_20: "(...(())...)",
  parse_tokens_valid_comment_21: "((...(...)))",
  parse_tokens_valid_comment_22: "((...()...))",
  parse_tokens_valid_comment_23: "((...())...)",
  parse_tokens_valid_comment_24: "(((...)...))",
  parse_tokens_valid_comment_25: "(((...))...)",
  parse_tokens_valid_comment_26: "((()...)...)",
  parse_tokens_valid_comment_27: "(...(...(...)))",
  parse_tokens_valid_comment_28: "(...(...()...))",
  parse_tokens_valid_comment_29: "(...(...())...)",
  parse_tokens_valid_comment_30: "(...((...)...))",
  parse_tokens_valid_comment_31: "(...((...))...)",
  parse_tokens_valid_comment_32: "(...(()...)...)",
  parse_tokens_valid_comment_33: "((...(...)...))",
  parse_tokens_valid_comment_34: "((...(...))...)",
  parse_tokens_valid_comment_35: "((...()...)...)",
  parse_tokens_valid_comment_36: "(((...)...)...)",
  parse_tokens_valid_comment_37: "(...(...(...)...))",
  parse_tokens_valid_comment_38: "(...(...(...))...)",
  parse_tokens_valid_comment_39: "(...(...()...)...)",
  parse_tokens_valid_comment_40: "(...((...)...)...)",
  parse_tokens_valid_comment_41: "((...(...)...)...)",
  parse_tokens_valid_comment_42: "(...(...(...)...)...)"
);

make_test!(@parse @tokens
  parse_tokens_valid_sequence_1: " " -> [Space],
  parse_tokens_valid_sequence_2: "." -> [FullStop],
  parse_tokens_valid_sequence_3: " ." -> [Space, FullStop],
  parse_tokens_valid_sequence_4: ". " -> [FullStop, Space],
  parse_tokens_valid_sequence_5: ".  " -> [FullStop, Space, Space],
  parse_tokens_valid_sequence_6: ".\n." -> [FullStop, LineFeed, FullStop],
  parse_tokens_valid_sequence_7: ".\n  " -> [FullStop, LineFeed, Space, Space],

  parse_tokens_valid_sequence_8: ".  \n." -> [
    FullStop,
    Space,
    Space,
    LineFeed,
    FullStop,
  ],

  parse_tokens_valid_sequence_9: ". ()\n ." -> [
    FullStop,
    Space,
    Comment,
    LineFeed,
    Space,
    FullStop,
  ]
);

make_test!(@parse @tokens
  parse_tokens_unexpected_1: "\r" -> [Unexpected {
    character: '\r',
    line: 1,
    position: 1,
  }],

  parse_tokens_unexpected_2: ".\r\n." -> [
    FullStop,
    Unexpected {
      character: '\r',
      line: 1,
      position: 2,
    },
    LineFeed,
    FullStop,
  ],

  parse_tokens_unexpected_3: " \r\n.\r\n ()" -> [
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

  parse_tokens_unexpected_4: "(...)\r\n.  (... (...) ...) \r\n." -> [
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

#[test]
fn method_equality() {
  assert_eq!(GraphDescription::default(), GraphDescription::new());
}

/******************************************************************************/
