extern crate nalgebra as na;
extern crate pbr;
extern crate rand;
extern crate rayon;
extern crate trtlib;
extern crate image;

use na::Vector3;
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::vec::Vec;
use trtlib::camera::pinhole::Pinhole;
use trtlib::camera::Camera;
use trtlib::hittable::{HitList, HitRecord, Hittable};
use trtlib::material::diffuse::Diffuse;
use trtlib::material::mirror::Mirror;
use trtlib::material::BSDF;
use trtlib::primitives::sphere::Sphere;
use trtlib::typedefs::*;
use image::ImageBuffer;

/// Constructs the objects in the scene and returns a vector populated by those objects.
fn scene() -> HitList<f32> {
    let mut v: Vec<(Box<Hittable<NumType = f32> + Sync>, Box<BSDF<f32> + Sync>)> = Vec::new();

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
            albedo: Vector3::new(0.8, 0.8, 0.0),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3::new(0.8, 0.6, 0.2),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(-1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3::new(0.8, 0.8, 0.8),
        }),
    ));

    // Return list with the HitList wrapper type
    HitList { list: v }
}

/// Calculate the background color that corresponds to an outgoing camera ray. Creates a blend of
/// blue and white.
///
/// `r` is the outgoing ray from the camera to the world
/// `objects` is a list of tuple(geometric primitives, materials) that are in the scene
/// `depth` is the recursion depth for global illumination
/// `depth_limit` is the recursion depth limit for global illumination
fn color(r: &Ray3f, primitives: &HitList<f32>, depth: u32, depth_limit: u32) -> Color3f {
    let hit_record: Option<(HitRecord<f32>, &Box<BSDF<f32> + Sync>)> =
        primitives.any_hit(r, Some(0.001), None);

    match hit_record {
        Some(pair) => {
            let hr = pair.0;
            let bsdf = pair.1;

            // if depth is less than depth limit, then global illumination
            if depth < depth_limit {
                let bsdf_record = bsdf.scatter(r, &hr);
                let attenuation: Vector3f = bsdf_record.attenuated;
                let scattered_ray: Ray3f = bsdf_record.out_scattered;
                return color(&scattered_ray, primitives, depth + 1, depth_limit)
                    .component_mul(&attenuation);
            } else {
                return Vector3f::new(0.0, 0.0, 0.0);
            }
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
    let nx = 200; // width
    let ny = 100; // height
    let ns = 100; // antialiasing factor

    // initialize scene objects
    let primitives = scene();
    let camera: Pinhole<f32> = Default::default();
    let rec_lim = 50;

    println!("Rendering scene...");

    // time how long it takes to render the scene
    let mut buffer: Vec<String> = Vec::with_capacity(nx * ny);

    // want to time how long it takes to render, now how long it takes to allocate the memory
    let start_time = Instant::now();

    (0..(nx * ny))
        .into_par_iter()
        .map(|idx| {
            let j = ny - (idx / nx);
            let i = idx % nx;
            let mut rng = thread_rng();

            // accumulate colors via AA
            let mut col = Color3f::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / (nx as f32);
                let v = (j as f32 + rng.gen::<f32>()) / (ny as f32);
                let r = camera.get_ray(u, v);
                col += color(&r, &primitives, 0, rec_lim);
            }

            // average out the color values
            col /= ns as f32;
            col.apply(|e| e.sqrt());

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
            return file_str;
        })
        .collect_into_vec(&mut buffer);

    let elapsed = start_time.elapsed().as_secs();

    println!("Scene took {} seconds to render to buffer\n", elapsed);

    println!("Writing buffer to file");
    let mut pb = ProgressBar::new(buffer.len() as u64);

    // open file and write P3 file header
    let mut file = File::create("pic.ppm")?;
    let file_str = format!("P3\n{} {}\n255\n", nx, ny);

    let png = ImageBuffer::from_raw(nx as u32, ny as u32, buffer).unwrap();
    // png.save("render.png")?;
    /*
    file.write_all(file_str.as_bytes())?;
    // dump contents of buffer into file
    for color in buffer.into_iter() {
        file.write_all(color.as_bytes())?;
        pb.inc();
    }
    */
    Ok(())
}
