pub fn order_tuple(i: usize, j: usize) -> (usize, usize) {
    if i <= j {
        (i, j)
    } else {
        (j, i)
    }
}

pub fn order_triple(i: usize, j: usize, k: usize) -> (usize, usize, usize) {
    let (a, b, c) = (i, j, k);

    let (a, b) = order_tuple(a, b);
    let (b, c) = order_tuple(b, c);
    let (a, b) = order_tuple(a, b);

    (a, b, c)
}
