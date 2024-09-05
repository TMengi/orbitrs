use std::f64;

const TWO_PI: f64 = 2.0 * f64::consts::PI;

pub fn wrap_0_2pi(angle: f64) -> f64 {
    let mut wrapped_angle = angle % TWO_PI;
    if wrapped_angle < 0.0 {
        wrapped_angle += TWO_PI;
    }
    wrapped_angle
}

#[cfg(test)]
mod wrap_0_2pi_tests {
    use super::*;

    #[test]
    /// Angles already on the domain [0, 2pi] are unaffected
    fn test_no_wrapping() {
        let angle = f64::consts::PI;
        let wrapped_angle = wrap_0_2pi(angle);
        assert_relative_eq!(wrapped_angle, angle);
    }

    #[test]
    /// Positive angles greater than 2pi get wrapped
    fn test_positive_wrapping() {
        let expected_angle = 3.0 * f64::consts::FRAC_PI_2;
        let angle = expected_angle + 3.0 * TWO_PI;
        let wrapped_angle = wrap_0_2pi(angle);
        assert_relative_eq!(wrapped_angle, expected_angle, epsilon = 10.0 * f64::EPSILON);
    }

    #[test]
    /// Negative angles are wrapped to positive
    fn test_negative_wrapping() {
        let expected_angle = 11.0 * f64::consts::FRAC_PI_6;
        let angle = expected_angle - 2.0 * TWO_PI;
        let wrapped_angle = wrap_0_2pi(angle);
        assert_relative_eq!(wrapped_angle, expected_angle, epsilon = 10.0 * f64::EPSILON);
    }
}
