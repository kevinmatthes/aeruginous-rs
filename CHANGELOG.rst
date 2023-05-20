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

.. _changelog-1.0.0:

1.0.0 — 2023-05-20
------------------

Added
.....

- Cargo.toml:  project keyword ``uncrlf``

- README:  document uncrlf

- uncrlf:  creation

- src/pattern/io_processor.rs:  PatternIOProcessor::io_write

- src/pattern/io_processor.rs:  PatternIOProcessor::io_write_silently

- src/pattern/writer.rs:  PatternWriter::truncate

- src/pattern/writer.rs:  PatternWriter::truncate_silently

- src/pattern/reader.rs:  impl PatternReader for Option<PathBuf>

- src/pattern/writer.rs:  impl PatternWriter for Option<PathBuf>

- src/traits/append_as_line.rs:  AppendAsLine

- src/traits/append_as_line.rs:  creation

- src/traits/mod.rs:  creation

- src/traits/prefer.rs:  creation

- src/traits/prefer.rs:  Prefer

- tests/prefer.rs:  creation

- src/macros.rs:  creation

- src/macros.rs:  getters

- src/macros.rs:  implement

- tests/getters.rs:  creation

- tests/implement.rs:  creation

- tests/pattern_writer.rs:  creation

- src/traits/convert_buffer.rs:  ConvertBuffer

- src/traits/convert_buffer.rs:  creation

- tests/convert_buffer.rs:  creation

- tests/running.rs:  creation

- src/graph_description.rs:  creation

- src/graph_description.rs:  AgdTokens

- src/graph_description.rs:  AeruginousGraphDescription

- tests/graph_description.rs:  creation

- dependency:  Rust crate ``anstyle``

- documentation:  software licenses of ``anstyle`` crate

- graphs/invalid/delimiters.agd:  creation

- graphs/invalid/more_delimiters.agd:  creation

- graphs/invalid/question_mark.agd:  creation

- graphs/invalid/too_long_comments.agd:  creation

- graphs/invalid/too_long_comments_and_typo.agd:  creation

- graphs/examples/comment.agd:  creation

- tests/append_as_line.rs:  creation

- tests/pattern_buffer.rs:  creation

- graph-description:  creation

- README:  document graph-description

- lints:  deny dead code

- lints:  deny unused imports

- lints:  deny unused macros

- lints:  deny unused parentheses

- src/traits/colour_message.rs:  creation

- src/traits/colour_message.rs:  ColourMessage

- tests/traits/colour_message.rs:  creation

- Cargo.toml:  project keyword ``cff``

- Cargo.toml:  project keyword ``cffref``

- Cargo.toml:  project keyword ``cff-reference``

- Cargo.toml:  project keyword ``citation-file-format``

- CITATION.cff:  project keyword ``cff``

- CITATION.cff:  project keyword ``cffref``

- CITATION.cff:  project keyword ``cff-reference``

- CITATION.cff:  project keyword ``citation-file-format``

- src/version.rs:  Version::new

- tests/version.rs:  creation

- graphs/examples/etc.agd:  creation

- src/macros.rs:  ceprint

- src/macros.rs:  ceprintln

- graphs/invalid/bad_spacing.agd:  creation

- graphs/invalid/missing_line_feed.agd:  creation

- graphs/invalid/wrong_order.agd:  creation

- lints:  deny unused assignments

- lints:  deny unused function results of functions marked ``#[must_use]``

- lints:  deny unused parenthesis

- lints:  deny unused variables

- src/traits/to_stderr.rs:  creation

- src/traits/to_stderr.rs:  ToStderr

- tests/to_stderr.rs:  creation

- CI:  code coverage update on mirroring PR

- README:  mention current code coverage in summary section

- README:  comment out AGD mode description for intermediate release

- src/application.rs:  comment out AGD mode for intermediate release

Changed
.......

- use own macros to render getter methods

- CI:  mirroring workflow now creates PR for changes

- CI:  MSRV upgrade workflow now creates PR for changes

- PatternReader:  rely on std::fs::read_to_string

- apply new features of sysexits v0.6.0

- README:  unite sections "Introduction" and "Meaning of the Name"

- MSRV:  1.69.0

Fixed
.....

- PatternIOProcessor::io and PatternIOProcessor::io_silent did not truncate the
  output file before writing to it

Removed
.......

- PatternIOProcessor::process

- PatternReader::read_bytes

- PatternReader::read_string

- PatternWriter::write_bytes

- PatternWriter::write_string

- Running::create

- Version::ParsingError

- PatternAppendAsLine

- src/pattern/append_as_line.rs

- Bors:  configuration

- README:  Bors badge

- README:  notes on deprecated symbols

.. _changelog-0.2.1:

0.2.1 — 2023-04-25
------------------

Added
.....

- README:  installation instructions

- src/pattern/buffer.rs:  creation

- src/pattern/io_processor.rs:  creation

- src/pattern/mod.rs:  creation

- src/pattern/reader.rs:  creation

- src/pattern/writer.rs:  creation

- lints:  deny deprecated symbols

- lints:  deny missing documentation

- README:  notes on deprecated symbols

- src/pattern/buffer.rs:  PatternBuffer

- src/pattern/io_processor.rs:  PatternIOProcessor::behaviour

- src/pattern/io_processor.rs:  PatternIOProcessor::io

- src/pattern/io_processor.rs:  PatternIOProcessor::io_append

- src/pattern/io_processor.rs:  PatternIOProcessor::io_append_silently

- src/pattern/io_processor.rs:  PatternIOProcessor::io_silent

- src/pattern/reader.rs:  PatternReader::behaviour

- src/pattern/reader.rs:  PatternReader::read

- src/pattern/reader.rs:  PatternReader::read_silently

- src/pattern/writer.rs:  PatternWriter::append

- src/pattern/writer.rs:  PatternWriter::append_silently

- src/pattern/writer.rs:  PatternWriter::behaviour

- src/pattern/writer.rs:  PatternWriter::write

- src/pattern/writer.rs:  PatternWriter::write_silently

- CFF:  cite CFF project

- GitHub Action workflow:  ``cargo fmt`` on PR

- src/pattern/reader.rs:  impl PatternReader for std::io::Stdin

- src/pattern/reader.rs:  impl PatternReader for PathBuf

- src/pattern/writer.rs:  impl PatternWriter for PathBuf

- src/pattern/writer.rs:  impl PatternWriter for std::io::Stdout

- GitHub Action workflow:  weekly Rust MSRV upgrade

- CI:  ``cargo fmt --check``

- lints:  deny broken links in documentation

- GitHub Action workflow:  code coverage determination on PR

- .gitignore:  Tarpaulin reports

- Tarpaulin:  configuration

- src/pattern/writer.rs:  impl PatternWriter for std::io::Stderr

- src/pattern/append_as_line.rs:  creation

- src/pattern/append_as_line.rs:  PatternAppendAsLine

Changed
.......

- apply new sysexits::Result type and semantics

- MSRV:  1.69.0

Deprecated
..........

- PatternIOProcessor::process

- PatternReader::read_bytes

- PatternReader::read_string

- PatternWriter::write_bytes

- PatternWriter::write_string

- Running::create

- Version::ParsingError

Fixed
.....

- CI:  mirror workflow now upgrades Rust during README mirroring job

- src/pattern/reader.rs:  only first 8192 bytes of file were read

Removed
.......

- src/pattern_io_processor.rs

- src/pattern_reader.rs

- src/pattern_writer.rs

.. _changelog-0.2.0:

0.2.0 — 2023-03-14
------------------

Added
.....

- cffreference:  creation

- README:  document cffreference

- src/pattern_io_processor.rs:  creation

- src/pattern_reader.rs:  creation

- src/pattern_writer.rs:  creation

- CFF:  cite Rust crate ``chrono``

- Cargo.toml:  project keyword ``cffreference``

- CFF:  project keyword ``cffreference``

- README:  docs.rs badge

Changed
.......

- src/application.rs:  apply new Rust coding pattern inspired traits

- Cargo.toml:  sort package metadata by alphabet

- GitHub Action workflow:  rename README mirroring workflow

- rs2md:  make Boolean switch presence suffice

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
