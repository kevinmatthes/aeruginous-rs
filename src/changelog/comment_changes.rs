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

/// Create comments on the commits of a branch in this repository.
#[derive(clap::Parser, Clone)]
#[command(aliases = ["changelog"])]
pub struct CommentChanges {
  /// Work with the commit messages' bodies instead of their summaries.
  #[arg(long, short = 'b')]
  body: bool,

  /// Only these categories shall be used to generate comments.
  #[arg(long, short = 'c')]
  category: Vec<String>,

  /// The delimiter to separate a category from the change description.
  #[arg(long, short = 'd')]
  delimiter: String,

  /// The count of commits to analyse, defaulting to infinity, if omitted.
  #[arg(aliases = ["count"], long, short = 'n')]
  depth: Option<usize>,

  /// The target format of the resulting fragment.
  #[arg(
      aliases = ["format"],
      default_value = "rst",
      long,
      short = 'f',
      value_parser = |f: &str| {
        if ["md", "ron", "rst"].contains(&f) {
          Ok(f.to_string())
        } else {
          Err(format!("extension '{f}' is not supported, yet"))
        }
      }
    )]
  extension: String,

  /// The default category to assign.
  #[arg(long, short = 'C')]
  fallback_category: Option<String>,

  /// The heading's level in the resulting fragment.
  #[arg(
      aliases = ["level"],
      default_value = "3",
      long,
      short = 'H',
      value_parser = clap::value_parser!(u8).range(1..=3)
    )]
  heading: u8,

  /// Set categories Added, Changed, Deprecated, Fixed, Removed, and Security.
  #[arg(long, short = 'k')]
  keep_a_changelog: bool,

  /// The hyperlinks to add as comments.
  #[arg(aliases = ["hyperlink"], long, short = 'l')]
  link: Vec<String>,

  /// The directory to write the generated fragment to.
  #[arg(
      aliases = ["dir", "directory"],
      default_value = ".",
      long = "output",
      short = 'o'
    )]
  output_directory: String,

  /// The hyperlinks' targets.
  #[arg(long, short = 't')]
  target: Vec<String>,
}

impl CommentChanges {
  /// Process the input data.
  ///
  /// # Errors
  ///
  /// - [`sysexits::ExitCode::DataErr`]
  /// - [`sysexits::ExitCode::Software`]
  /// - [`sysexits::ExitCode::Unavailable`]
  /// - [`sysexits::ExitCode::Usage`]
  pub fn main(&self) -> Result<()> {
    self.wrap().main()
  }

  /// Create a new instance.
  #[must_use]
  pub fn new(delimiter: String) -> Self {
    Self {
      body: false,
      category: Vec::new(),
      delimiter,
      depth: None,
      extension: "rst".to_string(),
      fallback_category: None,
      heading: 3,
      keep_a_changelog: false,
      link: Vec::new(),
      output_directory: ".".to_string(),
      target: Vec::new(),
    }
  }

  fn wrap(&self) -> Logic {
    Logic {
      branch: String::new(),
      categories: Vec::new(),
      changes: HashMap::new(),
      cli: self.clone(),
      hyperlinks: HashMap::new(),
      repository: None,
      user: String::new(),
    }
  }
}

struct Logic {
  branch: String,
  categories: Vec<String>,
  changes: HashMap<String, Vec<String>>,
  cli: CommentChanges,
  hyperlinks: HashMap<String, String>,
  repository: Option<Repository>,
  user: String,
}

impl Logic {
  fn generate(&self) -> String {
    let mut result = self.resolve();

    for (category, changes) in &self.changes {
      result.append_as_line(match self.cli.extension.as_str() {
        "md" => format!("{} {category}\n", "#".repeat(self.cli.heading.into())),
        "rst" => format!(
          "{category}\n{}\n",
          match self.cli.heading {
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

  fn get_branch(&mut self) -> Result<()> {
    if let Some(repository) = &self.repository {
      self.branch = repository.head().map_or_else(
        |error| {
          eprintln!("{error}");
          Err(ExitCode::Unavailable)
        },
        |reference| {
          reference
            .name()
            .map_or(Err(ExitCode::Unavailable), |name| Ok(name.to_string()))
        },
      )?;

      Ok(())
    } else {
      Err(ExitCode::Software)
    }
  }

  fn get_user(&mut self) -> Result<()> {
    if let Some(repository) = &self.repository {
      self.user = repository
        .config()
        .map_or_else(
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
        )?
        .replace(' ', "_");

      Ok(())
    } else {
      Err(ExitCode::Software)
    }
  }

  fn insert(
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

  fn main(&mut self) -> Result<()> {
    self.preprocess()?;
    self.query()?;
    self.report()
  }

  fn preprocess(&mut self) -> Result<()> {
    if self.cli.keep_a_changelog {
      self.categories.append(
        &mut vec![
          "Added",
          "Changed",
          "Deprecated",
          "Fixed",
          "Removed",
          "Security",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>(),
      );
    }

    self.categories.append(&mut self.cli.category.clone());

    self.hyperlinks = self
      .cli
      .link
      .iter()
      .zip(self.cli.target.iter())
      .map(|(a, b)| (a.to_string(), b.to_string()))
      .collect();

    Repository::open(".").map_or_else(
      |_| {
        eprintln!("This is not a Git repository.");
        Err(ExitCode::Usage)
      },
      |r| {
        self.repository = Some(r);
        Ok(())
      },
    )
  }

  fn query(&mut self) -> Result<()> {
    if let Some(repository) = &self.repository {
      match repository.revwalk() {
        Ok(mut revwalk) => match revwalk.push_head() {
          Ok(()) => {
            let mut count = 1;
            let mut result = HashMap::new();

            for oid in revwalk {
              if let Some(depth) = self.cli.depth {
                if count > depth {
                  break;
                }
              }

              if let Ok(oid) = oid {
                if let Ok(commit) = repository.find_commit(oid) {
                  if let Some(message) = if self.cli.body {
                    commit.body()
                  } else {
                    commit.summary()
                  } {
                    if let Some((category, change)) =
                      message.trim().split_once(&self.cli.delimiter)
                    {
                      let category = category.trim().to_string();
                      let change = change.trim().to_string();
                      let valid_category =
                        self.categories.iter().any(|c| c == &category);

                      if self.categories.is_empty() || valid_category {
                        Self::insert(&mut result, category, change);
                      } else if !valid_category {
                        if let Some(fallback) = &self.cli.fallback_category {
                          Self::insert(
                            &mut result,
                            fallback.to_string(),
                            change,
                          );
                        }
                      }
                    } else if let Some(fallback) = &self.cli.fallback_category {
                      Self::insert(
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
    } else {
      Err(ExitCode::Software)
    }
  }

  pub fn report(&mut self) -> Result<()> {
    let content = if self.cli.extension == "ron" {
      ron::ser::to_string_pretty(
        &crate::Fragment::new(&self.hyperlinks, &self.changes),
        ron::ser::PrettyConfig::default().indentor("  ".to_string()),
      )
      .map_or(Err(ExitCode::DataErr), Ok)?
    } else {
      self.generate()
    };

    self.get_branch()?;
    self.get_user()?;

    format!(
      "{}/{}_{}_{}.{}",
      self.cli.output_directory,
      chrono::Local::now().format("%Y%m%d_%H%M%S"),
      self.user,
      self.branch.split('/').last().unwrap_or("HEAD"),
      self.cli.extension
    )
    .write(Box::new(content))
  }

  fn resolve(&self) -> String {
    let mut result = String::new();

    if !self.hyperlinks.is_empty() {
      for (link_name, target) in &self.hyperlinks {
        result.append_as_line(match self.cli.extension.as_str() {
          "md" => format!("[{link_name}]:  {target}"),
          "rst" => format!(".. _{link_name}:  {target}"),
          _ => unreachable!(),
        });
      }

      result.push('\n');
    }

    result
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
