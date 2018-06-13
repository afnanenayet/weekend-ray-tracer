extern crate nalgebra as na;
extern crate trtlib;

use na::Vector3;
use std::fs::File;
use std::io::prelude::*;
use trtlib::typedefs::*;

/// Calculate whether a sphere primitive has been hit based on the properties of the sphere and
/// the camera ray.
///
/// _ TODO _: this function should be moved to somewhere more sensible, perhaps a structure to
/// hold meshes/geometric primitives
fn hit_sphere(center: &Point3f, radius: f32, r: &Ray3f) -> bool {
    let oc: Vector3f = r.origin - center;
    let a = na::norm_squared(&r.direction);
    let b = 2.0 * na::dot(&oc, &r.direction);
    let c = na::norm_squared(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    // using an analytic solution to determine whether sphere has been hit
    discriminant > 0.0
}

/// Calculate the background color that corresponds to an outgoing camera ray. Creates a blend of
/// blue and white.
fn color(r: &Ray3f) -> Color3f {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vector3::new(1.0, 0.0, 0.0);
    }

    let unit_dir = r.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);

    // linearly interpolate a color based on the angle of the ray
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let nx = 1920;
    let ny = 1080;

    // open file and write P3 file header
    let mut file = File::create("pic.P3")?;
    let file_str = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(file_str.as_bytes())?;

    // image corners
    let lower_left = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal: Vector3f = Vector3::new(4.0, 0.0, 0.0);
    let vertical: Vector3f = Vector3::new(0.0, 2.0, 0.0);
    let origin: Vector3f = Vector3::new(0.0, 0.0, 0.0);

    let mut j = ny - 1;
    let mut i = 0;

    while j >= 0 {
        while i < nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let dir: Vector3f = lower_left + (u * horizontal) + (v * vertical);
            let r = Ray3f::new(&origin, &dir);
            let color: Color3f = color(&r);
            let ir = (color.x * 255.99) as u32;
            let ig = (color.y * 255.99) as u32;
            let ib = (color.z * 255.99) as u32;

            // write to file
            let file_str = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(file_str.as_bytes())?;

            i += 1;
        }
        i = 0;
        j -= 1;
    }

    Ok(())
}
