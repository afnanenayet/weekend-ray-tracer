use super::{BSDFRecord, BSDF};
use crate::hittable::HitRecord;
use crate::na::{Real, Vector3};
use crate::ray::Ray;
use crate::sample::unit_sphere;
use num::FromPrimitive;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

/// Holds the properties for a diffuse BSDF
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub struct Diffuse<N: Real + Copy + Debug + PartialEq> {
    /// The fraction of light that is absorbed by the material
    pub albedo: Vector3<N>,
}

impl<N: FromPrimitive + Real> BSDF<N> for Diffuse<N> {
    // note that the incoming angle doesn't matter for a lambertian surface, which is why we ignore
    // the incoming ray
    fn scatter(&self, _in_ray: &Ray<N>, hit_record: &HitRecord<N>) -> BSDFRecord<N> {
        let target = hit_record.p + hit_record.normal + unit_sphere();
        let scattered = Ray {
            origin: hit_record.p,
            direction: target - hit_record.p,
        };
        let atten = self.albedo;

        BSDFRecord {
            out_scattered: scattered,
            attenuated: atten,
        }
    }
}
