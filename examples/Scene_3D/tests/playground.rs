#[test]
fn negative_sqrt() {
    let a = -4_f32;
    let b = a.sqrt();
    let c = a + b;
    println!("{c}");
}

#[test]
fn ord_float() {
    let vector: Vec<f32> = vec![0., 1., 2., f32::NAN, 3.];

    let v_max = vector.iter().max_by(|x, y| x.total_cmp(y));

    match v_max {
        Some(max_value) => println!("max value: {max_value}"),
        None => println!("none"),
    }
}