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

use aeruginous::{EdgeType, Edges, Graph, VertexData, Vertices};

#[test]
fn edge_type_debug_trait() {
    assert_eq!(
        format!("{:?}", EdgeType::directed("a", "b")),
        "DirectedEdge { departure: \"a\", arrival: \"b\" }"
    );
}

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

#[test]
fn edges_add_directed_edges() {
    let mut e = Edges::default();

    e.add_directed_edge("a", "b");
    e.add_directed_edge("b", "c");
    e.add_directed_edge("c", "a");

    assert!(e.contains(&EdgeType::directed("a", "b")));
    assert!(e.contains(&EdgeType::directed("b", "c")));
    assert!(e.contains(&EdgeType::directed("c", "a")));

    assert!(!e.contains(&EdgeType::directed("b", "a")));
    assert!(!e.contains(&EdgeType::directed("c", "b")));
    assert!(!e.contains(&EdgeType::directed("a", "c")));
}

#[test]
fn edges_add_undirected_edges() {
    let mut e = Edges::default();

    e.add_undirected_edge("a", "b");
    e.add_undirected_edge("b", "c");
    e.add_undirected_edge("c", "a");

    assert!(e.contains(&EdgeType::undirected("a", "b")));
    assert!(e.contains(&EdgeType::undirected("b", "c")));
    assert!(e.contains(&EdgeType::undirected("c", "a")));

    assert!(e.contains(&EdgeType::undirected("b", "a")));
    assert!(e.contains(&EdgeType::undirected("c", "b")));
    assert!(e.contains(&EdgeType::undirected("a", "c")));
}

#[test]
fn edges_debug_trait() {
    assert_eq!(format!("{:?}", Edges::default()), "Edges { edges: {} }");
}

#[test]
fn edges_equality_1() {
    let mut edges_a = Edges::default();
    let mut edges_b = Edges::default();

    edges_a.add_undirected_edge("a", "b");
    edges_b.add_undirected_edge("a", "b");

    assert_eq!(edges_a, edges_b);
    assert_eq!(edges_b, edges_a);

    edges_b.add_undirected_edge("c", "d");

    assert_ne!(edges_a, edges_b);
    assert_ne!(edges_b, edges_a);
}

#[test]
fn edges_equality_2() {
    let mut edges_a = Edges::default();
    let mut edges_b = Edges::default();

    edges_a.add_undirected_edge("a", "b");
    edges_b.add_undirected_edge("b", "a");

    assert_eq!(edges_a, edges_b);
    assert_eq!(edges_b, edges_a);

    edges_b.add_undirected_edge("c", "d");

    assert_ne!(edges_a, edges_b);
    assert_ne!(edges_b, edges_a);
}

#[test]
fn edges_method_equality() {
    assert_eq!(Edges::new(), Edges::default());
}

#[test]
fn edges_unequality() {
    let mut edges_a = Edges::default();
    let mut edges_b = Edges::default();

    edges_a.add_directed_edge("", "");
    edges_b.add_undirected_edge("", "");

    assert_ne!(edges_a, edges_b);
    assert_ne!(edges_b, edges_a);
}

#[test]
fn graph_connect_a_and_b_1() {
    let mut e = Edges::default();
    let mut g = Graph::<i32>::default();

    e.add_undirected_edge("a", "b");
    g.connect_a_and_b("a", "b");

    assert_eq!(g.edges(), &e);
    assert_eq!(&e, g.edges());
}

#[test]
fn graph_connect_a_and_b_2() {
    let mut e = Edges::default();
    let mut g = Graph::<i32>::default();

    e.add_undirected_edge("a", "b");
    g.connect_a_and_b("b", "a");

    assert_eq!(g.edges(), &e);
    assert_eq!(&e, g.edges());
}

#[test]
fn graph_connect_a_with_b() {
    let mut e = Edges::default();
    let mut g = Graph::<i32>::default();

    e.add_directed_edge("a", "b");
    g.connect_a_with_b("a", "b");

    assert_eq!(g.edges(), &e);
    assert_eq!(&e, g.edges());
}

#[test]
fn graph_declare() {
    let mut g = Graph::<i32>::default();
    let mut v = Vertices::default();

    g.declare("a");
    v.add_vertex("a");

    assert_eq!(g.vertices(), &v);
    assert_eq!(&v, g.vertices());
}

#[test]
fn graph_method_equality() {
    assert_eq!(Graph::<i32>::new(), Graph::default());
}

#[test]
fn vertex_data_connect_edge() {
    let mut vertex = VertexData::<i32>::default();
    vertex.connect_edge();

    assert_eq!(vertex, VertexData::default());
    assert_eq!(vertex.degree(), 1);
    assert_eq!(vertex.ingoing(), 1);
    assert_eq!(vertex.outgoing(), 1);
}

#[test]
fn vertex_data_getters() {
    let vertex = VertexData::new(128, 256, 512);

    assert_eq!(vertex.x(), &128);
    assert_eq!(vertex.y(), &256);
    assert_eq!(vertex.z(), &512);
}

#[test]
fn vertex_data_method_equality() {
    assert_eq!(VertexData::new(0, 0, 0), VertexData::default());
}

#[test]
fn vertex_data_move_by() {
    let mut vertex = VertexData::new(128, 256, 512);
    vertex.move_by(512, 256, 128);

    assert_eq!(vertex, VertexData::new(640, 512, 640));
}

#[test]
fn vertex_data_move_to() {
    let mut vertex = VertexData::new(128, 256, 512);
    vertex.move_to(0, 0, 0);

    assert_eq!(vertex, VertexData::default());
}

#[test]
fn vertex_data_receive_edge() {
    let mut vertex = VertexData::<i32>::default();
    vertex.receive_edge();

    assert_eq!(vertex, VertexData::default());
    assert_eq!(vertex.degree(), 1);
    assert_eq!(vertex.ingoing(), 1);
    assert_eq!(vertex.outgoing(), 0);
}

#[test]
fn vertex_data_scale() {
    let mut vertex = VertexData::new(128, 256, 512);
    vertex.scale(2);

    assert_eq!(vertex, VertexData::new(256, 512, 1024));
}

#[test]
fn vertex_data_send_edge() {
    let mut vertex = VertexData::<i32>::default();
    vertex.send_edge();

    assert_eq!(vertex, VertexData::default());
    assert_eq!(vertex.degree(), 1);
    assert_eq!(vertex.ingoing(), 0);
    assert_eq!(vertex.outgoing(), 1);
}

#[test]
fn vertices_add_vertex() {
    let mut v = Vertices::<i32>::default();

    assert!(v.add_vertex("a"));
    assert!(!v.add_vertex("a"));
}

#[test]
fn vertices_debug_trait() {
    assert_eq!(
        format!("{:?}", Vertices::<i32>::default()),
        "Vertices { vertices: {} }"
    );
}

#[test]
fn vertices_equality() {
    let mut vertices_1 = Vertices::<i32>::default();
    let mut vertices_2 = Vertices::<i32>::default();

    vertices_1.add_vertex("a");
    vertices_2.add_vertex("a");

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);

    vertices_2.add_vertex("b");

    assert_ne!(vertices_1, vertices_2);
    assert_ne!(vertices_2, vertices_1);
}

#[test]
fn vertices_method_equality() {
    assert_eq!(Vertices::<i32>::new(), Vertices::default());
}

#[test]
fn vertices_move_by() {
    let mut vertices_1 = Vertices::default();
    let mut vertices_2 = Vertices::default();

    vertices_1.add_vertex("a");
    vertices_2.add_vertex("a");

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);

    vertices_2.move_by(&128, &256, &512);

    assert_ne!(vertices_1, vertices_2);
    assert_ne!(vertices_2, vertices_1);
}

#[test]
fn vertices_scale_1() {
    let mut vertices_1 = Vertices::default();
    let mut vertices_2 = Vertices::default();

    vertices_1.add_vertex("a");
    vertices_2.add_vertex("a");

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);

    vertices_2.scale(&2);

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);
}

#[test]
fn vertices_scale_2() {
    let mut vertices_1 = Vertices::default();
    let mut vertices_2 = Vertices::default();

    vertices_1.add_vertex("a");
    vertices_2.add_vertex("a");

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);

    vertices_1.move_by(&128, &256, &512);
    vertices_2.move_by(&256, &512, &1024);

    assert_ne!(vertices_1, vertices_2);
    assert_ne!(vertices_2, vertices_1);

    vertices_1.scale(&2);

    assert_eq!(vertices_1, vertices_2);
    assert_eq!(vertices_2, vertices_1);
}

#[test]
fn vertices_unequality() {
    let mut vertices_1 = Vertices::<i32>::default();
    let mut vertices_2 = Vertices::<i32>::default();

    vertices_1.add_vertex("a");
    vertices_2.add_vertex("b");

    assert_ne!(vertices_1, vertices_2);
    assert_ne!(vertices_2, vertices_1);
}

/******************************************************************************/
