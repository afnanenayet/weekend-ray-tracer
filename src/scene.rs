//! Methods to construct scene data
//!
//! This module contains methods that construct a scene, whether it's the
//! default image, randomly generated circle, or deserialized from a file

use crate::hittable::{ObjRef, ObjVec};
use crate::material::diffuse::Diffuse;
use crate::material::mirror::Mirror;
use crate::primitives::sphere::Sphere;
use crate::typedefs::*;

/// Macro to initialize the ObjVec used for scenes
///
/// This vector is similar to the `vec!` macro, except it is customized for how the ObjVec is
/// commonly iniitalized (the elements of each tuple must be Boxed).
macro_rules! obj_vec {
    (
        // We need the type here otherwise the vector will automatically adopt the types of the
        // first elements passed in, rather than using `dyn Trait`, which causes compilation
        // errors. Additionally, this lets the user define what kind of numeric type to use for the
        // vectors (whether it's f32, f64).
        $t:ty; [ $( ( $prim:expr, $mat:expr ) ),* ]
    ) => {
        {
            let mut vector: ObjVec<$t> = Vec::new();
            $(
                vector.push(
                    ObjRef{
                        hittable: Box::new($prim),
                        material: Box::new($mat),
                    }
                );
            )*
            vector
        }
    };
}

/// Constructs the default scene found on the cover of the ray tracing in one weekend book
pub fn default_scene() -> ObjVec<f> {
    obj_vec!(f; [
        (
            Sphere {
                radius: 0.5,
                center: Vector3f::new(0.0, 0.0, -1.0),
            },
            Diffuse {
                albedo: Vector3f::new(0.8, 0.3, 0.3),
            }
        ),
        (
            Sphere {
                radius: 100.0,
                center: Vector3f::new(0.0, -100.5, -1.0),
            },
            Diffuse {
                albedo: Vector3f::new(0.8, 0.8, 0.0),
            }
        ),
        (
            Sphere {
                radius: 0.5,
                center: Vector3f::new(1.0, 0.0, -1.0),
            },
            Mirror {
                albedo: Vector3f::new(0.8, 0.6, 0.2),
                fuzziness: 0.0,
            }
        ),
        (
            Sphere {
                radius: 0.5,
                center: Vector3f::new(-1.0, 0.0, -1.0),
            },
            Mirror {
                albedo: Vector3f::new(0.8, 0.8, 0.8),
                fuzziness: 0.0,
            }
        )
    ])
}

/// A simple test scene for debugging
pub fn test_scene() -> ObjVec<f> {
    obj_vec!(f; [
             (Sphere {
                 center: Vector3f::new(0.0, 0.0, -1.0),
                 radius: 0.5,
             },
             Diffuse {
                 albedo: Vector3f::new(0.8, 0.3, 0.3)
             })
    ])
}
