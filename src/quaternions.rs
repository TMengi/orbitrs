use std::convert::{TryFrom, TryInto};
use std::ops;

use crate::vector::Vector3;
use crate::vector_ops;

#[derive(Clone, Debug, PartialEq)]
struct Quaternion {
    scalar: f64,
    vector: Vector3,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl Quaternion {
    fn identity() -> Self {
        Quaternion {
            scalar: 1.0,
            vector: Vector3::new([0.0, 0.0, 0.0]),
        }
    }

    fn from_angle_axis(angle: f64, axis: &[f64; 3]) -> Self {
        // TODO: Improve this API so axis can be a Vector3 as well
        let half_angle = angle / 2.0;
        let scalar = f64::cos(half_angle);

        let sin_half_angle = f64::sin(half_angle);
        let mut unit_axis = axis.clone();
        vector_ops::safe_normalize(&mut unit_axis);
        let vector = unit_axis.map(|elem| sin_half_angle * elem).into();
        Self { scalar, vector }
    }

    fn to_angle_axis(&self) -> (f64, [f64; 3]) {
        let half_angle = f64::acos(self.scalar);
        let sin_half_angle = f64::sin(half_angle);
        let axis = self.vector.elem.map(|elem| elem / sin_half_angle);
        (half_angle * 2.0, axis)
    }

    /// Returns the inverse as a new Quaternion
    fn inverted(&self) -> Self {
        let vector = self.vector.elem.map(|elem| -elem).into();
        Self {
            scalar: self.scalar,
            vector,
        }
    }

    /// Inverts the Quaternion in place
    fn invert(&mut self) {
        self.vector = self.vector.elem.map(|elem| -elem).into();
    }

    /// Rotates a vector with an alias (passive) convention.
    ///
    /// This is equivalent to representing the same vector in a new rotated frame.
    fn rotated_vec_alias(&self, vec: &Vector3) -> Vector3 {
        let vec_cross_quat = vec.cross(&self.vector);
        let term1 = vec_cross_quat.clone() * 2.0 * self.scalar;
        let term2 = vec_cross_quat.cross(&self.vector) * 2.0;
        let new_vec = vec.clone() + term1 + term2;
        new_vec
    }

    /// Rotates a vector with an alibi (active) convention.
    ///
    /// This is equivalent to actively rotating the vector in space.
    fn rotated_vec_alibi(&self, vec: &Vector3) -> Vector3 {
        let quat_cross_vec = self.vector.cross(&vec);
        let term1 = quat_cross_vec.clone() * 2.0 * self.scalar;
        let term2 = self.vector.cross(&quat_cross_vec) * 2.0;
        let new_vec = vec.clone() + term1 + term2;
        new_vec
    }
}

impl ops::Mul<Self> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let scalar = self.scalar * rhs.scalar - self.vector.dot(&rhs.vector);
        let q0 = self.scalar;
        let (q1, q2, q3) = self.vector.elem.into();
        let r0 = rhs.scalar;
        let (r1, r2, r3) = rhs.vector.elem.into();
        let vector = Vector3::new([
            r0 * q1 + r1 * q0 - r2 * q3 + r3 * q2,
            r0 * q2 + r1 * q3 + r2 * q0 - r3 * q1,
            r0 * q3 - r1 * q2 + r2 * q1 + r3 * q0,
        ]);
        Self { scalar, vector }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::f64;

    use crate::constants;
    use crate::testing;
    use crate::vector;

    #[test]
    /// The identity quaternion should have no effect when used to rotate vectors
    fn test_identity() {
        let identity = Quaternion::identity();

        let vec = Vector3::new([1.0, 2.0, 3.0]);
        let rotated_vec = identity.rotated_vec_alias(&vec);
        testing::assert_array_eq(&rotated_vec.elem, &vec.elem);

        let vec = Vector3::new([1.0, 2.0, 3.0]);
        let rotated_vec = identity.rotated_vec_alibi(&vec);
        testing::assert_array_eq(&rotated_vec.elem, &vec.elem);

        let identity_squared = identity.clone() * identity.clone();
        assert_eq!(identity_squared, identity);
    }

    #[test]
    /// Can construct a quaternion from an angle/axis representation and convert it back out
    fn test_from_angle_axis() {
        let original_angle = f64::consts::PI / 2.0;
        let original_axis = constants::X_AXIS;
        let quat = Quaternion::from_angle_axis(original_angle, &original_axis);
        let (angle, axis) = quat.to_angle_axis();
        assert_relative_eq!(angle, original_angle);
        testing::assert_array_eq(&axis, &original_axis);
    }

    #[test]
    /// Multiplying a quaternion by its inverse gives the identity quaternion
    fn test_inversion_gives_identity() {
        let q = Quaternion::from_angle_axis(3.0 * f64::consts::FRAC_PI_8, &constants::Z_AXIS);
        let q_inv = q.inverted();
        let qq_inv = q * q_inv;
        assert_eq!(qq_inv, Quaternion::identity());
    }

    #[test]
    /// Rotate a vector by a quaternion and then rotate it by the inverse quaternion, should get
    /// the original vector
    ///
    /// This version uses alias rotations
    fn test_rotate_with_inverse_is_noop_alias() {
        let mut q = Quaternion::from_angle_axis(3.0 * f64::consts::FRAC_PI_8, &constants::Z_AXIS);
        let original_vec = Vector3::new([1.0, 2.0, 3.0]);
        let rotated_vec = q.clone().rotated_vec_alias(&original_vec);
        q.invert();
        let rotated_vec = q.rotated_vec_alias(&rotated_vec);
        testing::assert_array_eq_atol(&rotated_vec.elem, &original_vec.elem, 3.0 * f64::EPSILON);
    }

    #[test]
    /// Rotate a vector by a quaternion and then rotate it by the inverse quaternion, should get
    /// the original vector
    ///
    /// This version uses alibi rotations
    fn test_rotate_with_inverse_is_noop_alibi() {
        let mut q = Quaternion::from_angle_axis(3.0 * f64::consts::FRAC_PI_8, &constants::Z_AXIS);
        let original_vec = Vector3::new([1.0, 2.0, 3.0]);
        let rotated_vec = q.clone().rotated_vec_alibi(&original_vec);
        q.invert();
        let rotated_vec = q.rotated_vec_alibi(&rotated_vec);
        testing::assert_array_eq_atol(&rotated_vec.elem, &original_vec.elem, 3.0 * f64::EPSILON);
    }

    #[test]
    /// Applying an alias rotation followed by an alibi rotation with the same quaternion gives the
    /// original vector back
    fn test_alias_alibi() {
        // Alias then alibi
        let q = Quaternion::from_angle_axis(7.0 * f64::consts::FRAC_PI_4, &constants::Y_AXIS);
        let original_vec = Vector3::new([1.0, -2.0, 3.0]);
        let rotated_vec = q.rotated_vec_alias(&q.rotated_vec_alibi(&original_vec));
        testing::assert_array_eq_atol(&rotated_vec.elem, &original_vec.elem, 2.0 * f64::EPSILON);

        // Alibi then alias
        let rotated_vec = q.rotated_vec_alibi(&q.rotated_vec_alias(&original_vec));
        testing::assert_array_eq_atol(&rotated_vec.elem, &original_vec.elem, 2.0 * f64::EPSILON);
    }
}
