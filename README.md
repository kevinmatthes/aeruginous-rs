<!------------------------------------------------------------------------->

[ci]:  https://github.com/kevinmatthes/aeruginous-rs/workflows/ci/badge.svg
[crate]:  https://crates.io/crates/aeruginous
[crates-io]:  https://img.shields.io/crates/v/aeruginous
[deps]:  https://deps.rs/repo/github/kevinmatthes/aeruginous-rs/status.svg
[deps-rs]:  https://deps.rs/repo/github/kevinmatthes/aeruginous-rs
[docs]:  https://docs.rs/aeruginous/badge.svg
[docs-rs]:  https://docs.rs/aeruginous
[downloads]:  https://img.shields.io/crates/d/aeruginous
[gpl3]:  https://github.com/kevinmatthes/aeruginous-rs/blob/main/LICENSE
[lcns]:  https://img.shields.io/github/license/kevinmatthes/aeruginous-rs
[lst]:  https://img.shields.io/github/last-commit/kevinmatthes/aeruginous-rs
[msrv]:  https://img.shields.io/badge/MSRV-1.75.0-brightgreen
[release]:  https://github.com/kevinmatthes/aeruginous-rs/releases/latest
[renovate]:  https://img.shields.io/badge/renovate-enabled-brightgreen.svg
[repository]:  https://github.com/kevinmatthes/aeruginous-rs
[tag]:  https://img.shields.io/github/v/tag/kevinmatthes/aeruginous-rs

<!------------------------------------------------------------------------->

<p align = 'center'>
<a href = 'https://github.com/kevinmatthes/aeruginous-rs'>
<img
  height = '200'
  src =
    'https://github.com/kevinmatthes/aeruginous-rs/raw/main/aeruginous.svg'
/>
</a>
<br/>
The Aeruginous Open Source Development Toolbox
</p>

## Summary

[![][ci]][repository]
[![][lst]][repository]
[![][lcns]][repository]
[![][renovate]][repository]
[![][tag]][release]
<br>
[![][crates-io]][crate]
[![][deps]][deps-rs]
[![][docs]][docs-rs]
[![][downloads]][crate]
[![][msrv]][repository]

1. [License](#license)
1. [Dependencies](#dependencies)
1. [Introduction](#introduction)
1. [Installation](#installation)
1. [Supported Subcommands](#supported-subcommands)
   1. [`cff-create`](#cff-create)
   1. [`cffreference`](#cffreference)
   1. ⚠️  [`cff-release-today`](#cff-release-today) (deprecated)
   1. [`comment-changes`](#comment-changes)
   1. [`complain`](#complain)
   1. [`increment-version`](#increment-version)
   1. [`mkcws`](#mkcws)
   1. [`ronlog`](#ronlog)
   1. [`rs2md`](#rs2md)
   1. [`uncrlf`](#uncrlf)
<!--
   1. [`cff-create`](#cff-create)
   1. [`cffreference`](#cffreference)
   1. ⚠️  [`cff-release-today`](#cff-release-today) (deprecated)
   1. [`comment-changes`](#comment-changes)
   1. [`complain`](#complain)
   1. [`graph-description`](#graph-description)
   1. [`increment-version`](#increment-version)
   1. [`mkcws`](#mkcws)
   1. [`ronlog`](#ronlog)
   1. [`rs2md`](#rs2md)
   1. [`uncrlf`](#uncrlf)
-->

The current code coverage is **<!-- cov -->74.41%<!-- cov -->**.

## License

This project's license is **GPL-3.0**.  The whole license text can be found
in [`LICENSE`][gpl3] in the repository root.  The brief version is as
follows:

> Copyright (C) 2023 Kevin Matthes
>
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
>
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU General Public License for more details.
>
> You should have received a copy of the GNU General Public License
> along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Dependencies

### GitHub Actions

- baptiste0928/cargo-install
  [![](https://img.shields.io/github/license/baptiste0928/cargo-install)
  ](https://github.com/baptiste0928/cargo-install)

- fregante/setup-git-user
  [![](https://img.shields.io/github/license/fregante/setup-git-user)
  ](https://github.com/fregante/setup-git-user)

### Rust

- [`aeruginous_io`]
  [![](https://img.shields.io/crates/l/aeruginous-io)
  ](https://github.com/kevinmatthes/aeruginous-io)

- [`anstyle`]
  [![](https://img.shields.io/crates/l/anstyle)
  ](https://github.com/rust-cli/anstyle)

- [`cargo_lock`]
  [![](https://img.shields.io/crates/l/cargo-lock)
  ](https://github.com/rustsec/rustsec)

- [`chrono`]
  [![](https://img.shields.io/crates/l/chrono)
  ](https://github.com/chronotope/chrono)

- [`clap`]
  [![](https://img.shields.io/crates/l/clap)
  ](https://github.com/clap-rs/clap)

- [`git2`]
  [![](https://img.shields.io/crates/l/git2)
  ](https://github.com/rust-lang/git2-rs)

- [`indexmap`]
  [![](https://img.shields.io/crates/l/indexmap)
  ](https://github.com/bluss/indexmap)

- [`quick_xml`]
  [![](https://img.shields.io/crates/l/quick-xml)
  ](https://github.com/tafia/quick-xml)

- [`ron`]
  [![](https://img.shields.io/crates/l/ron)
  ](https://github.com/ron-rs/ron)

- [`serde`]
  [![](https://img.shields.io/crates/l/serde)
  ](https://github.com/serde-rs/serde)

- [`sysexits`]
  [![](https://img.shields.io/crates/l/sysexits)
  ](https://github.com/sorairolake/sysexits-rs)

- [`toml`]
  [![](https://img.shields.io/crates/l/toml)
  ](https://github.com/toml-rs/toml)

## Introduction

`aeruginous` is a Rust application providing several development utilities.

When searching a name for this project, one main requirement was to reflect
both the originally intended main purpose of tracking time as well as the
coding language this CLI is written in, Rust.  The adjective *aeruginous*
fulfills both criteria as it means that the described noun has patina, a
special form of rust which appears after a certain period of time has
passed.

Originally, it was planned to be a time tracking CLI but during the
development of the first stable version, certain common tasks needed to be
fulfilled repeatedly.  Since the application already had a somehow stable
calling interface, the solutions to these tasks were added as subcommands
to `aeruginous` in order to provide a convenient and time efficient
automation.  One major advantage of doing so is the reduced maintenance
effort and overall setup overhead because there is only one project to
maintain instead multiple ones.

This is how the idea arose to design `aeruginous` to be a toolbox instead
of only a time tracker.

## Installation

To download the latest stable version from [`crates.io`][crate], run the
following command.

```bash
cargo install aeruginous
```

To install the latest nightly version from sources, Cargo also supports the
installation from the current repository state.

```bash
cargo install --git https://github.com/kevinmatthes/aeruginous-rs
```

## Supported Subcommands

### `cff-create`

> To be called with:
>
> - `cffcreate`
> - `cff-create`
> - `mkcff`

> To be installed with:
>
> - `-F cff-create`

This mode will analyse a given project manifest and create an initial
CITATION.cff from it.  Please note that the result does not necessarily
validate such that further adjustments are recommended.

Supported manifest formats are:

- Cargo.toml (`rs`)

### `cffreference`

> To be called with:
>
> - `cffref`
> - `cff-ref`
> - `cffreference`
> - `cff-reference`

CFF makes software citable.  Projects exposing a `CITATION.cff` can be cited
with APA plain text citations, BibTeX database entries, and also in another
`CITATION.cff`'s list of references.

This subcommand grabs the citation information of the named source CFF file
and pastes it at the end of the given output file.

If the input file is omitted, the input information are attemted to be read
from [`std::io::Stdin`].  Likewise, omitting the output file will cause
`cffreference` to write to [`std::io::Stdout`].

### `cff-release-today`

⚠️  This mode is deprecated.  Please use
[`increment-version`](#increment-version) instead.  ⚠️

> To be called with:
>
> - `cffrel`
> - `cff-rel`
> - `cffreleasetoday`
> - `cff-release-today`

This subcommand will set the release date in the given `CITATION.cff` to the
present day.

### `comment-changes`

> To be called with:
>
> - `changelog`
> - `comment-changes`

It is a good practice to document changes to the code base in a CHANGELOG.
This mode will read the recent commit messages and try to create a fragment
for the CHANGELOG.

This mode requires the specification of a delimiter separating the CHANGELOG
category from an entry for that category.  The application will browse the
Git history for commits which contain that delimiter in their messages and
split those messages at the first occurence of that delimiter; users are
free to choose whether to prefer the commits' summaries or their bodies,
defaulting to the former.  The count of commits to harvest can be controlled
by either an exact number, a commit SHA to stop at, or by omitting any stop
condition to consider all commits in the entire history.  Each commit which
does not contain the given delimiter in its message will be skipped.  The
resulting CHANGELOG fragment will be stored either in the current working
directory or in the given alternative directory.  The file name will consist
of a time stamp, the configured Git username, and some information on the
current branch.  The file format can be either reStructured Text (RST),
Markdown (MD), or the Rusty Object Notation (RON).  At option, hyperlinks
can be specified.

As an example, a repository might contain these four commits:

1. ```Added ::= source file `a.rs`_```
2. ```Added ::= source file `b.rs`_```
3. `Update c.rs`
4. ```Fixed ::= known bug in `d.rs`_```

To extract the changes from only these four commits, the application would
need to be called with the following command.

```bash
aeruginous comment-changes \
  -d ::= \
  -n 4 \
  -o directory/ \
  -l a.rs -t src/a.rs \
  -l b.rs -t src/b.rs \
  -l d.rs -t src/d.rs
```

If this command is invoked by a user named Emma Xample on 1st January 1970
at 01.23 am with the branch `example/test` being checked out, the resulting
fragment will be stored as `directory/19700101_012345_Emma_Xample_test.rst`.
The file contents will be the following:

```rst
.. _a.rs:  src/a.rs
.. _b.rs:  src/b.rs
.. _d.rs:  src/d.rs

Added
.....

- source file `a.rs`_

- source file `b.rs`_

Fixed
.....

- known bug in `d.rs`_

```

### `complain`

> To be called with:
>
> - `complain`

This application mode is a little linter to check whether the following
requirements are met:

1. Every file needs to be terminated by a line feed.
1. Files must not contain CRLFs.
1. Lines shall have a width of at most n characters.
1. Trailing white space characters must be removed.
1. Lines have to be indented by spaces / tabs.
1. Spaces and tabs must not be mixed for indentation.
1. Within any line, there shall not be any tab character.

All rules can be ignored, the line width as well as the indentation unit can
be configured.  Every violation is reported to [`std::io::Stderr`] with the
number of the rule being highlighted using the following colours.

| Colour | Meaning                       |
|:------:|:------------------------------|
| green  | easy to fix                   |
| yellow | moderate difficulty of fixing |
| red    | major changes required to fix |

After all rules have been checked for one file, a summary will be written to
[`std::io::Stderr`] consisting of an ASCII art crab as this application is
written in Rust, the number of violations, as well as the file name.

<!--
### `graph-description`

> To be called with:
>
> - `agd`
> - `graph-description`

The Aeruginous Graph Description is a very easy to learn coding language to
describe the structure of graphs.  The language itself is based on plain
English to ensure that learning does not require any programming skills at
all.

This mode is not finished, yet, but it can already detect some issues
regarding given input files.
-->

### `increment-version`

> To be called with:
>
> - incver
> - inc-ver
> - incrementversion
> - increment-version

This subcommand will increment the hard-coded version strings in the given
files by the specified version range.

### `mkcws`

> To be called with:
>
> - `mkcws`

> To be installed with:
>
> - `-F mkcws`

IDEs based on the source code of Visual Studio Code have the interesting
feature of keeping the current editor view across subsequent sessions.  When
starting the IDE, it will restore the latest state in order to enable a
seamless continuation of work.  Users are allowed to export the current view
as a Code Workspace to save their access to multiple editor states.  These
Code Workspaces are configuration files using a JSON-based notation in order
to store information on the Workspace's root directory as well as some
optional settings unique to that particular Workspace.

Usually, operating systems can be configured regarding the default
application for handling a certain file type.  This also holds for Code
Workspaces.  If the operating system is set to open Code Workspaces with a
Visual Studio Code-like IDE, the Code Workspaces can be used as project
launching shortcuts for a convenient user expericence with the IDE.

This application mode aims to simplify the creation of new Code Workspace
files by the provision of a rather simple and intuitive command line
interface to define a minimal and valid Code Workspace from scratch.

### `ronlog`

> To be called with:
>
> - `ronlog`

This mode will collect the RON fragments created by `comment-changes` and
assemble them to a RON CHANGELOG.

A RONLOG consists of multiple sections, sorted descendingly by the
respective versions they are documenting.  New sections are inserted into
that sorted list without breaking the sorting.  For example, if a particular
RONLOG should contain sections for some versions v1.0.0, v0.2.0, and v0.1.0,
a new section on v0.3.0 would be inserted between v1.0.0 and v0.2.0.

### `rs2md`

> To be called with:
>
> - `rs2md`

Source code should always be documented.  Rust's documentation system
supports Markdown syntax in documentation comments.  Thus, it is a
convenient decision to create a Rust project's README file from the crate
root's documentation.  This command is also helpful to check the
documentation comments for typos.

When called, the subcommand accepts a list of input files to read from.  If
no input file is given, `rs2md` will read from [`std::io::Stdin`].  At
option, an output file can be specified where the results will be written
to.  If omitted, the results will be written to [`std::io::Stdout`].

Users are free to choose whether they would like to extract Rust comments
starting with `//!` (outer comments) or comments starting with `///` (inner
comments).  If neither option is given, nothing will be extracted.

### `uncrlf`

> To be called with:
>
> - `uncrlf`

Source code should have a uniform appearance.  Some text editors terminate
lines by Carriage Return Line Feeds (CRLFs, `\r\n`).  This subcommand will
remove those from the given file.

<!------------------------------------------------------------------------->
