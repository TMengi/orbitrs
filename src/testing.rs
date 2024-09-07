pub fn assert_array_eq(a: &[f64], b: &[f64]) {
    let success = a
        .iter()
        .zip(b.iter())
        .map(|(elem_a, elem_b)| relative_eq!(elem_a, elem_b))
        .reduce(|acc, e| acc && e)
        .unwrap_or(false);
    if !success {
        panic!("\tleft  = {:?}\n\tright = {:?}", a, b);
    }
}
