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

.. _changelog-2.1.0:

2.1.0 — 2023-06-04
------------------

Added
.....

- README:  deps.rs badge

- tests/ronlog_section.rs

- Fragment::changes

- Fragment::references

- Fragment::move_references

- RonlogReferences

- RonlogSection

- src/changelog/section.rs:  creation

- README:  document ronlog

- RonlogSection::add_changes

- impl Default for Fragment

- Version:  error message for parsing failure

- FromRon

- src/traits/from_ron.rs:  creation

- ronlog:  creation

- RonlogAction

- Ronlog

- src/changelog/ronlog.rs:  creation

- src/changelog/action.rs:  creation

- tests/ronlog_action.rs:  creation

- impl PartialOrd for Version

- impl Ord for Version

- CFF:  entry for ``clap`` v4.3.1

- tests/ron_traits.rs

Changed
.......

- src/application.rs:  simplify source code

- comment-changes:  simplify source code

- cffreference:  simplify source code

Fixed
.....

- comment-changes:  amend existing fragments

Removed
.......

- tests/to_ron.rs

.. _changelog-2.0.0:

2.0.0 — 2023-06-01
------------------

Added
.....

- comment-changes:  allow for three different heading levels

- comment-changes:  make ``-c`` and ``-k`` options stack

- comment-changes:  support for Markdown fragments

- impl PatternWriter for Option<&PathBuf>

- impl PatternWriter for &PathBuf

- macro ``impl_pattern_writer_for``

- impl PatternWriter for Option<&str>

- impl PatternWriter for &Option<String>

- impl PatternWriter for Option<String>

- impl PatternWriter for &str

- impl PatternWriter for String

- documentation:  software licenses of ``serde`` crate

- documentation:  software licenses of ``ron`` crate

- comment-changes:  allow for RON fragments

- dependency:  Rust crate ``serde``

- dependency:  Rust crate ``ron``

- comment-changes:  query commit message bodies on demand

- comment-changes:  allow for fallback category

- CFF:  entry for ``chrono`` v0.4.25

- src/cffreference.rs:  Cffreference

- src/cffreference.rs:  creation

- CI:  update MSRV badge in README, as well

- README:  MSRV badge

- tests/cffreference.rs:  creation

- Cffreference::new

- CI:  validate sample CFFs for unit tests

- impl PatternReader for &str

- cffs/input_5.cff:  creation

- cffs/input_4.cff:  creation

- cffs/input_3.cff:  creation

- cffs/input_2.cff:  creation

- cffs/input_1.cff:  creation

- cffs/expectation.cff:  creation

- cffs/input_7.cff:  creation

- cffs/input_6.cff:  creation

- tests/fragment.rs:  creation

- Fragment::merge

- tests/to_ron.rs:  creation

- ToRon

- src/traits/to_ron.rs:  creation

- CFF:  entry for ``chrono`` v0.4.26

Changed
.......

- src/graph_description.rs:  move to src/graphing/

- src/graph.rs:  move to src/graphing/

- src/graphing/graph_description.rs:  rename to src/graphing/agd.rs

- move cffreference logic to own source file

- CI:  release workflow submits changes as PR

- comment-changes:  break library API

- comment-changes:  refactor logic

- Fragment:  move implementation to src/changelog/fragment.rs

- src/comment_changes.rs:  move to src/changelog/

Fixed
.....

- README:  wrong copyright years

- comment-changes:  quit on empty commit messages

- rs2md:  use singular form for option ``input_file``

- unnecessary dependency features

- README:  rename example branch to ``example/test``

Removed
.......

- CommentChanges::update_changes

- comment-changes:  unit tests

.. _changelog-1.1.2:

1.1.2 — 2023-05-27
------------------

Added
.....

- CI:  ``cargo nextest r``

- CI:  ``cargo b``

- CI:  ``cargo d``

- src/graph.rs:  creation

- tests/graph.rs:  creation

- CI:  only allow for the categories configured for Scriv

- comment-changes:  option to configure Keep a Changelog categories

Changed
.......

- CI:  simplify CI workflow

- comment-changes:  configure default directory using Clap

- src/version.rs:  adjust spacing

- src/running.rs:  adjust spacing

- ``cargo update``

- CI:  commit message of MSRV workflow

Fixed
.....

- CI:  shell script of MSRV workflow

- comment-changes:  wrong interpretation of categories

Removed
.......

- CI:  run ``comment-changes`` on PR

.. _changelog-1.1.1:

1.1.1 — 2023-05-25
------------------

Added
.....

- comment-changes:  category checker

- tests/comment_changes.rs:  test case for link resolution

- src/comment_changes.rs:  CommentChanges::resolve_links

Fixed
.....

- src/comment_changes.rs:  link correct method in documentation

- CI:  ``cargo update`` workflow now meets ``comment-changes`` requirements

- comment-changes:  missing ``_`` in link resolution

.. _changelog-1.1.0:

1.1.0 — 2023-05-23
------------------

Added
.....

- README:  document comment-changes

- comment-changes:  creation

- lints:  deny unused ``mut`` keyword

- src/comment_changes.rs:  creation

- Cargo.lock:  dependencies of ``git2``

- Cargo.toml:  dependency ``git2``

- documentation:  software licenses of ``git2`` crate

- CITATION.cff:  project keyword ``comment-changes``

- tests/comment_changes.rs:  creation

- GitHub Action workflow:  ``cargo update`` on demand

- GitHub Action workflow:  ``comment-changes`` on demand

- CI:  run ``comment-changes`` on PR creation

- comment-changes:  ignore whitespaces around delimiter

- comment-changes:  link resolution for target file

- CITATION.cff:  project keyword ``changelog``

- README:  verbose example for ``comment-changes``

- CLI:  allow for long argument parameter names

- Cargo.toml:  project keyword ``changelog``

Changed
.......

- ``cargo update``

- CI:  commit message of MSRV upgrade

- CI:  commit messages of README mirroring workflow

Fixed
.....

- CI:  settings of MSRV upgrade workflow

- CI:  settings of README mirroring workflow

Removed
.......

- CI:  Scriv fragment creation workflow

.. -------------------------------------------------------------------------- ..
