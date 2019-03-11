//! Methods to construct scene data
//!
//! This module contains methods that construct a scene, whether it's the
//! default image, randomly generated circle, or deserialized from a file

use crate::hittable::ObjVec;
use crate::material::diffuse::Diffuse;
use crate::material::mirror::Mirror;
use crate::primitives::sphere::Sphere;
use crate::material::BSDF;
use crate::hittable::Hittable;
use crate::typedefs::*;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use serde_yaml;

type SerializePair<N> = (Box<Hittable<NumType = N>>, Box<BSDF<N>>);

/// Constructs the default scene found on the cover of the ray tracing in one weekend book
pub fn default_scene() -> ObjVec<f32> {
    let mut v: ObjVec<f32> = Vec::new();
    let mut s: Vec<SerializePair<f32>> = Vec::new();

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
    s.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(-1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3f::new(0.8, 0.8, 0.8),
        }),
    ));

    // print the serialized scene for future use
    let yaml = serde_yaml::to_string(&s).unwrap();
    v
}

/// Randomly generate a scene with up to 100 primitives. The properties of the primitives will
/// be randomized
pub fn random_scene() -> ObjVec<f32> {
    let mut rng = ThreadRng::default();
    let mut v: ObjVec<f32> = Vec::new();
    let num_prims: u32 = rng.gen_range(0, 100);

    println!("Generating {} primitives", num_prims);

    for i in 0..num_prims {
        // randomly select a BSDF
        let center_vec = Vector3f::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-0.5, 0.5),
            rng.gen_range(-1.0, 1.0),
        );
        let albedo_vec = Vector3f::new(
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
        );
        //let radius: f32 = rng.gen_range(0.0, 10.0);
        let radius = 0.5;

        if rng.gen_bool(0.5) {
            v.push((
                Box::new(Sphere {
                    radius,
                    center: center_vec,
                }),
                Box::new(Mirror { albedo: albedo_vec }),
            ));
        } else {
            v.push((
                Box::new(Sphere {
                    radius,
                    center: center_vec,
                }),
                Box::new(Diffuse { albedo: albedo_vec }),
            ));
        }
    }
    v
}
