use rand::{thread_rng, Rng};
use na::Real;
use num::FromPrimitive;
use na::{self, Vector3};

/// The sample module contains convenience functions for statistical sampling.

/// Returns a randomly sampled vector inside the unit sphere.
///
/// Note that this uses rejection sampling, a simple, but inefficient sampling strategy.
pub fn unit_sphere<N: Real + FromPrimitive>() -> Vector3<N> {
    let mut rng = thread_rng();
    let mut v = Vector3::new(N::from_f32(0.0).unwrap(), 
                                N::from_f32(0.0).unwrap(), 
                                N::from_f32(0.0).unwrap());
    let unit = Vector3::<N>::new(
        N::from_f32(1.0).unwrap(), 
        N::from_f32(1.0).unwrap(), 
        N::from_f32(1.0).unwrap());

    // keep on generating new vectors until the generated vector falls within the unit sphere
    while na::norm_squared(&v) >= N::from_f32(1.0).unwrap() {
        v = Vector3::new(
            N::from_f32(rng.gen::<f32>()).unwrap(),
            N::from_f32(rng.gen::<f32>()).unwrap(),
            N::from_f32(rng.gen::<f32>()).unwrap(),
        ).map(|e| e * N::from_f32(2.0).unwrap()) - unit;
    }
    return v;
}
