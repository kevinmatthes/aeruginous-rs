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

use sysexits::{ExitCode, Result};

/// An Aeruginous Graph Description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphDescription {
  /// The known identifiers.
  identifiers: Vec<String>,

  /// The held string literals.
  string_literals: Vec<String>,

  /// The held tokens.
  tokens: Vec<Tokens>,
}

impl GraphDescription {
  crate::getters!(@fn @ref
    identifiers: Vec<String>,
    string_literals: Vec<String>,
    tokens: Vec<Tokens>
  );

  /// Initialise a new instance.
  #[must_use]
  pub const fn new() -> Self {
    Self {
      identifiers: Vec::new(),
      string_literals: Vec::new(),
      tokens: Vec::new(),
    }
  }

  /// Fill this instance based on an input file's contents.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  pub fn parse(&mut self, s: &str) -> Result<()> {
    let mut comment_depth = 0usize;
    let mut line = 1;
    let mut pending_token = None::<Tokens>;
    let mut position = 0;

    for character in s.chars() {
      position += 1;

      match pending_token {
        Some(token) => match token {
          Tokens::Comment => match character {
            '(' => {
              comment_depth += 1;
            }
            ')' => {
              comment_depth -= 1;

              if comment_depth == 0 {
                self.tokens.push(token);
                pending_token = None;
              }
            }
            _ => continue,
          },
          _ => unreachable!(),
        },
        None => match character {
          '\n' => {
            line += 1;
            position = 0;
            self.tokens.push(Tokens::LineFeed);
          }
          ' ' => {
            self.tokens.push(Tokens::Space);
          }
          '(' => {
            comment_depth += 1;
            pending_token = Some(Tokens::Comment);
          }
          '.' => {
            self.tokens.push(Tokens::FullStop);
          }
          _ => {
            self.tokens.push(Tokens::Unexpected {
              character,
              line,
              position,
            });
          }
        },
      }
    }

    if pending_token.is_none() {
      Ok(())
    } else {
      Err(ExitCode::DataErr)
    }
  }
}

impl Default for GraphDescription {
  fn default() -> Self {
    Self::new()
  }
}

/// The possible token types for the Aeruginous Graph Description.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tokens {
  /// The `Abbreviate` keyword.
  Abbreviate,

  /// The `and` keyword.
  And,

  /// The `by` keyword.
  By,

  /// A comment.
  Comment,

  /// The `Connect` keyword.
  Connect,

  /// The `Declare` keyword.
  Declare,

  /// The `.` keyword.
  FullStop,

  /// An identifier's index within a [`GraphDescription`]'s set of identifiers.
  Identifier(usize),

  /// A simple line feed.
  LineFeed,

  /// A space character.
  Space,

  /// A string literal's index within a [`GraphDescription`]'s set of string
  /// literals.
  StringLiteral(usize),

  /// An unexpected character.
  Unexpected {
    /// The unexpected character.
    character: char,

    /// The line in which the unexpected character occurs.
    line: usize,

    /// The column in which the unexpected character occurs.
    position: usize,
  },
}

/******************************************************************************/
