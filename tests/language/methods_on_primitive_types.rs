#[test]
fn methods_on_primitive_types() {
    let number = 10.0_f32;
    let larger_number = number.powf(2.5);
    dbg!(larger_number);
    let pi = std::f64::consts::PI;
    dbg!(pi);
}
