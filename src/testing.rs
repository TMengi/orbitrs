use std::f64;

// TODO: Replace this with a macro so we don't need multiple versions
pub fn assert_array_eq(a: &[f64], b: &[f64]) {
    assert_array_eq_atol(a, b, f64::EPSILON);
}

pub fn assert_array_eq_atol(a: &[f64], b: &[f64], atol: f64) {
    let success = a
        .iter()
        .zip(b.iter())
        .map(|(elem_a, elem_b)| relative_eq!(elem_a, elem_b, epsilon = atol))
        .reduce(|acc, e| acc && e)
        .unwrap_or(false);
    if !success {
        let max_diff: f64 = a
            .iter()
            .zip(b.iter())
            .map(|(elem_a, elem_b)| f64::abs(elem_a - elem_b))
            .fold(0.0, f64::max);

        panic!(
            "\tleft  = {:?}\n\tright = {:?}\n\tmax adiff = {:.3e}\n\ttol = {:.3e}",
            a, b, max_diff, atol
        );
    }
}
