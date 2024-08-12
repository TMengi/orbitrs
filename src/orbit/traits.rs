pub trait Sized {
    fn semi_major_axis(&self) -> f64;
    fn semi_latus_rectum(&self) -> f64;
    fn period(&self) -> f64;
}

pub trait Shaped {
    fn eccentricity(&self) -> f64;
}

pub trait Oriented {
    fn inclination(&self) -> f64;
    fn arg_peri(&self) -> f64;
    fn raan(&self) -> f64;
}
