//! The camera module defines a trait that implementations of cameras can use
//! to generically swap-in different types of cameras. It also provides implementations of various
//! cameras.

use crate::na::RealField;
use crate::ray::Ray;

pub mod pinhole;

pub use pinhole::Pinhole;

/// The `Camera` trait should be implemented for any specific camera implementation.
pub trait Camera<N: RealField + Sync> {
    /// Given uv coordinates, return an outgoing ray originating from the viewer's eye
    fn get_ray(&self, u: N, v: N) -> Ray<N>;
}
