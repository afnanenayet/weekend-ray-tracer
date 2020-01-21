use super::Camera;
use crate::na::{RealField, Vector3};
use crate::ray::Ray;
use crate::typedefs::Vector3f;
use std::default::Default;

/// A pinhole camera
///
/// The camera has a configurable field of view that can be set in two axes.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Pinhole<N: RealField> {
    /// The center
    pub origin: Vector3<N>,
    /// The horizontal span of the camera's field of view
    pub horizontal: Vector3<N>,
    /// The vertical span of the camera's field of view
    pub vertical: Vector3<N>,
    /// The lower left corner of the camera's field of view
    pub lower_left: Vector3<N>,
}

impl<N: RealField> Camera<N> for Pinhole<N> {
    /// Return an outgoing directional ray for a camera based on supplied uv coordinates
    fn get_ray(&self, u: N, v: N) -> Ray<N> {
        let direction =
            self.lower_left + self.horizontal.map(|e| e * u) + self.vertical.map(|e| e * v)
                - self.origin;
        Ray {
            origin: self.origin,
            direction,
        }
    }
}

impl Default for Pinhole<f32> {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Self {
        Self {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            horizontal: Vector3f::new(4.0, 0.0, 0.0),
            vertical: Vector3f::new(0.0, 2.0, 0.0),
            lower_left: Vector3f::new(-2.0, -1.0, -1.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ray() {
        let camera: Pinhole<f32> = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<f32> = Ray {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            direction: Vector3f::new(-2.0, -1.0, -1.0),
        };
        assert_eq!(camera.get_ray(0.0, 0.0), ray);

        // middle
        let ray: Ray<f32> = Ray {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            direction: Vector3f::new(0.0, 0.0, -1.0),
        };
        assert_eq!(camera.get_ray(0.5, 0.5), ray);

        // upper left corner
        let ray: Ray<f32> = Ray {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            direction: Vector3f::new(-2.0, 1.0, -1.0),
        };
        assert_eq!(camera.get_ray(0.0, 1.0), ray);

        // upper right corner
        let ray: Ray<f32> = Ray {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            direction: Vector3f::new(2.0, 1.0, -1.0),
        };
        assert_eq!(camera.get_ray(1.0, 1.0), ray);

        // lower right corner
        let ray: Ray<f32> = Ray {
            origin: Vector3f::new(0.0, 0.0, 0.0),
            direction: Vector3f::new(2.0, -1.0, -1.0),
        };
        assert_eq!(camera.get_ray(1.0, 0.0), ray);
    }
}
