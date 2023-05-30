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
use std::collections::HashMap;
use sysexits::{ExitCode, Result};

/// Create comments on the latest changes to this repository.
pub struct CommentChangesData {
  /// Whether to query the commit messages' bodies rather than their summaries.
  body: bool,

  /// The allowed categories.
  categories: Vec<String>,

  /// The changes to report.
  changes: HashMap<String, Vec<String>>,

  /// The delimiter to separate a category from the change description.
  delimiter: String,

  /// The count of commits to analyse.
  depth: Option<usize>,

  /// The hyperlinks to define in the output file.
  hyperlinks: HashMap<String, String>,

  /// This repository.
  repository: Option<Repository>,
}

impl CommentChangesData {
  /// What is the name of the current branch?
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Unavailable`]
  /// - See [`Self::open_repository`].
  pub fn branch_name(&mut self) -> Result<String> {
    if self.repository.is_none() {
      self.open_repository()?;
    }

    let Some(repository) = &self.repository else { unreachable!() };

    repository.head().map_or_else(
      |error| {
        eprintln!("{error}");
        Err(ExitCode::Unavailable)
      },
      |reference| {
        reference
          .name()
          .map_or(Err(ExitCode::Unavailable), |name| Ok(name.to_string()))
      },
    )
  }

  /// Generate the changelog fragment.
  #[must_use]
  pub fn generate_changelog_fragment(
    &self,
    heading: u8,
    extension: &str,
  ) -> String {
    let mut result = self.resolve_links(extension);

    for (category, changes) in &self.changes {
      result.append_as_line(match extension {
        "md" => format!("{} {category}\n", "#".repeat(heading.into())),
        "rst" => format!(
          "{category}\n{}\n",
          match heading {
            1 => "=",
            2 => "-",
            3 => ".",
            _ => unreachable!(),
          }
          .repeat(category.len())
        ),
        _ => unreachable!(),
      });

      for change in changes {
        result.append_as_line(format!("- {change}\n"));
      }
    }

    result
  }

  /// Insert a new change into this instance's set of changes.
  fn insert_change(
    map: &mut HashMap<String, Vec<String>>,
    category: String,
    change: String,
  ) {
    if !map.contains_key(&category) {
      map.insert(category.clone(), Vec::new());
    }

    let mut changes = map[&category].clone();
    changes.push(change);
    map.insert(category, changes);
  }

  /// Analyse the latest changes and create a report.
  ///
  /// # Errors
  ///
  /// See
  ///
  /// - [`Self::query_last_n_commits`]
  /// - [`Self::report_changes`]
  pub fn main(
    &mut self,
    output_directory: &str,
    heading: u8,
    extension: &str,
    fallback_category: &Option<String>,
  ) -> Result<()> {
    self.query_last_n_commits(fallback_category)?;
    self.report_changes(output_directory, heading, extension)
  }

  /// Create a new instance from the command line arguments.
  #[must_use]
  pub fn new(
    depth: Option<usize>,
    delimiter: String,
    hyperlinks: HashMap<String, String>,
    categories: Vec<String>,
    body: bool,
  ) -> Self {
    Self {
      body,
      categories,
      changes: HashMap::new(),
      delimiter,
      depth,
      hyperlinks,
      repository: None,
    }
  }

  /// Open this repository.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Usage`]
  pub fn open_repository(&mut self) -> Result<()> {
    Repository::open(".").map_or_else(
      |_| {
        eprintln!("This is not a Git repository.");
        Err(ExitCode::Usage)
      },
      |repository| {
        self.repository = Some(repository);
        Ok(())
      },
    )
  }

  /// Query the given amount of commits.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::Unavailable`]
  /// - See [`Self::open_repository`].
  pub fn query_last_n_commits(
    &mut self,
    fallback_category: &Option<String>,
  ) -> Result<()> {
    if self.repository.is_none() {
      self.open_repository()?;
    }

    let Some(repository) = &self.repository else { unreachable!() };

    match repository.revwalk() {
      Ok(mut revwalk) => match revwalk.push_head() {
        Ok(()) => {
          let mut count = 1;
          let mut result = HashMap::new();

          for oid in revwalk {
            if let Some(depth) = self.depth {
              if count > depth {
                break;
              }
            }

            if let Ok(oid) = oid {
              if let Ok(commit) = repository.find_commit(oid) {
                if let Some(message) = if self.body {
                  commit.body()
                } else {
                  commit.summary()
                } {
                  if let Some((category, change)) =
                    message.trim().split_once(&self.delimiter)
                  {
                    let category = category.trim().to_string();
                    let change = change.trim().to_string();
                    let valid_category =
                      self.categories.iter().any(|c| c == &category);

                    if self.categories.is_empty() || valid_category {
                      Self::insert_change(&mut result, category, change);
                    } else if !valid_category && fallback_category.is_some() {
                      let Some(fallback) = fallback_category else { unreachable!() };
                      Self::insert_change(
                        &mut result,
                        fallback.to_string(),
                        change,
                      );
                    }
                  } else if let Some(fallback) = fallback_category {
                    Self::insert_change(
                      &mut result,
                      fallback.to_string(),
                      message.trim().to_string(),
                    );
                  }
                }
              } else {
                eprintln!("Commit {oid} does not seem to exist.");
                return Err(ExitCode::DataErr);
              }
            } else {
              eprintln!("There were not enough commits fetched on checkout.");
              return Err(ExitCode::Usage);
            }

            count += 1;
          }

          self.changes = result;

          Ok(())
        }
        Err(error) => {
          eprintln!("{error}");
          Err(ExitCode::Unavailable)
        }
      },
      Err(error) => {
        eprintln!("{error}");
        Err(ExitCode::Unavailable)
      }
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
  pub fn report_changes(
    &mut self,
    output_directory: &str,
    heading: u8,
    extension: &str,
  ) -> Result<()> {
    let branch = self.branch_name()?;
    let content = if extension == "ron" {
      ron::ser::to_string_pretty(
        &Fragment::new(&self.hyperlinks, &self.changes),
        ron::ser::PrettyConfig::default().indentor("  ".to_string()),
      )
      .map_or(Err(ExitCode::DataErr), Ok)?
    } else {
      self.generate_changelog_fragment(heading, extension)
    };
    let user = self.who_am_i()?.replace(' ', "_");

    format!(
      "{output_directory}/{}_{user}_{}.{extension}",
      chrono::Local::now().format("%Y%m%d_%H%M%S"),
      branch.split('/').last().unwrap_or("HEAD")
    )
    .write(Box::new(content))
  }

  /// Assemble the links for the resulting report.
  #[must_use]
  pub fn resolve_links(&self, extension: &str) -> String {
    let mut result = String::new();

    if !self.hyperlinks.is_empty() {
      for (link_name, target) in &self.hyperlinks {
        result.append_as_line(match extension {
          "md" => format!("[{link_name}]:  {target}"),
          "rst" => format!(".. _{link_name}:  {target}"),
          _ => unreachable!(),
        });
      }

      result.push('\n');
    }

    result
  }

  /// Who is configured as user?
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  /// - [`sysexits::ExitCode::Unavailable`]
  /// - See [`Self::open_repository`].
  pub fn who_am_i(&mut self) -> Result<String> {
    if self.repository.is_none() {
      self.open_repository()?;
    }

    let Some(repository) = &self.repository else { unreachable!() };

    repository.config().map_or_else(
      |error| {
        eprintln!("{error}");
        Err(ExitCode::Unavailable)
      },
      |config| {
        config.get_string("user.name").map_or_else(
          |_| {
            eprintln!("There is no Git username configured, yet.");
            Err(ExitCode::DataErr)
          },
          Ok,
        )
      },
    )
  }
}

/// The fragment type for exporting the harvested changes.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Fragment {
  /// The hyperlinks to references for further reading.
  references: HashMap<String, String>,

  /// The harvested changes.
  changes: HashMap<String, Vec<String>>,
}

impl Fragment {
  /// Create a new instance.
  #[must_use]
  pub fn new(
    references: &HashMap<String, String>,
    changes: &HashMap<String, Vec<String>>,
  ) -> Self {
    Self {
      references: references.clone(),
      changes: changes.clone(),
    }
  }
}

/******************************************************************************/
