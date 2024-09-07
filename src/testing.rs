pub fn assert_array_eq(a: &[f64], b: &[f64]) {
    let mut success = true;
    for (elem_a, elem_b) in a.iter().zip(b.iter()) {
        success &= relative_eq!(elem_a, elem_b);
    }
    if !success {
        // let msg = format!("{:?}, {:?}", a, b);
        // panic!(msg)
        panic!("\tleft  = {:?}\n\tright = {:?}", a, b);
    }
}
