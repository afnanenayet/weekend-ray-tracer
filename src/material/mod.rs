use hittable::HitRecord;
use na::{Real, Vector3};
use ray::Ray;

pub mod diffuse;
pub mod mirror;

/// Any struct that implements the BSDF trait should contain only parameters relevant to the
/// scattering function. It returns a record struct containing information relevant to the
/// scattering results.
///
/// `in_ray` is the incoming ray from the camera to the surface that was struck.
/// `hit_record` is the HitRecord that is associated with that hit.
pub trait BSDF<N: Real + Sync> {
    fn scatter(&self, in_ray: &Ray<N>, hit_record: &HitRecord<N>) -> BSDFRecord<N>;
}

/// The BSDF record is similar to the `HitRecord` struct. It contains information about the
/// scattering of light when an object is struck.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct BSDFRecord<N: Real + Sync> {
    /// The direction of the ray as a result of the scattering (where it goes)
    pub out_scattered: Ray<N>,

    /// How much the ray should be attenuated
    pub attenuated: Vector3<N>,
}
