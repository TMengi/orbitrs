use crate::orbit::traits::*;
use crate::orbit::FromBuilder;

pub use coe_canonical::COE;
mod coe_canonical {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct COE {
        pub semi_major_axis: f64,
        pub eccentricity: f64,
        pub inclination: f64,
        pub arg_peri: f64,
        pub raan: f64,
        pub true_anomaly: f64,
    }

    impl FromBuilder for COE {}

    impl Sized for COE {
        fn semi_major_axis(&self) -> f64 {
            self.semi_major_axis
        }

        fn semi_latus_rectum(&self) -> f64 {
            self.semi_major_axis * (1. - self.eccentricity.powi(2))
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
        use crate::orbit::Builder;
        use crate::orbit::Orbit;

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

        #[test]
        fn builder_test() {
            let sma = 42_164_000.;
            let mut builder = Builder::new();
            builder.semi_major_axis(sma);
            let orbit = builder.build();
            if let Orbit::COE(coe) = orbit {
                assert_eq!(coe.semi_major_axis, sma);
            } else {
                panic!("Got the wrong orbit type");
            }
        }
    }
}

pub use coe_slr::COESlr;
mod coe_slr {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct COESlr {
        pub semi_latus_rectum: f64,
        pub eccentricity: f64,
        pub inclination: f64,
        pub arg_peri: f64,
        pub raan: f64,
        pub true_anomaly: f64,
    }

    impl Sized for COESlr {
        fn semi_major_axis(&self) -> f64 {
            self.semi_latus_rectum / (1. - self.eccentricity.powi(2))
        }

        fn semi_latus_rectum(&self) -> f64 {
            self.semi_latus_rectum
        }
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::*;

    #[test]
    fn coe_size_test() {
        let coe_canonical = COE {
            semi_major_axis: 1234.,
            eccentricity: 0.,
            inclination: 1.,
            arg_peri: 2.,
            raan: 3.,
            true_anomaly: 4.,
        };
        let coe_slr = COESlr {
            semi_latus_rectum: 1234.,
            eccentricity: 0.,
            inclination: 1.,
            arg_peri: 2.,
            raan: 3.,
            true_anomaly: 4.,
        };
        assert_eq!(coe_canonical.semi_major_axis(), coe_slr.semi_major_axis());
        assert_eq!(coe_canonical.semi_latus_rectum(), coe_slr.semi_latus_rectum());
    }
}
