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

use crate::PatternReader;
use anstyle::{AnsiColor, Style};
use std::io::stderr;
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
  pub fn line_width(&self, input: &str) -> Result<usize> {
    let mut column = 0;
    let mut line = 1;
    let line_width_colour =
      Style::new().fg_color(Some(AnsiColor::Yellow.into()));
    let mut result = 0;

    for character in input.chars() {
      if character == '\n' {
        if column > 80 {
          result += 1;

          match line_width_colour.write_to(&mut stderr()) {
            Ok(()) => {
              eprint!("  Line ");

              match line_width_colour.write_reset_to(&mut stderr()) {
                Ok(()) => {
                  eprintln!("{line} is {} characters too long.", column - 80);
                }
                Err(error) => {
                  eprintln!("{error}");
                  return Err(ExitCode::IoErr);
                }
              }
            }
            Err(error) => {
              eprintln!("{error}");
              return Err(ExitCode::IoErr);
            }
          }
        }

        column = 0;
        line += 1;
      } else {
        column += 1;
      }
    }

    Ok(result)
  }

  /// The main function for the Aeruginous Graph Description processing.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`crate::PatternBuffer::try_into_string`]
  /// - [`PatternReader::read`]
  /// - [`Self::read`]
  /// - [`Self::typos`]
  pub fn main(input: &Option<std::path::PathBuf>) -> Result<()> {
    let mut agd = Self::new();
    let input = input.read()?.try_into_string()?;
    let lines = agd.line_width(&input)?;
    agd.read(&input)?;
    let typos = agd.typos()?;
    let sum = lines + typos;

    if sum == 0 {
      Ok(())
    } else {
      let failure_colour = Style::new().fg_color(Some(AnsiColor::Red.into()));

      match failure_colour.write_to(&mut stderr()) {
        Ok(()) => {
          eprint!("Failed ");

          match failure_colour.write_reset_to(&mut stderr()) {
            Ok(()) => {
              eprintln!(
                "due to {sum} issue{} to fix.",
                if sum == 1 { "" } else { "s" }
              );
              Err(ExitCode::DataErr)
            }
            Err(error) => {
              eprintln!("{error}");
              Err(ExitCode::IoErr)
            }
          }
        }
        Err(error) => {
          eprintln!("{error}");
          Err(ExitCode::IoErr)
        }
      }
    }
  }

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
  pub fn read(&mut self, s: &str) -> Result<()> {
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
      eprintln!("This source file is not ready for review, yet.");
      Err(ExitCode::DataErr)
    }
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
  pub fn typos(&self) -> Result<usize> {
    let mut result = 0;
    let typo_colour = Style::new().fg_color(Some(AnsiColor::Green.into()));

    for token in &self.tokens {
      match token {
        Tokens::Unexpected {
          character,
          line,
          position,
        } => {
          result += 1;

          match typo_colour.write_to(&mut stderr()) {
            Ok(()) => {
              eprint!("  Typo ");

              match typo_colour.write_reset_to(&mut stderr()) {
                Ok(()) => {
                  eprintln!(
                    "'{character}' in line {line} at position {position}.",
                  );
                }
                Err(error) => {
                  eprintln!("{error}");
                  return Err(ExitCode::IoErr);
                }
              }
            }
            Err(error) => {
              eprintln!("{error}");
              return Err(ExitCode::IoErr);
            }
          }
        }
        _ => continue,
      }
    }

    Ok(result)
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
