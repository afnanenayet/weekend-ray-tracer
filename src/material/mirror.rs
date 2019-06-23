use crate::common::{mirror, random_in_unit_sphere};
use crate::hittable::HitRecord;
use crate::material::{BSDFRecord, BSDF};
use crate::na::{Matrix, RealField, Vector3};
use crate::ray::Ray;
use num::{Float, FromPrimitive};
use std::fmt::Debug;

/// Contains the parameters for a mirror struct. The albedo determines the tint of the color
/// retrieved from the mirror BSDF.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Mirror<N>
where
    N: RealField + Copy + Debug + PartialEq,
{
    /// A vector representing the tint of the color retrieved from the mirror BSDF
    pub albedo: Vector3<N>,

    /// How "fuzzy" the reflections from the materials should be.
    ///
    /// This parameter determines how much the reflected ray will be randomized, which makes the
    /// material seem like it is partially diffuse. This value will be clamped between 0 and 1.
    pub fuzziness: N,
}

impl<N> BSDF<N> for Mirror<N>
where
    N: FromPrimitive + RealField + Float,
    rand::distributions::Standard: rand::distributions::Distribution<N>,
{
    /// Implements the scatter function for a mirror surface. This mirror implementation takes
    /// the albedo into account and attenuates the reflection based off the albedo. The mirror
    /// reflects the incoming ray about the normal of the incoming ray.
    fn scatter(&self, in_ray: &Ray<N>, hit_record: &HitRecord<N>) -> BSDFRecord<N> {
        let fuzz = num::clamp(
            self.fuzziness,
            N::from_u32(0).unwrap(),
            N::from_u32(1).unwrap(),
        );
        let fuzz_vector = random_in_unit_sphere().map(|x| x * fuzz);
        let reflection = mirror(&in_ray.direction, &hit_record.normal) + fuzz_vector;
        let scatter_out = Ray {
            direction: reflection,
            origin: hit_record.p,
        };
        let mut bsdf_record: BSDFRecord<N> = BSDFRecord {
            out_scattered: scatter_out,
            attenuated: self.albedo,
        };

        if Matrix::dot(&scatter_out.direction, &reflection) <= N::from_u32(0).unwrap() {
            bsdf_record.attenuated = Vector3::new(
                N::from_u32(0).unwrap(),
                N::from_u32(0).unwrap(),
                N::from_u32(0).unwrap(),
            );
        }
        bsdf_record
    }
}
