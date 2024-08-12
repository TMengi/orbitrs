/// Define a single Orbit struct that can be created from a builder
///
/// Advantages:
///     - Nice syntax for creating objects, inherently keyword-based
///
/// Disadvantages:
///     - Underlying data storage has to use the same field representation

#[derive(Debug, PartialEq)]
pub struct Orbit {
    semi_major_axis: f64,
}

impl Orbit {
    pub fn new(semi_major_axis: f64) -> Self {
        Self{semi_major_axis}
    }

    pub fn build() -> OrbitBuilder {
        OrbitBuilder::new()
    }
}

pub struct OrbitBuilder {
    semi_major_axis: Option<f64>,
}

impl OrbitBuilder {
    pub fn new() -> Self {
        Self {
            semi_major_axis: None,
        }
    }

    pub fn semi_major_axis(mut self, sma: f64) -> Self {
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
    }
}
