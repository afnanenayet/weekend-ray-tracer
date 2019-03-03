use crate::hittable::{HitRecord, Hittable};
use crate::na::{Matrix, Real, Vector3};
use crate::ray::Ray;
use num::FromPrimitive;
use serde_derive::{Deserialize, Serialize};

/// Contains the relevant information for a sphere primitive
#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Sphere<N: Real> {
    pub radius: N,
    pub center: Vector3<N>,
}

// This could be more generic, but even if it was, it would be generic over float primitives,
// which would require me to implement traits over primitive types, which is not recommended
// by Rust best practices.
impl<N: Real + FromPrimitive> Hittable for Sphere<N> {
    type NumType = N;

    fn hit(&self, ray: &Ray<Self::NumType>) -> Option<HitRecord<Self::NumType>> {
        let oc = ray.origin - self.center;
        let a = Matrix::norm_squared(&ray.direction);
        let b = Matrix::dot(&oc, &ray.direction);
        let c = Matrix::norm_squared(&oc) - na::Real::powi(self.radius, 2);
        let discriminant = na::Real::powi(b, 2) - (a * c);
        let t = (-b - na::Real::sqrt(discriminant)) / a;

        if discriminant >= N::from_f32(0.0).unwrap() {
            let p = ray.point_at_param(t);
            let normal = (p - self.center).map(|n| n / self.radius);
            return Some(HitRecord { t, p, normal });
        }
        None
    }
}
