use na::{Real, Vector3};
use ray::Ray;
use std::default::Default;
use typedefs::Vector3f;

/// stores data for camera abstraction
#[derive(Clone, Debug, PartialEq, Copy)]
struct Camera<N: Real> {
    pub origin: Vector3<N>,
    pub horizontal: Vector3<N>,
    pub vertical: Vector3<N>,
    pub lower_left: Vector3<N>,
}

impl<N: Real> Camera<N> {
    /// Return an outgoing directional ray for a camera based on supplied uv coordinates
    pub fn get_ray(&self, u: N, v: N) -> Ray<N> {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal.map(|e| e * u)
                + self.vertical.map(|e| e * v) - self.origin,
        }
    }
}

impl Default for Camera<f32> {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Camera<f32> {
        Camera {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            horizontal: Vector3f::new(4.0, 0.0, 0.0),
            vertical: Vector3f::new(0.0, 2.0, 0.0),
            lower_left: Vector3f::new(-2.0, -1.0, -1.0),
        }
    }
}
