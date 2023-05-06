#![deny(
  clippy::all,
  clippy::cargo,
  clippy::complexity,
  clippy::correctness,
  clippy::nursery,
  clippy::pedantic,
  clippy::perf,
  clippy::suspicious,
  clippy::style,
  dead_code,
  deprecated,
  missing_docs,
  rustdoc::broken_intra_doc_links,
  unused_macros
)]
#![allow(clippy::multiple_crate_versions)]

mod application;
mod graph_description;
mod macros;
mod pattern;
mod running;
mod traits;
mod version;

pub use crate::{
  application::{Action, Clap as Application},
  graph_description::{GraphDescription, Tokens as AgdTokens},
  pattern::{
    Buffer as PatternBuffer, IOProcessor as PatternIOProcessor,
    Reader as PatternReader, Writer as PatternWriter,
  },
  running::Running,
  traits::{
    AppendAsLine, AppendAsLine as PatternAppendAsLine, ConvertBuffer, Prefer,
  },
  version::Version,
};

/// This crate's name.
pub const NAME: &str = "aeruginous";

/// This crate's self-description.
pub const SELF_DESCRIPTION: &str =
  "The Aeruginous Open Source Development Toolbox";

/// This crate's version.
pub const VERSION: &str = "v0.2.1";

/******************************************************************************/
