use na::{Real, Vector3};
use ray::Ray;

/// A struct that is returned by a hit query that indicates whether some object has been hit by a
/// ray, and relevant location information if it has.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct HitRecord<N: Real> {
    pub t: N,
    pub p: Vector3<N>,
    pub normal: Vector3<N>,
}

/// Any object/struct that implements `Hittable` is something that can be hit by a ray and
/// rendered on-screen. The function returns a `HitRecord` struct, which contains a relevant
/// information about the hit.
pub trait Hittable {
    type NumType: Real;

    /// Whether the object was hit. If so, it will be indicated in the hit record along with other
    /// relevant info. If there is a hit, then there will be a hit record. If not, a `None`
    /// will be returned.
    fn hit(
        &self,
        ray: &Ray<Self::NumType>,
        t_min: Self::NumType,
        t_max: Self::NumType,
    ) -> Option<HitRecord<Self::NumType>>;
}
