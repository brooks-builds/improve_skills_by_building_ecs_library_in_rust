#[test]
fn tuples() {
    let dimensions = (100.0, 150.0);
    print_dimensions(dimensions);
}

fn print_dimensions((x, _): (f32, f32)) {
    dbg!(x);
}
