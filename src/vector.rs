use std::convert::TryFrom;
use std::ops;

use crate::vector_ops;

// TODO: How do we make a custom assert_eq behavior?

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

// TODO: How do I make this not consume the original?
impl ops::Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let res: Vec<f64> = self
            .elem
            .iter()
            .zip(rhs.elem.iter())
            .map(|(elem_a, elem_b)| elem_a + elem_b)
            .collect();
        Self::try_from(res).unwrap()
    }
}

// TODO: How do I make this not consume the original?
impl<T> ops::Mul<T> for Vector3
where
    f64: ops::MulAssign<T>,
    T: Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut output = self.clone();
        output.elem.iter_mut().for_each(|elem| *elem *= rhs);
        output
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
        let cross_product = v1.cross(&v2);
        testing::assert_array_eq(&cross_product.elem, &[-3.0, 6.0, -3.0]);
    }

    #[test]
    fn test_add_vectors() {
        let v1 = Vector3::new([1.0, -2.0, -3.0]);
        let v2 = Vector3::new([4.0, 5.0, 6.0]);
        let res = v1 + v2;
        testing::assert_array_eq(&res.elem, &[5.0, 3.0, 3.0]);
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vector3::new([10.0, 20.0, 5.0]);
        let res = v * 2.0;
        testing::assert_array_eq(&res.elem, &[20.0, 40.0, 10.0]);

    }
}
