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

//! The application's subcommands.

use crate::{CRATE_VERSION, SELF_DESCRIPTION};
use chrono::Local;
use clap::Subcommand;
use sysexits::ExitCode;

/// The supported application modes.
///
/// Depending on the given command line arguments, `aeruginous` will show a
/// different behaviour.
#[derive(Subcommand)]
pub enum Action {
  /// Show some information on this application.
  Info,

  /// Show the current time.
  Now,
}

impl Action {
  /// Show some information on this application.
  fn info() -> ExitCode {
    println!("This is the {SELF_DESCRIPTION}, {CRATE_VERSION}.");
    ExitCode::Ok
  }

  /// Show the current time.
  fn now() -> ExitCode {
    println!("{}", Local::now());
    ExitCode::Ok
  }

  /// Execute the selected action.
  pub fn run(&self) -> ExitCode {
    match self {
      Self::Info => Self::info(),
      Self::Now => Self::now(),
    }
  }
}

/******************************************************************************/
