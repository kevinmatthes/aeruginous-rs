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

use crate::{ceprintln, PatternReader};
use sysexits::{ExitCode, Result};

/// An Aeruginous Graph Description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphDescription {
  /// The buffer holding the characters of the pending token.
  buffer: String,

  /// The current comment depth.
  comment_depth: usize,

  /// The count of identifiers found so far.
  count_identifiers: usize,

  /// The count of string literals found so far.
  count_strings: usize,

  /// The known identifiers.
  identifiers: Vec<String>,

  /// The current line.
  line: usize,

  /// The pending token.
  pending_token: Option<Tokens>,

  /// The current column position.
  position: usize,

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

  /// Assume that the next token is going to be a comment.
  fn assume_comment(&mut self) {
    self.comment_depth += 1;
    self.pending_token = Some(Tokens::Comment);
  }

  /// Assume that the next token is going to be an identifier.
  fn assume_identifier(&mut self, character: char) {
    self.buffer.push(character);
    self.pending_token = Some(Tokens::Identifier(self.count_identifiers));
  }

  /// Assume that the next token is going to be a line feed.
  fn assume_line_feed(&mut self) {
    self.line += 1;
    self.position = 0;
    self.pending_token = Some(Tokens::LineFeed(1));
  }

  /// The detected syntax issues in the input source file.
  ///
  /// These mistakes are critical, so they will be written in red.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::IoErr`]
  pub fn check_for_syntax_issues(&mut self) -> Result<usize> {
    let mut result = 0;

    if !matches!(self.tokens.last(), Some(Tokens::LineFeed(_))) {
      result += 1;

      ceprintln!(
        "Syntax "!Red,
        "problem:  every source file needs to be terminated by a line feed."
      );
    }

    Ok(result)
  }

  /// The detected typos in the input source file.
  ///
  /// The typos will be named well human readable by writing the found
  /// character and the line as well as the column position it is found in to
  /// [`std::io::Stderr`].  As typos are mistakes which are easy to fix, the
  /// error message will be written in green.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::IoErr`]
  pub fn check_for_typos(&self) -> Result<usize> {
    let mut result = 0;

    for token in &self.tokens {
      match token {
        Tokens::Unexpected {
          character,
          line,
          position,
        } => {
          result += 1;

          ceprintln!(
            "  Typo "!Green,
            "'{character}' in line {line} at position {position}."
          );
        }
        _ => continue,
      }
    }

    Ok(result)
  }

  /// Determine whether all lines fit the line width of 80 characters.
  ///
  /// A line is allowed to consist of at most 80 characters and a line feed.  If
  /// a line should have more characters, this is a sign that the overall design
  /// of the source file urgently deserves a refactoring.  Hence, this is a more
  /// critical issue than just a simple typo which is why the indication colour
  /// of the error message is written in yellow.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::IoErr`]
  pub fn check_line_width(&self, input: &str) -> Result<usize> {
    let mut column = 0;
    let mut line = 1;
    let mut result = 0;

    for character in input.chars() {
      if character == '\n' {
        if column > 80 {
          result += 1;

          ceprintln!(
            "  Line "!Yellow,
            "{line} is {} characters too long.",
            column - 80
          );
        }

        column = 0;
        line += 1;
      } else {
        column += 1;
      }
    }

    Ok(result)
  }

  /// Push the pending token and match the current character.
  fn finalise_pending_token(&mut self, token: Tokens, character: char) {
    self.tokens.push(token);
    self.pending_token = None;
    self.match_character(character);
  }

  /// The main function for the Aeruginous Graph Description processing.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`crate::PatternBuffer::try_into_string`]
  /// - [`PatternReader::read`]
  /// - [`Self::check_for_typos`]
  /// - [`Self::read`]
  pub fn main(input: &Option<std::path::PathBuf>) -> Result<()> {
    let mut agd = Self::new();
    let input = input.read()?.try_into_string()?;
    let lines = agd.check_line_width(&input)?;
    agd.read(&input)?;
    let typos = agd.check_for_typos()?;
    let syntax = agd.check_for_syntax_issues()?;
    let sum = lines + syntax + typos;

    if sum == 0 {
      Ok(())
    } else {
      ceprintln!(
        "Failed "!Red,
        "due to {sum} issue{} to fix.",
        if sum == 1 { "" } else { "s" }
      );

      Err(ExitCode::DataErr)
    }
  }

  /// Match an input character against the possible redirections.
  fn match_character(&mut self, character: char) {
    match character {
      '\n' => self.assume_line_feed(),
      ' ' => self.pending_token = Some(Tokens::Space(1)),
      '"' => {
        self.pending_token = Some(Tokens::StringLiteral(self.count_strings));
      }
      '(' => self.assume_comment(),
      '.' => self.tokens.push(Tokens::FullStop),
      'A'..='Z' | 'a'..='z' => self.assume_identifier(character),
      _ => self.push_unexpected(character),
    }
  }

  /// Initialise a new instance.
  #[must_use]
  pub const fn new() -> Self {
    Self {
      buffer: String::new(),
      comment_depth: 0,
      count_identifiers: 0,
      count_strings: 0,
      identifiers: Vec::new(),
      line: 1,
      pending_token: None,
      position: 0,
      string_literals: Vec::new(),
      tokens: Vec::new(),
    }
  }

  /// Push an identifier to the list of tokens.
  fn push_identifier(&mut self) {
    let identifier = self.buffer.clone();
    self.buffer.clear();
    self.pending_token = None;

    match identifier.as_str() {
      "Abbreviate" => self.tokens.push(Tokens::Abbreviate),
      "Connect" => self.tokens.push(Tokens::Connect),
      "Declare" => self.tokens.push(Tokens::Declare),
      "and" => self.tokens.push(Tokens::And),
      "by" => self.tokens.push(Tokens::By),
      _ => {
        self.identifiers.push(identifier);
        self.tokens.push(Tokens::Identifier(self.count_identifiers));

        self.count_identifiers += 1;
      }
    }
  }

  /// Push a string literal to the list of tokens.
  fn push_string(&mut self) {
    self.string_literals.push(self.buffer.clone());
    self.tokens.push(Tokens::StringLiteral(self.count_strings));

    self.buffer.clear();
    self.pending_token = None;
    self.count_strings += 1;
  }

  /// Push an unexpected character to the list of tokens.
  fn push_unexpected(&mut self, character: char) {
    self.tokens.push(Tokens::Unexpected {
      character,
      line: self.line,
      position: self.position,
    });
  }

  /// Fill this instance based on an input file's contents.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  pub fn read(&mut self, s: &str) -> Result<()> {
    for character in s.chars() {
      self.position += 1;

      match self.pending_token {
        Some(token) => match token {
          Tokens::Comment => match character {
            '(' => {
              self.comment_depth += 1;
            }
            ')' => {
              self.comment_depth -= 1;

              if self.comment_depth == 0 {
                self.tokens.push(token);
                self.pending_token = None;
              }
            }
            _ => continue,
          },
          Tokens::Identifier(_) => {
            if matches!(
              character,
              '0'..='9'
              | 'A'..='Z'
              | 'a'..='z'
              | '_'
              | '-'
            ) {
              self.buffer.push(character);
            } else {
              self.push_identifier();
              self.match_character(character);
            }
          }
          Tokens::LineFeed(n) => {
            if character == '\n' {
              self.line += 1;
              self.position = 0;
              self.pending_token = Some(Tokens::LineFeed(n + 1));
            } else {
              self.finalise_pending_token(token, character);
            }
          }
          Tokens::Space(n) => {
            if character == ' ' {
              self.pending_token = Some(Tokens::Space(n + 1));
            } else {
              self.finalise_pending_token(token, character);
            }
          }
          Tokens::StringLiteral(_) => {
            if character == '"' {
              self.push_string();
            } else {
              self.buffer.push(character);
            }
          }
          _ => unreachable!(),
        },
        None => self.match_character(character),
      }
    }

    if self.pending_token.is_none() {
      Ok(())
    } else {
      match self.pending_token {
        Some(token) => {
          if matches!(token, Tokens::LineFeed(_) | Tokens::Space(_)) {
            self.tokens.push(token);
            self.pending_token = None;
            Ok(())
          } else {
            eprintln!("This source file is not ready for review, yet.");
            Err(ExitCode::DataErr)
          }
        }
        None => unreachable!(),
      }
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
  LineFeed(usize),

  /// A space character.
  Space(usize),

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
