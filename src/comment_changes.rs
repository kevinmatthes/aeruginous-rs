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

use crate::{AppendAsLine, PatternWriter};
use git2::Repository;
use std::{collections::HashMap, path::PathBuf};
use sysexits::{ExitCode, Result};

/// Create comments on the latest changes to this repository.
pub struct CommentChanges {
  /// The changes to report.
  changes: HashMap<String, Vec<String>>,

  /// The delimiter to separate a category from the change description.
  delimiter: String,

  /// The count of commits to analyse.
  depth: Option<usize>,

  /// This repository.
  repository: Option<Repository>,
}

impl CommentChanges {
  /// Which is the current branch?
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Software`]
  /// - [`sysexits::ExitCode::Unavailable`]
  pub fn branch_name(&self) -> Result<String> {
    self
      .repository
      .as_ref()
      .map_or(Err(ExitCode::Software), |repository| {
        repository
          .head()
          .map_or(Err(ExitCode::Unavailable), |reference| {
            reference
              .name()
              .map_or(Err(ExitCode::Unavailable), |name| Ok(name.to_string()))
          })
      })
  }

  /// Analyse the latest changes and create a report.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`Self::open_repository`]
  /// - [`Self::query_last_n_commits`]
  /// - [`Self::report_changes`]
  pub fn main(&mut self) -> Result<()> {
    self.open_repository()?;
    self.changes = self.query_last_n_commits()?;
    self.report_changes()
  }

  /// Create a new instance from the command line arguments.
  #[must_use]
  pub fn new(depth: Option<usize>, delimiter: String) -> Self {
    Self {
      changes: HashMap::new(),
      delimiter,
      depth,
      repository: None,
    }
  }

  /// Open this repository.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Usage`]
  pub fn open_repository(&mut self) -> Result<()> {
    if let Ok(repository) = Repository::open(".") {
      self.repository = Some(repository);
      Ok(())
    } else {
      eprintln!("This is not a Git repository.");
      Err(ExitCode::Usage)
    }
  }

  /// Query the given amount of commits.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Software`]
  /// - [`sysexits::ExitCode::Unavailable`]
  pub fn query_last_n_commits(&self) -> Result<HashMap<String, Vec<String>>> {
    let mut result = HashMap::new();

    match self.repository.as_ref() {
      Some(repository) => match repository.revwalk() {
        Ok(mut revwalk) => match revwalk.push_head() {
          Ok(()) => {
            let mut count = 1;

            for oid in revwalk {
              if let Some(depth) = self.depth {
                if count > depth {
                  break;
                }
              }

              match oid {
                Ok(oid) => match repository.find_commit(oid) {
                  Ok(commit) => match commit.summary() {
                    Some(summary) => {
                      if let Some((category, change)) =
                        summary.split_once(&self.delimiter)
                      {
                        if !result.contains_key(category) {
                          result.insert(category.to_string(), Vec::new());
                        }

                        let mut changes = result[category].clone();
                        changes.push(change.to_string());
                        result.insert(category.to_string(), changes);
                      }
                    }
                    None => return Err(ExitCode::Unavailable),
                  },
                  Err(_) => return Err(ExitCode::Unavailable),
                },
                Err(_) => return Err(ExitCode::Unavailable),
              }

              count += 1;
            }

            Ok(result)
          }
          Err(_) => Err(ExitCode::Unavailable),
        },
        Err(_) => Err(ExitCode::Unavailable),
      },
      None => Err(ExitCode::Software),
    }
  }

  /// Report the logged changes.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`Self::branch_name`]
  /// - [`Self::who_am_i`]
  pub fn report_changes(&self) -> Result<()> {
    let mut changelog = String::new();

    for (category, vector) in &self.changes {
      changelog.append_as_line(format!(
        "{category}\n{}\n",
        ".".repeat(category.len())
      ));

      for item in vector {
        changelog.append_as_line(format!("- {item}\n"));
      }
    }

    let branch = self.branch_name()?;
    let user = self.who_am_i()?.replace(' ', "_");

    PathBuf::from(format!(
      "changelog.d/{}_{user}_{}.rst",
      chrono::Local::now().format("%Y%m%d_%H%M%S"),
      branch.split('/').last().unwrap_or("HEAD")
    ))
    .write(Box::new(changelog))
  }

  /// Who is configured as user?
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  /// - [`sysexits::ExitCode::Unavailable`]
  pub fn who_am_i(&self) -> Result<String> {
    self
      .repository
      .as_ref()
      .map_or(Err(ExitCode::Software), |repository| {
        repository
          .config()
          .map_or(Err(ExitCode::Unavailable), |config| {
            config.get_string("user.name").map_or_else(
              |_| {
                eprintln!("There is no Git user name configured, yet.");
                Err(ExitCode::DataErr)
              },
              Ok,
            )
          })
      })
  }
}

/******************************************************************************/
