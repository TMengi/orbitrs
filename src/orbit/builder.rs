use crate::orbit::structs;

#[derive(Default)]
pub struct Builder {
    semi_major_axis: Option<f64>,
}

pub trait FromBuilder {
    fn build() -> Builder {
        Builder::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn semi_major_axis(&mut self, sma: f64) -> &mut Self {
        self.semi_major_axis = Some(sma);
        self
    }

    pub fn build(&mut self) -> structs::Orbit {
        match self.semi_major_axis {
            Some(semi_major_axis) => structs::Orbit::COE(structs::COE {
                semi_major_axis,
                eccentricity: 0.,
                inclination: 0.,
                arg_peri: 0.,
                raan: 0.,
                true_anomaly: 0.,
            }),
            None => panic!("Not enough fields defined to construct any known Orbit variant"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_builder_test() {
        // Make an arbitrary orbit struct that can be built with the builder
        struct ImaginaryOrbit;
        impl FromBuilder for ImaginaryOrbit {}

        // Should start off empty
        let builder = ImaginaryOrbit::build();
        assert_eq!(builder.semi_major_axis, None);
    }
}
