use crate::material::BSDF;
use crate::na::{RealField, Vector3};
use crate::ray::Ray;
use log::{info, warn};

/// A struct that is returned by a hit query that indicates whether some object has been hit by a
/// ray, and relevant location information if it has.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct HitRecord<N: RealField + Sync> {
    pub t: N,
    pub p: Vector3<N>,
    pub normal: Vector3<N>,
}

/// Any object/struct that implements `Hittable` is something that can be hit by a ray and
/// rendered on-screen. The function returns a `HitRecord` struct, which contains a relevant
/// information about the hit.
pub trait Hittable {
    type NumType: RealField + Sync;

    /// Whether the object was hit. If so, it will be indicated in the hit record along with other
    /// relevant info. If there is a hit, then there will be a hit record. If not, a `None`
    /// will be returned.
    fn hit(&self, ray: &Ray<Self::NumType>) -> Option<HitRecord<Self::NumType>>;
}

/// A parallel reference to a hittable object
type HittableRef<N> = Box<dyn Hittable<NumType = N> + Sync>;

/// An owned reference to a BSDF trait object that is also `Sync`
pub type BSDFRef<N> = Box<dyn BSDF<N> + Sync>;

/// Objects that consist of a primitive and a material property
///
/// Objects, in this ray tracer, consist of some primitive geometry, and a material property (a
/// BSDF). An `ObjectRef` is threadsafe.
pub struct ObjRef<N> {
    /// The geometric primitive type of the object
    pub hittable: HittableRef<N>,

    /// The material/BSDF property of the object
    pub material: BSDFRef<N>,
}

/// A vector of geometry <-> BSDF ref tuples
pub type ObjVec<N> = Vec<ObjRef<N>>;

/// Return a tuple with a (`HitRecord` struct, `BSDF`) struct, if any structure in the hit
/// list is hit by the ray, within the bounds. If nothing is hit, `None` will be returned.
/// You can specify the maximum and minimum distances for the length of a traced ray.
pub fn any_hit<'a, N: RealField + Sync>(
    list: &'a ObjVec<N>,
    ray: &Ray<N>,
    t_min: Option<N>,
    t_max: Option<N>,
) -> Option<(HitRecord<N>, &'a ObjRef<N>)> {
    if list.len() < 1 {
        warn!("The list of objects was empty. Unless your scene is empty, this should not happen");
        return None;
    }
    let mut closest_hit: Option<HitRecord<N>> = None;
    let mut hit_obj: Option<&ObjRef<N>> = None;

    // Iterate through each object, looking for a hit. If we have a closer hit, update the hit
    // object.
    for obj_ref in list {
        if let Some(hit_record) = obj_ref.hittable.hit(ray) {
            // There are two cases here: either we didn't have a hit before, and now we do, or we
            // already have a hit, and the new hit is closer than that.
            if let Some(curr_closest_hit) = closest_hit {
                if (hit_record.t < curr_closest_hit.t)
                    && (t_min.is_none() || hit_record.t >= t_min.unwrap())
                    && (t_max.is_none() || hit_record.t <= t_max.unwrap())
                {
                    closest_hit = Some(curr_closest_hit);
                    hit_obj = Some(obj_ref);
                }
            } else {
                closest_hit = Some(hit_record);
                hit_obj = Some(obj_ref);
            }
        }
    }

    if let (Some(hit), Some(obj)) = (closest_hit, hit_obj) {
        return Some((hit, obj));
    }
    None
}
