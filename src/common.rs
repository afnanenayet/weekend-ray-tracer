//! This module contains common utility functions to be shared across modules. These are functions
//! that don't fit in anywhere else, but have a scope that extends beyond just one module.

use crate::na::{Matrix, RealField, Vector3};
use num::FromPrimitive;
use rand::prelude::*;
use rand::thread_rng;

/// Mirror a vector about a unit direction. `vector` is the incoming vector, and `normal` is the
/// vector to mirror `vector` around. Returns a mirrored vector. Note that `normal` must be a
/// unit vector.
pub fn mirror<N: RealField + FromPrimitive>(
    vector: &Vector3<N>,
    normal: &Vector3<N>,
) -> Vector3<N> {
    vector - normal.map(|e| e * (N::from_u32(2).unwrap() * Matrix::dot(vector, normal)))
}

/// Generate a random point within the unit sphere
pub fn random_in_unit_sphere<N>() -> Vector3<N>
where
    rand::distributions::Standard: rand::distributions::Distribution<N>,
    N: RealField + FromPrimitive + num::Float,
{
    let mut rng = thread_rng();

    let x: N = rng.gen();
    let y: N = rng.gen();
    let z: N = rng.gen();

    // Get the magnitude of the sphere
    let mag = num::Float::sqrt(x * x + y * y + z * z);
    let norm_factor = num::Float::powf(mag, N::from(1.0 / 3.0).unwrap());
    Vector3::<N>::new(x * norm_factor, y * norm_factor, z * norm_factor)
}
