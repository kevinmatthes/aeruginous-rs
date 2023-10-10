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

use crate::{FragmentExportFormat, PatternWriter, ToMd, ToRon, ToRst};
use git2::{Oid, Repository};
use indexmap::IndexMap;
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

    /// The commit to stop at.
    #[arg(long, short = 'S')]
    stop_at: Option<Oid>,

    /// The tag to stop at.
    #[arg(long, short = 'T')]
    tag: Option<String>,

    /// The hyperlinks' targets.
    #[arg(long, short)]
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
            extension: FragmentExportFormat::Rst,
            fallback_category: None,
            force: false,
            heading: 3,
            keep_a_changelog: false,
            link: Vec::new(),
            output_directory: ".".to_string(),
            stop_at: None,
            tag: None,
            target: Vec::new(),
        }
    }

    fn wrap(&self) -> Logic {
        Logic {
            branch: String::new(),
            categories: Vec::new(),
            changes: IndexMap::new(),
            cli: self.clone(),
            hyperlinks: IndexMap::new(),
            repository: None,
            stop_at_tag_oid: None,
            user: String::new(),
        }
    }
}

struct Logic {
    branch: String,
    categories: Vec<String>,
    changes: IndexMap<String, Vec<String>>,
    cli: CommentChanges,
    hyperlinks: crate::RonlogReferences,
    repository: Option<Repository>,
    stop_at_tag_oid: Option<Oid>,
    user: String,
}

impl Logic {
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

    fn insert(
        map: &mut IndexMap<String, Vec<String>>,
        category: String,
        change: String,
    ) {
        map.entry(category.clone()).or_default();
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

                if let Some(tag) = &self.cli.tag {
                    if let Some(repository) = &self.repository {
                        if let Ok(target) =
                            repository.resolve_reference_from_short_name(tag)
                        {
                            if target.is_tag() {
                                self.stop_at_tag_oid = target.target();

                                if self.stop_at_tag_oid.is_some() {
                                    Ok(())
                                } else {
                                    eprintln!(
                                        "There is a problem with tag {tag}."
                                    );
                                    Err(ExitCode::DataErr)
                                }
                            } else {
                                eprintln!("{tag} does not seem to be a tag.");
                                Err(ExitCode::Usage)
                            }
                        } else {
                            eprintln!("Tag {tag} does not seem to exist.");
                            Err(ExitCode::Usage)
                        }
                    } else {
                        Err(ExitCode::Software)
                    }
                } else {
                    Ok(())
                }
            },
        )
    }

    fn query(&mut self) -> Result<()> {
        if let Some(repository) = &self.repository {
            match repository.revwalk() {
                Ok(mut revwalk) => match revwalk.push_head() {
                    Ok(()) => {
                        let mut count = 1;
                        let mut result = IndexMap::new();

                        for oid in revwalk {
                            if let Some(depth) = self.cli.depth {
                                if count > depth {
                                    break;
                                }
                            }

                            if let Ok(oid) = oid {
                                if let Some(stop_at) = self.cli.stop_at {
                                    if stop_at == oid {
                                        break;
                                    }
                                } else if let Some(stop_at) =
                                    self.stop_at_tag_oid
                                {
                                    if stop_at == oid {
                                        break;
                                    }
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
                                            Self::insert(
                                                &mut result,
                                                category,
                                                change,
                                            );
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
                                                Self::insert(
                                                    &mut result,
                                                    category,
                                                    change,
                                                );
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

    fn report(&mut self) -> Result<()> {
        let mut fragment =
            crate::Fragment::new(&self.hyperlinks, &self.changes);
        fragment.sort();

        let content = match self.cli.extension {
            FragmentExportFormat::Md => fragment.to_md(self.cli.heading),
            FragmentExportFormat::Ron => fragment.to_ron(2),
            FragmentExportFormat::Rst => fragment.to_rst(self.cli.heading),
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
