use std::collections::HashSet;

use crate::delaunay;
use crate::mesh::Mesh;
use crate::order;

fn triangulate(mut mesh: Mesh) -> Mesh {
    for (a, b, c) in delaunay::delaunay_triangulate(&mesh.coordinates) {
        add_triangle(&mut mesh, a, b, c);
    }

    mesh
}

fn add_triangle(mesh: &mut Mesh, a: usize, b: usize, c: usize) {
    let abc = order::order_triple(a, b, c);

    mesh.triangles.insert(abc);

    for ((i, j), k) in [((a, b), c), ((b, c), a), ((c, a), b)] {
        mesh.edge_to_opposite_vertices
            .entry(order::order_tuple(i, j))
            .or_insert_with(HashSet::new)
            .insert(k);
    }

    for i in [a, b, c] {
        mesh.vertex_to_triangles
            .entry(i)
            .or_insert_with(HashSet::new)
            .insert(abc);
    }
}
