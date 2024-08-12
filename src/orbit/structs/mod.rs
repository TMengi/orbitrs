pub mod coe;

pub use coe::COE;

pub enum Orbit {
    COE(coe::COE),
    EOE,
}
