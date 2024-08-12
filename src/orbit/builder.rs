#[derive(Debug, PartialEq)]
pub struct Orbit {
    semi_major_axis: f64,
}

impl Orbit {
    pub fn new(semi_major_axis: f64) -> Self {
        Self{semi_major_axis}
    }

    pub fn build() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    semi_major_axis: Option<f64>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            semi_major_axis: None,
        }
    }

    pub fn semi_major_axis(&mut self, sma: f64) -> &mut Self {
        self.semi_major_axis = Some(sma);
        self
    }

    pub fn build(&self) -> Orbit {
        Orbit {
            semi_major_axis: self.semi_major_axis.expect("Semi major axis is not defined"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let sma_raw = 42_164_000.;
        let orbit_from_new = Orbit::new(sma_raw);
        let orbit_from_builder = Orbit::build().semi_major_axis(sma_raw).build();
        assert_eq!(orbit_from_new, orbit_from_builder);

        let mut builder = Orbit::build();
        builder.semi_major_axis(sma_raw);
        let other_orbit_from_builder = builder.build();
        assert_eq!(orbit_from_builder, other_orbit_from_builder);
    }
}
