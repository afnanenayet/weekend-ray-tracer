extern crate image;
extern crate nalgebra as na;
extern crate indicatif;
extern crate rand;
extern crate rayon;
extern crate trtlib;

use na::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::default::Default;
use std::fs;
use std::path::Path;
use std::time::Instant;
use std::borrow::Cow;
use std::vec::Vec;
use trtlib::camera::pinhole::Pinhole;
use trtlib::camera::Camera;
use trtlib::hittable::{HitList, HitRecord, Hittable};
use trtlib::material::diffuse::Diffuse;
use trtlib::material::mirror::Mirror;
use trtlib::material::BSDF;
use trtlib::primitives::sphere::Sphere;
use trtlib::typedefs::*;

/// Create the render/output directory if it doesn't already exist. If it does, do nothing.
fn create_render_dir(dir: &str) -> std::io::Result<()> {
    if !Path::new(dir).exists() {
        fs::create_dir(dir)?
    }
    Ok(())
}

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
                color(&scattered_ray, primitives, depth + 1, depth_limit)
                    .component_mul(&attenuation)
            } else {
                Vector3f::new(0.0, 0.0, 0.0)
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

/// Creates a progress bar with the style we want for this app
fn create_progress_bar(size: u64) -> ProgressBar {
    let style = ProgressStyle::default_bar()
        .progress_chars("=>-")
        .template("{elapsed_precise} / {eta_precise} (ETA) [{wide_bar}] {percent}%");
    let pb = ProgressBar::new(size);
    pb.set_style(style);
    pb
}

fn main() -> std::io::Result<()> {
    let nx = 1920; // width
    let ny = 1080; // height
    let ns = 200; // antialiasing factor

    // initialize scene objects
    let primitives = scene();
    let camera: Pinhole<f32> = Default::default();

    // recursion limit
    let rec_lim = 50;

    println!("Rendering scene...");
    let mut buffer: Vec<[u8; 3]> = Vec::with_capacity(nx * ny);

    // initialize progress bar so we can track progress from the CLI
    let pb = create_progress_bar((nx * ny) as u64);

    (0..(nx * ny))
        .into_par_iter()
        .map(|idx| {
            pb.inc(1);
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

            // write to file with some sanity checking
            if ir > 256 || ig > 256 || ib > 256 {
                println!("ERROR: invalid color value ({}, {}, {})", ir, ig, ib);
                return [1 as u8; 3];
            }
            [ir as u8, ig as u8, ib as u8]
        }).collect_into_vec(&mut buffer);
    pb.finish();

    println!("Writing buffer to file");
    let start_time = Instant::now();

    // flatten the image buffer so it can be saved using the image crate
    // Note that this is a performance issue as it doubles the memory necessary
    let image_buffer: Vec<u8> = buffer.iter().flat_map(|n| n.iter().cloned()).collect();
    create_render_dir("renders")?;
    image::save_buffer(
        "renders/render.png",
        &image_buffer,
        nx as u32,
        ny as u32,
        image::RGB(8),
        )?;
    let elapsed = start_time.elapsed().as_secs();
    println!("File took {} seconds to write to disk\n", elapsed);
    Ok(())
}
