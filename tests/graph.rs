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

use aeruginous::{EdgeType, Edges, Graph, Vertices};

#[test]
fn edge_type_equality_directed() {
  assert_eq!(EdgeType::directed("", ""), EdgeType::directed("", ""));
  assert_eq!(EdgeType::directed("a", "a"), EdgeType::directed("a", "a"));
  assert_eq!(EdgeType::directed("a", "b"), EdgeType::directed("a", "b"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::directed("a", "c"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::directed("b", "a"));
}

#[test]
fn edge_type_equality_mixed_1() {
  assert_ne!(EdgeType::directed("", ""), EdgeType::undirected("", ""));
  assert_ne!(EdgeType::directed("a", "a"), EdgeType::undirected("a", "a"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::undirected("a", "b"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::undirected("b", "a"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::undirected("a", "c"));
  assert_ne!(EdgeType::directed("a", "b"), EdgeType::undirected("c", "d"));
}

#[test]
fn edge_type_equality_mixed_2() {
  assert_ne!(EdgeType::undirected("", ""), EdgeType::directed("", ""));
  assert_ne!(EdgeType::undirected("a", "a"), EdgeType::directed("a", "a"));
  assert_ne!(EdgeType::undirected("a", "b"), EdgeType::directed("a", "b"));
  assert_ne!(EdgeType::undirected("b", "a"), EdgeType::directed("a", "b"));
  assert_ne!(EdgeType::undirected("a", "c"), EdgeType::directed("a", "b"));
  assert_ne!(EdgeType::undirected("c", "d"), EdgeType::directed("a", "b"));
}

#[test]
fn edge_type_equality_undirected() {
  assert_eq!(EdgeType::undirected("", ""), EdgeType::undirected("", ""));
  assert_eq!(
    EdgeType::undirected("a", "a"),
    EdgeType::undirected("a", "a")
  );
  assert_eq!(
    EdgeType::undirected("a", "b"),
    EdgeType::undirected("a", "b")
  );
  assert_eq!(
    EdgeType::undirected("a", "b"),
    EdgeType::undirected("b", "a")
  );
  assert_ne!(
    EdgeType::undirected("a", "b"),
    EdgeType::undirected("a", "c")
  );
}

/******************************************************************************/
