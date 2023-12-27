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

#![cfg(feature = "cff-create")]

use aeruginous::{
    CffCreate, CffCreateManifestType, PatternWriter, ReadFile, NAME,
    SELF_DESCRIPTION, VERSION,
};
use chrono::Local;
use std::fs::remove_file;
use sysexits::ExitCode;

#[test]
fn cargo_toml_author_without_email() {
    let i = "cargo_author_without_email.toml";
    let o = "cargo_author_without_email.cff";
    let mut cc = CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o));

    cc.suppress_release_date();
    i.truncate(Box::new(
        "\
[package]
authors = [\"Emma Xample\"]
"
        .to_string(),
    ))
    .unwrap();

    assert!(cc.main().is_ok());
    assert_eq!(
        o.read().unwrap(),
        "\
authors:
  - name: Emma Xample
cff-version: 1.2.0
keywords:
  - Rust
message: Please cite this project using these information.
"
    );

    remove_file(i).unwrap();
    remove_file(o).unwrap();
}

#[test]
fn cargo_toml_empty() {
    let i = "cargo_empty.toml";
    let o = "cargo_empty.cff";

    i.truncate(Box::new(String::default())).unwrap();

    assert_eq!(
        CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o)).main(),
        Err(ExitCode::DataErr)
    );

    remove_file(i).unwrap();
}

#[test]
fn cargo_toml_empty_package_section() {
    let i = "cargo_empty_package_section.toml";
    let o = "cargo_empty_package_section.cff";
    let mut cc = CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o));

    cc.suppress_release_date();
    i.truncate(Box::new("[package]\n".to_string())).unwrap();

    assert!(cc.main().is_ok());
    assert_eq!(
        o.read().unwrap(),
        "\
cff-version: 1.2.0
keywords:
  - Rust
message: Please cite this project using these information.
",
    );

    remove_file(i).unwrap();
    remove_file(o).unwrap();
}

#[test]
fn cargo_toml_invalid_author() {
    let i = "cargo_invalid_author.toml";
    let o = "cargo_invalid_author.cff";

    i.truncate(Box::new("[package]\nauthors = [\"\"]\n".to_string()))
        .unwrap();

    assert_eq!(
        CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o)).main(),
        Err(ExitCode::DataErr)
    );

    remove_file(i).unwrap();
}

#[test]
fn cargo_toml_invalid_toml() {
    let i = "cargo_invalid_toml.toml";
    let o = "cargo_invalid_toml.cff";

    i.truncate(Box::new("!\n".to_string())).unwrap();

    assert_eq!(
        CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o)).main(),
        Err(ExitCode::DataErr)
    );

    remove_file(i).unwrap();
}

#[test]
fn cargo_toml_multiple_licenses() {
    let i = "cargo_multiple_licenses.toml";
    let o = "cargo_multiple_licenses.cff";
    let mut cc = CffCreate::new(Some(i), CffCreateManifestType::Rust, Some(o));

    cc.suppress_release_date();
    i.truncate(Box::new(
        "[package]\nlicense = \"Apache-2.0 OR MIT\"".to_string(),
    ))
    .unwrap();

    assert!(cc.main().is_ok());
    assert_eq!(
        o.read().unwrap(),
        "\
cff-version: 1.2.0
keywords:
  - Rust
license:
  - Apache-2.0
  - MIT
message: Please cite this project using these information.
"
    );

    remove_file(i).unwrap();
    remove_file(o).unwrap();
}

#[test]
fn project_manifest() {
    let o = "cargo.cff";

    CffCreate::new(Some("Cargo.toml"), CffCreateManifestType::Rust, Some(o))
        .main()
        .unwrap();

    assert_eq!(
        o.read().unwrap(),
        format!(
            "\
abstract: {}.
authors:
  - email: aeruginous.rs@gmail.com
    name: Kevin Matthes
cff-version: 1.2.0
date-released: {}
keywords:
  - Rust
  - aeruginous
  - changelog
  - citation-file-format
  - command-line-utilities
  - config
  - development-tools
  - linter
  - rust-patterns
  - template-engine
  - toolbox
license: GPL-3.0
message: Please cite this project using these information.
repository-code: https://github.com/kevinmatthes/aeruginous-rs
title: {}
url: https://github.com/kevinmatthes/aeruginous-rs
version: {}
",
            SELF_DESCRIPTION,
            Local::now().date_naive().format("%Y-%m-%d"),
            NAME,
            VERSION.trim_matches('v')
        )
    );

    remove_file(o).unwrap();
}

/******************************************************************************/
