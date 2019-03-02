//! Methods to construct scene data
//!
//! This module contains methods that construct a scene, whether it's the
//! default image, randomly generated circle, or deserialized from a file

use crate::hittable::ObjVec;
use crate::material::diffuse::Diffuse;
use crate::material::mirror::Mirror;
use crate::primitives::sphere::Sphere;
use crate::typedefs::*;

/// Constructs the default scene found on the cover of the ray tracing in one weekend book
pub fn default_scene() -> ObjVec<f32> {
    let mut v: ObjVec<f32> = Vec::new();

    // specify objects here
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(0.0, 0.0, -1.0),
        }),
        Box::new(Diffuse {
            albedo: Vector3f::new(0.8, 0.3, 0.3),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 100.0,
            center: Vector3f::new(0.0, -100.5, -1.0),
        }),
        Box::new(Diffuse {
            albedo: Vector3f::new(0.8, 0.8, 0.0),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3f::new(0.8, 0.6, 0.2),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(-1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3f::new(0.8, 0.8, 0.8),
        }),
    ));
    v
}

pub fn random_scene() -> ObjVec<f32> {
    let mut v: ObjVec<f32> = Vec::new();
    v
}
