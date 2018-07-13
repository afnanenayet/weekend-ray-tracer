//! The camera module provides a generic camera trait that can be implemented
//! to provide different camera types.

use na::Real;
use ray::Ray;

pub mod pinhole;

/// The `Camera` trait should be implemented for any specific camera implementation.
pub trait Camera<N: Real> {
    /// Given uv coordinates, return an outgoing ray originating from the viewer's eye
    fn get_ray(&self, u: N, v: N) -> Ray<N>;
}
