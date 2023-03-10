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
//! [gpl3]:  https://github.com/kevinmatthes/aeruginous-rs/blob/main/LICENSE
//! [lcns]:  https://img.shields.io/github/license/kevinmatthes/aeruginous-rs
//! [lcnss]:  https://github.com/kevinmatthes/aeruginous-rs/tree/main/LICENSEs
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
//! [![][lcns]][repository]
//!
//! Yet another time tracking CLI, written in Rust.
//!
//! 1. [License](#license)
//! 1. [Introduction](#introduction)
//! 1. [Meaning of the Name](#meaning-of-the-name)
//!
//! ## License
//!
//! ![[][lcns]][repository]
//!
//! This project's license is **GPL-3.0**.  The whole license text can be found
//! in [`LICENSE`][gpl3] in the repository root.  The brief version is as
//! follows:
//!
//! License information about the dependencies this software requires to work
//! can be found in [`LICENSEs`][lcnss].
//!
//! ## Introduction
//!
//! `aeruginous` is a time tracking CLI, written in Rust.  Its main purpose is
//! to create and maintain working hours, stored in fragments named "time
//! frames".
//!
//! ## Meaning of the Name
//!
//! When searching a name for this project, one main requirement was to reflect
//! both the purpose of tracking time as well as the coding language this CLI is
//! written in, Rust.  The adjective *aeruginous* fulfills both criterions as it
//! means that the described noun has patina, a special form of rust which
//! appears after a certain period of time has passed.
//!
//! <!------------------------------------------------------------------------->

mod action;
mod clap;
mod constants;
mod running;
mod version;

pub use crate::{
  action::Action,
  clap::Clap,
  constants::{CRATE_NAME, CRATE_VERSION, SELF_DESCRIPTION},
  running::Running,
  version::{Version, VersionParsingError},
};

/******************************************************************************/
