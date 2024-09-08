#![allow(clippy::wildcard_imports)]

#[macro_use]
extern crate approx;

pub mod angle_ops;
pub mod constants;
pub mod orbit;
pub mod quaternions;
pub mod testing;
pub mod vector;
pub mod vector_ops;

fn main() {
    println!("Hello, world!");
}
