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

use crate::getters;
use std::{
  collections::{HashMap, HashSet},
  ops::{Add, AddAssign, MulAssign},
};

/// The possible edge types.
#[derive(Debug)]
pub enum EdgeType {
  /// A directed edge from a departure to an arrival.
  DirectedEdge {
    /// The vertex where this edge begins.
    departure: String,

    /// The vertex where this edge ends.
    arrival: String,
  },

  /// An undirected edge.
  UndirectedEdge {
    /// One vertex connected by this edge.
    one: String,

    /// The other vertex connected by this edge.
    two: String,
  },
}

impl EdgeType {
  /// Create a new directed edge.
  #[must_use]
  pub fn directed(departure: &str, arrival: &str) -> Self {
    Self::DirectedEdge {
      departure: departure.to_string(),
      arrival: arrival.to_string(),
    }
  }

  /// Create a new undirected edge.
  #[must_use]
  pub fn undirected(one: &str, two: &str) -> Self {
    Self::UndirectedEdge {
      one: one.to_string(),
      two: two.to_string(),
    }
  }
}

impl Eq for EdgeType {}

impl PartialEq for EdgeType {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Self::DirectedEdge {
        departure: a,
        arrival: b,
      } => match other {
        Self::DirectedEdge {
          departure: c,
          arrival: d,
        } => a == c && b == d,
        Self::UndirectedEdge { .. } => false,
      },
      Self::UndirectedEdge { one: a, two: b } => match other {
        Self::DirectedEdge { .. } => false,
        Self::UndirectedEdge { one: c, two: d } => {
          (a == c && b == d) || (a == d && b == c)
        }
      },
    }
  }
}

impl std::hash::Hash for EdgeType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Self::DirectedEdge { departure, arrival } => {
        departure.hash(state);
        arrival.hash(state);
      }
      Self::UndirectedEdge { one, two } => {
        one.hash(state);
        two.hash(state);
      }
    }
  }
}

/// A set of edges.
#[derive(Debug)]
pub struct Edges {
  /// The held edges.
  edges: HashSet<EdgeType>,
}

impl Edges {
  /// Add a new, directed edge.
  pub fn add_directed_edge(&mut self, departure: &str, arrival: &str) -> bool {
    self.edges.insert(EdgeType::directed(departure, arrival))
  }

  /// Add a new, undirected edge.
  pub fn add_undirected_edge(&mut self, one: &str, two: &str) -> bool {
    self.edges.insert(EdgeType::undirected(one, two))
  }

  /// Check whether there is a certain edge.
  #[must_use]
  pub fn contains(&self, edge: &EdgeType) -> bool {
    match edge {
      EdgeType::DirectedEdge { departure, arrival } => {
        self.edges.contains(&EdgeType::directed(departure, arrival))
      }
      EdgeType::UndirectedEdge { one, two } => {
        self.edges.contains(&EdgeType::undirected(one, two))
          || self.edges.contains(&EdgeType::undirected(two, one))
      }
    }
  }

  /// Create a new instance.
  #[must_use]
  pub fn new() -> Self {
    Self {
      edges: HashSet::new(),
    }
  }
}

impl Eq for Edges {}

impl Default for Edges {
  fn default() -> Self {
    Self::new()
  }
}

impl PartialEq for Edges {
  fn eq(&self, other: &Self) -> bool {
    let mut result = self.edges.len() == other.edges.len();

    if result {
      for edge in &self.edges {
        if !other.contains(edge) {
          result = false;
          break;
        }
      }
    }

    result
  }
}

/// A graph.
#[derive(Eq, PartialEq)]
pub struct Graph<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  /// The held edges.
  edges: Edges,

  /// The held vertices.
  vertices: Vertices<T>,
}

impl<T> Graph<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  getters!(@fn @ref edges: Edges, vertices: Vertices<T>);

  /// Connect the vertices `a` and `b` with an undirected edge.
  pub fn connect_a_and_b(&mut self, a: &str, b: &str) {
    self.vertices.add_undirected_edge(a, b);
    self.edges.add_undirected_edge(a, b);
  }

  /// Connect the vertices `a` and `b` with a directed edge from `a` to `b`.
  pub fn connect_a_with_b(&mut self, a: &str, b: &str) {
    self.vertices.add_directed_edge(a, b);
    self.edges.add_directed_edge(a, b);
  }

  /// Declare a new vertex.
  pub fn declare(&mut self, vertex: &str) {
    self.vertices.add_vertex(vertex);
  }

  /// Create a new instance.
  #[must_use]
  pub fn new() -> Self {
    Self {
      edges: Edges::default(),
      vertices: Vertices::default(),
    }
  }
}

impl<T> Default for Graph<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  fn default() -> Self {
    Self::new()
  }
}

/// The metadata of a vertex.
#[derive(Clone, Debug)]
pub struct VertexData<T>
where
  T: Add + AddAssign + Clone + MulAssign,
{
  /// The sum of in- and outgoing edges of this vertex.
  degree: usize,

  /// The count of ingoing edges.
  ingoing: usize,

  /// The count of outgoing edges.
  outgoing: usize,

  /// The x coordinate of this vertex.
  x: T,

  /// The y coordinate of this vertex.
  y: T,

  /// The z coordinate of this vertex.
  z: T,
}

impl<T> VertexData<T>
where
  T: Add + AddAssign + Clone + MulAssign,
{
  getters!(@fn @cp degree: usize, ingoing: usize, outgoing: usize);
  getters!(@fn @ref x: T, y: T, z: T);

  /// An undirected edge connects this vertex with another one.
  pub fn connect_edge(&mut self) {
    self.degree += 1;
    self.ingoing += 1;
    self.outgoing += 1;
  }

  /// Move this instance by certain values.
  pub fn move_by(&mut self, x: T, y: T, z: T) {
    self.x += x;
    self.y += y;
    self.z += z;
  }

  /// Move this instance to a certain point.
  pub fn move_to(&mut self, x: T, y: T, z: T) {
    self.x = x;
    self.y = y;
    self.z = z;
  }

  /// Create a new instance.
  #[must_use]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self {
      degree: 0,
      ingoing: 0,
      outgoing: 0,
      x,
      y,
      z,
    }
  }

  /// An edge arrives at this vertex.
  pub fn receive_edge(&mut self) {
    self.degree += 1;
    self.ingoing += 1;
  }

  /// Scale this vertex by a certain factor.
  pub fn scale(&mut self, n: T) {
    self.x *= n.clone();
    self.y *= n.clone();
    self.z *= n;
  }

  /// An edge departs from this vertex.
  pub fn send_edge(&mut self) {
    self.degree += 1;
    self.outgoing += 1;
  }
}

impl<T> Eq for VertexData<T> where
  T: Add + AddAssign + Clone + MulAssign + PartialEq
{
}

impl<T> Default for VertexData<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  fn default() -> Self {
    Self::new(0.into(), 0.into(), 0.into())
  }
}

impl<T> PartialEq for VertexData<T>
where
  T: Add + AddAssign + Clone + MulAssign + PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
}

/// A set of vertices.
#[derive(Debug)]
pub struct Vertices<T>
where
  T: Add + AddAssign + Clone + MulAssign,
{
  /// The held vertices.
  vertices: HashMap<String, VertexData<T>>,
}

impl<T> Vertices<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  /// Update these vertices due to a directed edge.
  pub fn add_directed_edge(&mut self, departure: &str, arrival: &str) {
    self.add_vertex(departure);
    self.add_vertex(arrival);

    let mut departure_vertex = self.vertices[departure].clone();
    departure_vertex.send_edge();
    self
      .vertices
      .insert(departure.to_string(), departure_vertex);

    let mut arrival_vertex = self.vertices[arrival].clone();
    arrival_vertex.receive_edge();
    self.vertices.insert(arrival.to_string(), arrival_vertex);
  }

  /// Update these vertices due to an undirected edge.
  pub fn add_undirected_edge(&mut self, a: &str, b: &str) {
    self.add_vertex(a);
    self.add_vertex(b);

    let mut a_vertex = self.vertices[a].clone();
    a_vertex.connect_edge();
    self.vertices.insert(a.to_string(), a_vertex);

    let mut b_vertex = self.vertices[b].clone();
    b_vertex.connect_edge();
    self.vertices.insert(b.to_string(), b_vertex);
  }

  /// Define a new vertex.
  pub fn add_vertex(&mut self, label: &str) -> bool {
    if self.vertices.get(label).is_some() {
      false
    } else {
      self
        .vertices
        .insert(label.to_string(), VertexData::default());
      true
    }
  }

  /// Move all vertices by certain values.
  pub fn move_by(&mut self, x: &T, y: &T, z: &T) {
    let mut update = HashMap::new();

    for (label, vertex) in &self.vertices {
      let mut vertex = vertex.clone();
      vertex.move_by(x.clone(), y.clone(), z.clone());
      update.insert(label.to_string(), vertex);
    }

    self.vertices = update;
  }

  /// Create a new instance.
  #[must_use]
  pub fn new() -> Self {
    Self {
      vertices: HashMap::new(),
    }
  }

  /// Scale all vertices by a certain factor.
  pub fn scale(&mut self, n: &T) {
    let mut update = HashMap::new();

    for (label, vertex) in &self.vertices {
      let mut vertex = vertex.clone();
      vertex.scale(n.clone());
      update.insert(label.to_string(), vertex);
    }

    self.vertices = update;
  }
}

impl<T> Eq for Vertices<T> where
  T: Add + AddAssign + Clone + From<u8> + MulAssign + PartialEq
{
}

impl<T> Default for Vertices<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T> PartialEq for Vertices<T>
where
  T: Add + AddAssign + Clone + From<u8> + MulAssign + PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    let mut result = self.vertices.len() == other.vertices.len();

    if result {
      for (label, vertex) in &self.vertices {
        if other.vertices.get(label).is_none() {
          result = false;
          break;
        }

        if &other.vertices[label] != vertex {
          result = false;
          break;
        }
      }
    }

    result
  }
}

/******************************************************************************/
