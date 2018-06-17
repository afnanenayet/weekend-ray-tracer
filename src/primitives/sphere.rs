use hittable::{HitRecord, Hittable};
use na::{self, Real, Vector3};
use ray::Ray;

/// Contains the relevant information for a sphere primitive
#[derive(Clone, Debug, Copy)]
pub struct Sphere<N: Real> {
    pub radius: N,
    pub center: Vector3<N>,
}

// This could be more generic, but even if it was, it would be generic over float primitives,
// which would require me to implement traits over primitive types, which is not recommended
// by Rust best practices.
impl Hittable for Sphere<f32> {
    type NumType = f32;

    fn hit(
        &self,
        ray: &Ray<Self::NumType>,
        t_min: Self::NumType,
        t_max: Self::NumType,
    ) -> Option<HitRecord<Self::NumType>> {
        let oc = ray.origin - self.center;
        let a = na::norm_squared(&ray.direction);
        let b = 2.0 * na::dot(&oc, &ray.direction);
        let c = na::norm_squared(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        let t = (-b - discriminant.sqrt()) / (2.0 * a);

        if discriminant >= 0.0 && (t < t_max && t > t_min) {
            let p = ray.point_at_param(t);
            let normal = (p - self.center).map(|n| n / self.radius);
            return Some(HitRecord { t, p, normal });
        }
        None
    }
}
