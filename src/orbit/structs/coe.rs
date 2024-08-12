use crate::orbit::traits::{Sized, Shaped, Oriented};

pub struct COE {
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
    arg_peri: f64,
    raan: f64,
    true_anomaly: f64,
}

impl Sized for COE {
    fn semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }
}

impl Shaped for COE {
    fn eccentricity(&self) -> f64 {
        self.eccentricity
    }
}

impl Oriented for COE {
    fn inclination(&self) -> f64 {
        self.inclination
    }

    fn arg_peri(&self) -> f64 {
        self.arg_peri
    }

    fn raan(&self) -> f64 {
        self.raan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coe_test() {
        let coe = COE {
            semi_major_axis: 42_164_000.,
            inclination: 1.,
            eccentricity: 0.1,
            arg_peri: 2.,
            raan: 3.,
            true_anomaly: 4.,
        };
        assert_eq!(coe.semi_major_axis(), coe.semi_major_axis);
        assert_eq!(coe.inclination(), coe.inclination);
        assert_eq!(coe.eccentricity(), coe.eccentricity);
        assert_eq!(coe.arg_peri(), coe.arg_peri);
        assert_eq!(coe.raan(), coe.raan);
    }
}
