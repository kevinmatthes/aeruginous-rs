######################## GNU General Public License 3.0 ########################
##                                                                            ##
## Copyright (C) 2023 Kevin Matthes                                           ##
##                                                                            ##
## This program is free software: you can redistribute it and/or modify       ##
## it under the terms of the GNU General Public License as published by       ##
## the Free Software Foundation, either version 3 of the License, or          ##
## (at your option) any later version.                                        ##
##                                                                            ##
## This program is distributed in the hope that it will be useful,            ##
## but WITHOUT ANY WARRANTY; without even the implied warranty of             ##
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the              ##
## GNU General Public License for more details.                               ##
##                                                                            ##
## You should have received a copy of the GNU General Public License          ##
## along with this program.  If not, see <https://www.gnu.org/licenses/>.     ##
##                                                                            ##
################################################################################

################################################################################
##
##  AUTHOR      Kevin Matthes
##  BRIEF       The recipes in order to test this project.
##  COPYRIGHT   GPL-3.0
##  DATE        2023
##  FILE        .justfile
##  NOTE        See `LICENSE' for full license.
##              See `README.md' for project details.
##
################################################################################

# The default recipe to execute.
default: ci

# Run all CI steps.
ci:
    cargo fmt --check
    cargo r -- complain src/ tests/
    just feature cff
    just feature cff-create
    just feature mkcws
    just feature rs2md
    just feature uncrlf
    just feature utilities
    just features

# Clean the repository.
clean:
    git clean -dfx

# Determine the coverage.
coverage: llvm-cov
    pycobertura show cobertura.xml

# Run all feature dependent CI steps for a single feature configuration.
feature f:
    cargo build --no-default-features -F {{ f }}
    cargo check --no-default-features -F {{ f }}
    cargo clippy --no-default-features -F {{ f }}
    cargo doc --no-default-features -F {{ f }}
    cargo test --no-default-features -F {{ f }}

# Run all feature dependent CI steps for the given, fixed feature set.
feature-set s:
    cargo build {{ s }}
    cargo check {{ s }}
    cargo clippy {{ s }}
    cargo doc {{ s }}
    cargo test {{ s }}

# Run all feature dependent CI steps for all fixed feature sets.
features:
    just feature-set \;
    just feature-set --all-features
    just feature-set --no-default-features

# Run the coverage tool.
llvm-cov: clean
    cargo llvm-cov --cobertura --output-path cobertura.xml

# Render the project logo.
logo:
    asy .github/aeruginous.asy

################################################################################
