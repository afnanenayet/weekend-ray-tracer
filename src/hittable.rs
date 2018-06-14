use na::{Real, Scalar, Vector3};
use ray::Ray;

/// A struct that is returned by a hit query that indicates whether some object has been hit by a
/// ray, and relevant location information if it has.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct HitRecord<N: Scalar + Real> {
    t: N,
    p: Vector3<N>,
    normal: Vector3<N>,
    hit: bool,
}

/// Any object/struct that implements `Hittable` is something that can be hit by a ray and
/// rendered on-screen. The function returns a `HitRecord` struct, which contains a relevant
/// information about the hit
pub trait Hittable {
    /// Whether the object was hit. If so, it will be indicated in the hit record along with other
    /// relevant info.
    fn hit<N: Scalar + Real>(ray: &Ray<N>, t_min: N, t_max: N) -> HitRecord<N>;
}
