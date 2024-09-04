use approx::relative_eq;

pub fn assert_array_eq(a: &[f64], b: &[f64]) {
    for (elem_a, elem_b) in a.iter().zip(b.iter()) {
        assert!(relative_eq!(elem_a, elem_b));
    }
}
