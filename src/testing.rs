pub fn assert_array_eq(a: &[f64], b: &[f64]) {
    for (elem_a, elem_b) in a.into_iter().zip(b.into_iter()) {
        assert_relative_eq!(elem_a, elem_b);
    }
}
