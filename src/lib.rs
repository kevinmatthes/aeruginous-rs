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

//! <!------------------------------------------------------------------------->
//!
//! [bors]:  https://bors.tech/images/badge_small.svg
//! [bors-url]:  https://app.bors.tech/repositories/63092
//! [ci]:  https://github.com/kevinmatthes/aeruginous-rs/workflows/ci/badge.svg
//! [ci-url]:  https://github.com/kevinmatthes/aeruginous-rs/workflows/ci
//! [crates-io]:  https://img.shields.io/crates/v/aeruginous
//! [crates-io-url]:  https://crates.io/crates/aeruginous
//! [docs-rs]:  https://docs.rs/aeruginous/badge.svg
//! [docs-rs-url]:  https://docs.rs/aeruginous
//! [downloads]:  https://img.shields.io/crates/d/aeruginous
//! [gpl3]:  https://github.com/kevinmatthes/aeruginous-rs/blob/main/LICENSE
//! [lcns]:  https://img.shields.io/github/license/kevinmatthes/aeruginous-rs
//! [lcnss]:  https://github.com/kevinmatthes/aeruginous-rs/tree/main/LICENSEs
//! [lst]:  https://img.shields.io/github/last-commit/kevinmatthes/aeruginous-rs
//! [repository]:  https://github.com/kevinmatthes/aeruginous-rs
//!
//! <!------------------------------------------------------------------------->
//!
//! # `aeruginous`
//!
//! ## Summary
//!
//! [![][bors]][bors-url]
//! [![][ci]][ci-url]
//! [![][crates-io]][crates-io-url]
//! [![][docs-rs]][docs-rs-url]
//! [![][downloads]][crates-io-url]
//! [![][lst]][repository]
//! [![][lcns]][repository]
//!
//! The Aeruginous Open Source Development Toolbox.
//!
//! 1. [License](#license)
//! 1. [Introduction](#introduction)
//! 1. [Meaning of the Name](#meaning-of-the-name)
//! 1. [Supported Subcommands](#supported-subcommands)
//!    1. [`cffreference`](#cffreference)
//!    1. [`rs2md`](#rs2md)
//!
//! ## License
//!
//! This project's license is **GPL-3.0**.  The whole license text can be found
//! in [`LICENSE`][gpl3] in the repository root.  The brief version is as
//! follows:
//!
//! > Copyright (C) 2022???2023 Kevin Matthes
//! >
//! > This program is free software: you can redistribute it and/or modify
//! > it under the terms of the GNU General Public License as published by
//! > the Free Software Foundation, either version 3 of the License, or
//! > (at your option) any later version.
//! >
//! > This program is distributed in the hope that it will be useful,
//! > but WITHOUT ANY WARRANTY; without even the implied warranty of
//! > MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! > GNU General Public License for more details.
//! >
//! > You should have received a copy of the GNU General Public License
//! > along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! License information about the dependencies this software requires to work
//! can be found in [`LICENSEs`][lcnss].
//!
//! ## Introduction
//!
//! `aeruginous` is a Rust application providing several development utilities.
//!
//! Originally, it was planned to be a time tracking CLI but during the
//! development of the first stable version, certain common tasks needed to be
//! fulfilled repeatedly.  Since the application already had a somehow stable
//! calling interface, the solutions to these tasks were added as subcommands
//! to `aeruginous` in order to provide a convenient and time efficient
//! automation.  One major advantage of doing so is the reduced maintenance
//! effort and overall setup overhead because there is only one project to
//! maintain instead multiple ones.
//!
//! This is how the idea arose to design `aeruginous` to be a toolbox instead
//! of only a time tracker.
//!
//! ## Meaning of the Name
//!
//! When searching a name for this project, one main requirement was to reflect
//! both the originally intended main purpose of tracking time as well as the
//! coding language this CLI is written in, Rust.  The adjective *aeruginous*
//! fulfills both criterions as it means that the described noun has patina, a
//! special form of rust which appears after a certain period of time has
//! passed.
//!
//! ## Supported Subcommands
//!
//! ### `cffreference`
//!
//! CFF makes software citable.  Projects exposing a `CITATION.cff` can be cited
//! with APA plain text citations, BibTeX database entries, and also in another
//! `CITATION.cff`'s list of references.
//!
//! This subcommand grabs the citation information of the named source CFF file
//! and pastes it at the end of the given output file.
//!
//! If the input file is omitted, the input information are attemted to be read
//! from `stdin`.  Likewise, omitting the output file will cause `cffreference`
//! to write to `stdout`.
//!
//! ### `rs2md`
//!
//! Source code should always be documented.  Rust's documentation system
//! supports Markdown syntax in documentation comments.  Thus, it is a
//! convenient decision to create a Rust project's README file from the crate
//! root's documentation.  This command is also helpful to check the
//! documentation comments for typos.
//!
//! When called, the subcommand accepts a list of input files to read from.  If
//! no input file is given, `rs2md` will read from `stdin`.  At option, an
//! output file can be specified where the results will be written to.  If
//! omitted, the results will be written to `stdout`.
//!
//! Users are free to choose whether they would like to extract Rust comments
//! starting with `//!` (outer comments) or comments starting with `///` (inner
//! comments).  If neither option is given, nothing will be extracted.
//!
//! <!------------------------------------------------------------------------->

#![deny(
  clippy::all,
  clippy::cargo,
  clippy::complexity,
  clippy::correctness,
  clippy::nursery,
  clippy::pedantic,
  clippy::perf,
  clippy::suspicious,
  clippy::style
)]

mod application;
mod pattern_io_processor;
mod pattern_reader;
mod pattern_writer;
mod running;
mod version;

pub use crate::{
  application::Clap as Application,
  pattern_io_processor::PatternIOProcessor,
  pattern_reader::PatternReader,
  pattern_writer::PatternWriter,
  running::Running,
  version::{ParsingError as VersionParsingError, Version},
};

/// This crate's name.
pub const NAME: &str = "aeruginous";

/// This crate's self-description.
pub const SELF_DESCRIPTION: &str =
  "The Aeruginous Open Source Development Toolbox";

/// This crate's version.
pub const VERSION: &str = "v0.2.0";

/******************************************************************************/
