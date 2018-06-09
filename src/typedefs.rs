/// Convenience type aliases for commonly used vector types. Note that all types are
/// signed.
use na::Vector3;
use ray::Ray;

/// A three dimensional integer vector
pub type Vector3d = Vector3<i32>;

/// A three dimensional floating point vector
pub type Vector3f = Vector3<f32>;

/// A three dimension floating point vector meant to represent an RGB color value
pub type Color3f = Vector3<f32>;

/// A three dimensional floating point vector meant to represent a 3D point in space
pub type Point3f = Vector3<f32>;

/// A ray that contains a three dimensional floating point vector for both its origin and
/// direction
pub type Ray3f = Ray<f32>;
