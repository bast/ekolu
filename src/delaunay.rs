pub fn delaunay_triangulate(coordinates: &[(f64, f64)]) -> Vec<(usize, usize, usize)> {
    let points: Vec<delaunator::Point> = coordinates
        .iter()
        .map(|&(x, y)| delaunator::Point { x, y })
        .collect();

    let result = delaunator::triangulate(&points);

    result
        .triangles
        .chunks(3)
        .map(|t| (t[0], t[1], t[2]))
        .collect()
}
