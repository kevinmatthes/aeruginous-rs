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

use crate::{FragmentExportFormat, PatternWriter, ToMd, ToRon, ToRst, ToXml};
use git2::{Oid, Repository};
use sysexits::{ExitCode, Result};

/// Create comments on the commits of a branch in this repository.
#[derive(clap::Parser, Clone)]
#[command(visible_aliases = ["changelog"])]
pub struct CommentChanges {
    /// Work with the commit messages' bodies instead of their summaries.
    #[arg(long, short)]
    body: bool,

    /// Only these categories shall be used to generate comments.
    #[arg(long, short)]
    category: Vec<String>,

    /// The delimiter to separate a category from the change description.
    #[arg(long, short)]
    delimiter: String,

    /// The count of commits to analyse, defaulting to infinity, if omitted.
    #[arg(long, short = 'n', visible_aliases = ["count"])]
    depth: Option<usize>,

    /// The target format of the resulting fragment.
    #[arg(
        default_value = "rst",
        long,
        short = 'f',
        visible_aliases = ["format"]
    )]
    extension: FragmentExportFormat,

    /// The default category to assign.
    #[arg(long, short = 'C')]
    fallback_category: Option<String>,

    /// Whether to enforce the fragment creation.
    #[arg(long, short = 'F')]
    force: bool,

    /// The heading's level in the resulting fragment.
    #[arg(
        default_value = "3",
        long,
        short = 'H',
        value_parser = clap::value_parser!(u8).range(1..=3),
        visible_aliases = ["level"]
    )]
    heading: u8,

    /// Set categories Added, Changed, Deprecated, Fixed, Removed, and Security.
    #[arg(long, short)]
    keep_a_changelog: bool,

    /// The hyperlinks to add as comments.
    #[arg(long, short, visible_aliases = ["hyperlink"])]
    link: Vec<String>,

    /// The directory to write the generated fragment to.
    #[arg(
      default_value = ".",
      long = "output",
      short,
      visible_aliases = ["dir", "directory"]
    )]
    output_directory: String,

    /// The position to stop at.
    #[arg(long, short = '@')]
    stop: Vec<String>,

    /// The hyperlinks' targets.
    #[arg(long, short)]
    target: Vec<String>,
}

impl CommentChanges {
    /// Process the input data.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn main(&self) -> Result<()> {
        self.wrap().main()
    }

    /// Create a new instance.
    #[allow(deprecated)]
    #[must_use]
    pub fn new(delimiter: String) -> Self {
        Self {
            body: false,
            category: Vec::new(),
            delimiter,
            depth: None,
            extension: FragmentExportFormat::Rst,
            fallback_category: None,
            force: false,
            heading: 3,
            keep_a_changelog: false,
            link: Vec::new(),
            output_directory: ".".to_string(),
            stop: Vec::new(),
            target: Vec::new(),
        }
    }

    fn wrap(&self) -> Logic {
        Logic {
            branch: String::new(),
            categories: Vec::new(),
            cli: self.clone(),
            fragment: crate::Fragment::default(),
            repository: None,
            stop_conditions: Vec::new(),
            user: String::new(),
        }
    }
}

struct Logic {
    branch: String,
    categories: Vec<String>,
    cli: CommentChanges,
    fragment: crate::Fragment,
    repository: Option<Repository>,
    stop_conditions: Vec<Oid>,
    user: String,
}

impl Logic {
    fn analyse_stop_condition(&mut self) -> Result<()> {
        for stop in &self.cli.stop {
            if let Some(repository) = &self.repository {
                if let Ok(target) =
                    repository.resolve_reference_from_short_name(stop)
                {
                    if let Some(oid) = target.target() {
                        self.stop_conditions.push(oid);
                    } else {
                        eprintln!("`{stop}` cannot be used as stop condition.");
                        return Err(ExitCode::Usage);
                    }
                } else {
                    let oid = git2::Oid::from_str(stop).map_or_else(
                        |_| {
                            eprintln!("`{stop}` does not seem to exist.");
                            Err(ExitCode::Usage)
                        },
                        Ok,
                    )?;

                    if repository.find_commit(oid).is_ok() {
                        self.stop_conditions.push(oid);
                    } else {
                        eprintln!("There is no such commit `{stop}`.");
                        return Err(ExitCode::Usage);
                    }
                }
            } else {
                return Err(ExitCode::Software);
            }
        }

        Ok(())
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
                        .map_or(Err(ExitCode::Unavailable), |name| {
                            Ok(name.to_string())
                        })
                },
            )?;

            Ok(())
        } else {
            Err(ExitCode::Software)
        }
    }

    fn get_user(&mut self) -> Result<()> {
        if let Some(repository) = &self.repository {
            self.user =
                repository
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

    fn harvest_message(&self, message: &str) -> Option<(String, String)> {
        if message.is_empty() {
            None
        } else if let Some((category, change)) =
            message.trim().split_once(&self.cli.delimiter)
        {
            let category = category.trim().to_string();
            let change = change.trim().to_string();
            let valid_category = self.categories.iter().any(|c| c == &category);

            if self.categories.is_empty() || valid_category {
                Some((category, change))
            } else if !valid_category {
                self.cli
                    .fallback_category
                    .as_ref()
                    .map(|fallback| (fallback.to_string(), change))
            } else {
                None
            }
        } else {
            self.cli.fallback_category.as_ref().map(|fallback| {
                (fallback.to_string(), message.trim().to_string())
            })
        }
    }

    fn main(&mut self) -> Result<()> {
        self.preprocess()?;
        self.query()?;
        self.report()
    }

    #[allow(deprecated)]
    fn preprocess(&mut self) -> Result<()> {
        if self.cli.keep_a_changelog {
            self.categories.append(
                &mut [
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
        self.fragment.reference(
            self.cli
                .link
                .iter()
                .zip(self.cli.target.iter())
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
        );

        Repository::open(".").map_or_else(
            |_| {
                eprintln!("This is not a Git repository.");
                Err(ExitCode::Usage)
            },
            |r| {
                self.repository = Some(r);
                self.analyse_stop_condition()
            },
        )
    }

    #[allow(deprecated)]
    fn query(&mut self) -> Result<()> {
        if let Some(repository) = &self.repository {
            match repository.revwalk() {
                Ok(mut revwalk) => match revwalk.push_head() {
                    Ok(()) => {
                        let mut count = 1;

                        for oid in revwalk {
                            if let Some(depth) = self.cli.depth {
                                if count > depth {
                                    break;
                                }
                            }

                            if let Ok(oid) = oid {
                                if self.stop_conditions.contains(&oid) {
                                    break;
                                }

                                if let Ok(commit) = repository.find_commit(oid)
                                {
                                    if let Some(message) = commit.message() {
                                        let (summary, body) = message
                                            .split_once('\n')
                                            .unwrap_or((message, ""));

                                        if let Some((category, change)) = self
                                            .harvest_message(if self.cli.body {
                                                body.trim()
                                            } else {
                                                summary.trim()
                                            })
                                        {
                                            self.fragment
                                                .insert(&category, &change);
                                        } else if self.cli.force {
                                            if let Some((category, change)) =
                                                self.harvest_message(
                                                    if self.cli.body {
                                                        summary.trim()
                                                    } else {
                                                        body.trim()
                                                    },
                                                )
                                            {
                                                self.fragment
                                                    .insert(&category, &change);
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!(
                                        "Commit {oid} does not seem to exist."
                                    );
                                    return Err(ExitCode::DataErr);
                                }
                            } else {
                                eprintln!(
                                    "Too few commits were fetched on checkout."
                                );
                                return Err(ExitCode::Usage);
                            }

                            count += 1;
                        }

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

    fn report(&mut self) -> Result<()> {
        self.fragment.sort();

        let content = match self.cli.extension {
            FragmentExportFormat::Md => self.fragment.to_md(self.cli.heading),
            FragmentExportFormat::Ron => self.fragment.to_ron(2),
            FragmentExportFormat::Rst => self.fragment.to_rst(self.cli.heading),
            FragmentExportFormat::Xml => self.fragment.to_xml(),
        }?;

        if !std::path::Path::new(&self.cli.output_directory).try_exists()? {
            std::fs::create_dir_all(&self.cli.output_directory)?;
        }

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
        .append(Box::new(content))
    }
}

/******************************************************************************/
