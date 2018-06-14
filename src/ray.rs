/// The ray class contains an origin and a direction. Both of these are 3D vectors.
use na::{Real, Scalar, Vector3};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray<N: Scalar + Real> {
    pub origin: Vector3<N>,
    pub direction: Vector3<N>,
}

impl<N: Scalar + Real> Ray<N> {
    /// Returns the coordinate value of a ray that has been extended fowards or
    /// backwards by a certain delta value. A ray takes the form `O + Dt` where
    /// t is the `delta` parameter in this function.
    pub fn point_at_param(self, delta: N) -> Vector3<N> {
        self.origin + self.direction.map(|e| e * delta)
    }

    /// Create a new Ray with a specified origin and direction
    pub fn new(origin: &Vector3<N>, direction: &Vector3<N>) -> Ray<N> {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_ray() {
        let o: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let d: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let r: Ray<f32> = Ray::new(&o, &d);

        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }

    #[test]
    fn test_point_at_param() {
        let o: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let d: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let r: Ray<f32> = Ray::new(&o, &d);

        let point = r.point_at_param(-1.0);
        assert_eq!(point, Vector3::new(0.0, 0.0, -1.0));

        let point = r.point_at_param(1.0);
        assert_eq!(point, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_struct() {
        let o: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let d: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
        let r = Ray {
            origin: o,
            direction: d,
        };
        assert_eq!(r.origin, o);
        assert_eq!(r.direction, d);
    }
}
