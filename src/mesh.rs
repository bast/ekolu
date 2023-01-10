use std::collections::{HashMap, HashSet};

#[derive(Default, Clone)]
pub struct Mesh {
    pub coordinates: Vec<(f64, f64)>,
    pub triangles: HashSet<(usize, usize, usize)>,
    pub edge_to_opposite_vertices: HashMap<(usize, usize), HashSet<usize>>,
    pub vertex_to_triangles: HashMap<usize, HashSet<(usize, usize, usize)>>,
}
