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

.. -------------------------------------------------------------------------- ..
