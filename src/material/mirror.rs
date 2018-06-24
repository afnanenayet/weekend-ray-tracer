use ::num::FromPrimitive;
use ::na::{self, Real, Vector3};
use material::{BSDFRecord, BSDF};
use hittable::HitRecord;
use ray::Ray;
use common::mirror;
use std::fmt::Debug;

/// Contains the parameters for a mirror struct. The albedo determines the tint of the color
/// retrieved from the mirror BSDF. 
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Mirror<N: Real + Copy + Debug + PartialEq> {
    pub albedo: Vector3<N>,
}


impl<N: FromPrimitive + Real> BSDF<N> for Mirror<N> {
    /// Implements the scatter function for a mirror surface. This mirror implementation takes
    /// the albedo into account and attenuates the reflection based off the albedo. The mirror
    /// reflects the incoming ray about the normal of the incoming ray.
    fn scatter(&self, in_ray: &Ray<N>, hit_record: &HitRecord<N>) -> BSDFRecord<N> {
        let reflection = mirror(&in_ray.direction, &hit_record.normal);
        let scatter_out = Ray{ direction: reflection, origin: hit_record.p};

        let mut bsdf_record: BSDFRecord<N> = BSDFRecord {
            out_scattered: scatter_out, 
            attenuated: self.albedo,
        };

        if na::dot(&scatter_out.direction, &reflection) <= N::from_u32(0).unwrap() {
            bsdf_record.attenuated = Vector3::new(
                N::from_u32(0).unwrap(),
                N::from_u32(0).unwrap(),
                N::from_u32(0).unwrap(),
            );
        }
        bsdf_record
    }
}
