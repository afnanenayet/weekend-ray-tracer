use material::BSDF;
use na::{Real, Vector3};
use ray::Ray;

/// A struct that is returned by a hit query that indicates whether some object has been hit by a
/// ray, and relevant location information if it has.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct HitRecord<N: Real + Sync> {
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
pub struct HitList<N: Sync> {
    pub list: Vec<(Box<Hittable<NumType = N> + Sync>, Box<BSDF<N> + Sync>)>,
}

impl<N: Real + Sync> HitList<N> {
    /// Return a tuple with a (`HitRecord` struct, `BSDF`) struct, if any structure in the hit
    /// list is hit by the ray, within the bounds. If nothing is hit, `None` will be returned.
    pub fn any_hit(
        &self,
        ray: &Ray<N>,
        t_min: Option<N>,
        t_max: Option<N>,
    ) -> Option<(HitRecord<N>, &Box<BSDF<N> + Sync>)> {
        let mut closest_hit: Option<HitRecord<N>> = None;
        let mut mat: &Box<BSDF<N> + Sync> = &self.list[0].1;

        // use iter instead of into_iter because we don't actually need to manipulate
        // any of the primitives, and we can avoid a compiler error
        for pair in self.list.iter() {
            let record = pair.0.hit(ray);

            if record.is_some()
                && (closest_hit.is_none() || record.unwrap().t < closest_hit.unwrap().t)
                && (t_min.is_none() || record.unwrap().t >= t_min.unwrap())
                && (t_max.is_none() || record.unwrap().t <= t_max.unwrap())
            {
                closest_hit = record;
                mat = &pair.1;
            }
        }

        if closest_hit.is_some() {
            return Some((closest_hit.unwrap(), mat));
        }
        return None;
    }
}
