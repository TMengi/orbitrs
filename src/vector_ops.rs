pub fn vector_2norm(vec: &[f64]) -> f64 {
    let sum: f64 = vec.iter().sum();
    sum.sqrt()
}

pub fn safe_normalize(vec: &mut [f64]) {
    let norm = vector_2norm(vec);
    if norm > 0. {
        vec.iter_mut().for_each(|elem| *elem /= norm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing;

    #[test]
    fn test_vector_2norm() {
        // Array version
        let arr = [1., 1., 1.];
        let expected_norm = (3 as f64).sqrt();
        assert_eq!(vector_2norm(&arr), expected_norm);

        // Vector version
        let vec = vec![1., 1., 1.];
        assert_eq!(vector_2norm(&vec), expected_norm);
    }

    #[test]
    fn test_normalize_array() {
        // Unit array
        let arr = [1., 0., 0.];
        let mut normalized_arr = arr.clone();
        safe_normalize(&mut normalized_arr);
        testing::assert_array_eq(&normalized_arr, &arr);

        // Non-unit vector
        let vec = vec![1., 0., 1.];
        let mut normalized_arr = vec.clone();
        safe_normalize(&mut normalized_arr);
        let expected_arr = [(0.5 as f64).sqrt(), 0., (0.5 as f64).sqrt()];
        testing::assert_array_eq(&normalized_arr, &expected_arr);

        // Zero vector
        let vec = vec![0., 0., 0., 0.];
        let mut normalized_vec = vec.clone();
        safe_normalize(&mut normalized_vec);
        testing::assert_array_eq(&normalized_vec, &vec);
    }
}
