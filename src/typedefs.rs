/// Convenience type aliases for commonly used vector types. Note that all types are
/// signed.
use crate::na::Vector3;
use crate::ray::Ray;

/// A typedef to define the precision of floating points
#[allow(non_camel_case_types)]
pub type f = f32;

/// The signed integer size
#[allow(non_camel_case_types)]
pub type i = i32;

/// The unsigned integer size
#[allow(non_camel_case_types)]
pub type u = u32;

/// A three dimensional integer vector
pub type Vector3d = Vector3<i>;

/// A three dimensional floating point vector
pub type Vector3f = Vector3<f>;

/// A three dimension floating point vector meant to represent an RGB color value
pub type Color3f = Vector3<f>;

/// A three dimensional floating point vector meant to represent a 3D point in space
pub type Point3f = Vector3<f>;

/// A ray that contains a three dimensional floating point vector for both its origin and
/// direction
pub type Ray3f = Ray<f>;
