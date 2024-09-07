use crate::vector_ops;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub elem: [f64; 3],
}

impl Vector3 {
    pub fn new(elem: [f64; 3]) -> Self {
        Self { elem }
    }

    pub fn norm(&self) -> f64 {
        vector_ops::vector_2norm(&self.elem)
    }

    pub fn safe_normalize(&mut self) {
        vector_ops::safe_normalize(&mut self.elem)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        vector_ops::vector_dot(&self.elem, &other.elem)
    }

    pub fn cross(&self, other: &Self) -> Self {
        let new_elem = [
            self.elem[1] * other.elem[2] - self.elem[2] * other.elem[1],
            self.elem[2] * other.elem[0] - self.elem[0] * other.elem[2],
            self.elem[0] * other.elem[1] - self.elem[1] * other.elem[0],
        ];
        Self { elem: new_elem }
    }
}

impl TryFrom<Vec<f64>> for Vector3 {
    type Error = String;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        match value.try_into() {
            Ok(elem) => Ok(Self { elem }),
            Err(val) => Err(format!("Expected 3 elements, found {}", val.len())),
        }
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(value: [f64; 3]) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing;

    #[test]
    fn test_from_iterator() {
        Vector3::try_from(vec![1.0, 2.0, -3.0]).unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected 3 elements")]
    fn test_from_iterator_wrong_len() {
        Vector3::try_from(vec![1.0, 2.0]).unwrap();
    }

    #[test]
    fn test_norm() {
        let vec = Vector3::new([1.0, 2.0, -3.0]);
        let expected_norm = (14.0 as f64).sqrt();
        assert_relative_eq!(vec.norm(), expected_norm);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3::new([1.0, -2.0, 3.0]);
        let v2 = Vector3::new([4.0, 5.0, 6.0]);
        let dot_product = v1.dot(&v2);
        assert_relative_eq!(dot_product, 12.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        let v2 = Vector3::new([4.0, 5.0, 6.0]);
        let expected_product = Vector3::new([-3.0, 6.0, -3.0]);
        let cross_product = v1.cross(&v2);
        testing::assert_array_eq(&cross_product.elem, &expected_product.elem);
    }
}
