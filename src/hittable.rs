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
    fn hit(&self, ray: &Ray<Self::NumType>) -> Option<HitRecord<Self::NumType>>;
}

/// Essentially a wrapper for a vector of Hittable types. It provides a convenience function to
/// detect whether any geometry has been hit, and also makes it easy to add primitives to the
/// list.
pub struct HitList<N> {
    pub list: Vec<Box<Hittable<NumType = N>>>,
}

impl<N: Real> HitList<N> {
    /// Return a `HitRecord` struct if any of the primitives in the hit list were hit by the
    /// ray, within the bounds (inclusive). If nothing was hit, `None` will be returned.
    pub fn any_hit(
        &self,
        ray: &Ray<N>,
        t_min: Option<N>,
        t_max: Option<N>,
    ) -> Option<HitRecord<N>> {
        let mut closest_hit: Option<HitRecord<N>> = None;

        // use iter instead of into_iter because we don't actually need to manipulate
        // any of the primitives, and we can avoid a compiler error
        for primitive in self.list.iter() {
            let record = primitive.hit(ray);

            if record.is_some()
                && (closest_hit.is_none() || record.unwrap().t < closest_hit.unwrap().t)
                && (t_min.is_none() || record.unwrap().t >= t_min.unwrap())
                && (t_max.is_none() || record.unwrap().t <= t_max.unwrap())
            {
                closest_hit = record;
            }
        }

        return closest_hit;
    }
}
