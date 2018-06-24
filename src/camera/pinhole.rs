//! The camera module defines a trait that implementations of cameras can use
//! to generically swap-in different types of cameras.

use na::{Real, Vector3};
use ray::Ray;
use std::default::Default;
use typedefs::Vector3f;
use super::Camera;

/// stores data for camera abstraction
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Pinhole<N: Real> {
    pub origin: Vector3<N>,
    pub horizontal: Vector3<N>,
    pub vertical: Vector3<N>,
    pub lower_left: Vector3<N>,
}

impl<N: Real> Camera<N> for Pinhole<N> {
    /// Return an outgoing directional ray for a camera based on supplied uv coordinates
    fn get_ray(&self, u: N, v: N) -> Ray<N> {
        Ray {
            origin: self.origin,
            direction: self.lower_left
                + self.horizontal.map(|e| e * u)
                + self.vertical.map(|e| e * v) - self.origin,
        }
    }
}

impl Default for Pinhole<f32> {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Pinhole<f32> {
        Pinhole {
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
