//! This module contains common utility functions to be shared across modules. These are functions
//! that don't fit in anywhere else, but have a scope that extends beyond just one module.

use na::{self, Real, Vector3};
use num::FromPrimitive;

/// Mirror a vector about a unit direction. `vector` is the incoming vector, and `normal` is the
/// vector to mirror `vector` around. Returns a mirrored vector. Note that `normal` must be a
/// unit vector.
pub fn mirror<N: Real + FromPrimitive>(vector: &Vector3<N>, normal: &Vector3<N>) -> Vector3<N> {
    vector - normal.map(|e| e * (N::from_u32(2).unwrap() * na::dot(vector, normal)))
}
