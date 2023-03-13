.. --------------------- GNU General Public License 3.0 --------------------- ..
..                                                                            ..
.. Copyright (C) 2023 Kevin Matthes                                           ..
..                                                                            ..
.. This program is free software: you can redistribute it and/or modify       ..
.. it under the terms of the GNU General Public License as published by       ..
.. the Free Software Foundation, either version 3 of the License, or          ..
.. (at your option) any later version.                                        ..
..                                                                            ..
.. This program is distributed in the hope that it will be useful,            ..
.. but WITHOUT ANY WARRANTY; without even the implied warranty of             ..
.. MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the              ..
.. GNU General Public License for more details.                               ..
..                                                                            ..
.. You should have received a copy of the GNU General Public License          ..
.. along with this program.  If not, see <https://www.gnu.org/licenses/>.     ..
..                                                                            ..
.. -------------------------------------------------------------------------- ..

.. -------------------------------------------------------------------------- ..
..
..  AUTHOR      Kevin Matthes
..  BRIEF       The development history of this project.
..  COPYRIGHT   GPL-3.0
..  DATE        2023
..  FILE        CHANGELOG.rst
..  NOTE        See `LICENSE' for full license.
..              See `README.md' for project details.
..
.. -------------------------------------------------------------------------- ..

.. -------------------------------------------------------------------------- ..
..
.. _changelog.d: changelog.d/
.. _Keep a Changelog: https://keepachangelog.com/en/1.0.0/
.. _Scriv: https://github.com/nedbat/scriv
..
.. -------------------------------------------------------------------------- ..

Changelog
=========

All notable changes to this project are documented in this file.  The format is
based on `Keep a Changelog`_ and optimised for maintenance with `Scriv`_.

Unreleased
----------

All pending changelog entries are stored in `changelog.d`_.

.. scriv-insert-here

.. _changelog-0.1.0:

0.1.0 — 2023-03-11
------------------

Added
.....

- README:  document rs2md

- CHANGELOG:  creation

- GitHub Action workflow:  Scriv fragment creation

- Scriv:  configuration

- Scriv:  fragment storage

- CFF:  creation

- CI:  Bors invocation job

- CI:  CFF validation

- GitHub Action workflow:  CI

- Bors:  configuration

- CODEOWNERS:  creation

- Dependabot:  GitHub Action setup

- Dependabot:  Rust setup

- bump2version:  configuration

- GitHub Action workflow:  release preparations

- Cargo.lock:  creation

- Cargo.toml:  GPL header

- .gitignore:  GPL header

- README:  GPL header

- src/main.rs:  GPL header

- README:  Bors badge

- README:  CI badge

- README:  information about the meaning of "aeruginous"

- README:  license badge

- README:  license information section

- README:  table of contents

- CI:  ``cargo c``

- CI:  ``cargo clippy``

- CI:  ``cargo t``

- rustfmt:  configuration

- src/lib.rs:  creation

- src/version.rs:  creation

- CFF:  cite Rust crate ``sysexits``

- dependency:  Rust crate ``sysexits``

- documentation:  create directory for license copies of software dependencies

- documentation:  software licenses of ``sysexits`` crate

- README:  document new directory ``LICENSEs/``

- Clippy:  configuration

- dependency:  Rust crate ``chrono``

- documentation:  software license of ``chrono`` crate

- src/running.rs:  creation

- dependency:  Rust crate ``clap``

- documentation:  documentation comments

- documentation:  software licenses of ``clap`` crate

- src/application.rs:  creation

- GitHub Action workflow:  README creation from ``src/lib.rs``

- rs2md:  creation

- CFF:  cite Rust crate ``clap``

- Cargo.toml:  project keywords

- src/lib.rs:  very strict Clippy linting settings

- README:  crates.io badge

- README:  crates.io download badge

- README:  last commit badge

Changed
.......

- src/main.rs:  ``aeruginous::Application::parse().action().run()``

- src/main.rs:  make application quit with a ``sysexits::ExitCode``

- Cargo.toml:  set minimal supported Rust version to 1.67.1

- Cargo.toml:  change project description

.. _changelog-0.0.0:

0.0.0 — 2023-03-04
------------------

Added
.....

- Cargo.toml:  creation

- .gitignore:  creation

- LICENSE:  GPL-3.0

- README:  creation

- repository:  creation

- src/main.rs:  creation

.. -------------------------------------------------------------------------- ..
