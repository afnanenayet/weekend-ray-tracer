extern crate nalgebra as na;
extern crate pbr;
extern crate rand;
extern crate trtlib;

use na::Vector3;
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use trtlib::camera::Camera;
use trtlib::hittable::{HitList, HitRecord, Hittable};
use trtlib::primitives::sphere::Sphere;
use trtlib::typedefs::*;

/// Constructs the objects in the scene and returns a vector populated by those objects.
fn scene() -> HitList<f32> {
    let mut v: Vec<Box<Hittable<NumType = f32>>> = Vec::new();

    // specify objects here
    v.push(Box::new(Sphere {
        radius: 0.5,
        center: Vector3f::new(0.0, 0.0, -1.0),
    }));
    v.push(Box::new(Sphere {
        radius: 100.0,
        center: Vector3f::new(0.0, -100.5, -1.0),
    }));

    // Return list with the HitList wrapper type
    HitList { list: v }
}

/// Calculate the background color that corresponds to an outgoing camera ray. Creates a blend of
/// blue and white.
///
/// `r` is the outgoing ray from the camera to the world
/// `objects` is a list of geometric primitives that are in the scene
fn color(r: &Ray3f, primitives: &HitList<f32>) -> Color3f {
    let hit_record: Option<HitRecord<f32>> = primitives.any_hit(r, Some(0.0), None);

    match hit_record {
        Some(hr) => {
            // with hit, return color based on the normal
            return 0.5 * Vector3::new(hr.normal.x + 1.0, hr.normal.y + 1.0, hr.normal.z + 1.0);
        }
        None => {
            let unit_dir = r.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);

            // linearly interpolate a color based on the angle of the ray
            return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    // initialize scene objects
    let primitives = scene();
    let camera: Camera<f32> = Default::default();

    // open file and write P3 file header
    let mut file = File::create("pic.p3")?;
    let file_str = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(file_str.as_bytes())?;

    // random generator
    let mut rng = thread_rng();
    let mut j = ny;

    // initialize progress bar for terminal
    let mut pb = ProgressBar::new(nx * ny);

    while j > 0 {
        for i in 0..nx {
            // accumulate colors via AA
            let mut col = Color3f::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
                let r = camera.get_ray(u, v);
                col += color(&r, &primitives);
            }

            // average out the color values
            col /= ns as f32;

            // writing colors as u16 instead of u8 because this allows us to sanity check
            // whether colors would wrap/be invalid
            let ir = (col.x * 255.99) as u16;
            let ig = (col.y * 255.99) as u16;
            let ib = (col.z * 255.99) as u16;
            let mut file_str = format!("{} {} {}\n", ir, ig, ib);

            // write to file with some sanity checking
            if ir > 256 || ig > 256 || ib > 256 {
                println!("ERROR: invalid color value ({}, {}, {})", ir, ig, ib);
                file_str = "1 1 1\n".to_string();
            }
            file.write_all(file_str.as_bytes())?;
            pb.inc();
        }
        j -= 1;
    }
    Ok(())
}
