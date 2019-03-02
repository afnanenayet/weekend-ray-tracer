extern crate nalgebra as na;

use crate::na::Vector3;
use clap::{load_yaml, value_t, App};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::default::Default;
use std::fs;
use std::path::Path;
use std::time::Instant;
use std::vec::Vec;
use trtlib::camera::pinhole::Pinhole;
use trtlib::camera::Camera;
use trtlib::hittable::{HitList, ObjVec};
use trtlib::material::diffuse::Diffuse;
use trtlib::material::mirror::Mirror;
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
    let mut v: ObjVec<f32> = Vec::new();

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
            albedo: Vector3f::new(0.8, 0.8, 0.0),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3f::new(0.8, 0.6, 0.2),
        }),
    ));
    v.push((
        Box::new(Sphere {
            radius: 0.5,
            center: Vector3f::new(-1.0, 0.0, -1.0),
        }),
        Box::new(Mirror {
            albedo: Vector3f::new(0.8, 0.8, 0.8),
        }),
    ));
    HitList { list: v }
}

/// Calculate the background color that corresponds to an outgoing camera ray. Creates a blend of
/// blue and white.
///
/// `r` is the outgoing ray from the camera to the world `objects` is a list of tuple(geometric
/// primitives, materials) that are in the scene `depth` is the recursion depth for global
/// illumination `depth_limit` is the recursion depth limit for global illumination
fn color(r: &Ray3f, primitives: &HitList<f32>, depth: u32, depth_limit: u32) -> Color3f {
    let hit_record = primitives.any_hit(r, Some(0.001), None);

    if let Some(pair) = hit_record {
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
    } else {
        let unit_dir = r.direction.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);

        // linearly interpolate a color based on the angle of the ray
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

/// Creates a progress bar with the style we want for this app
fn create_progress_bar(size: u64) -> ProgressBar {
    let style = ProgressStyle::default_bar()
        .progress_chars("=> ")
        .template("{elapsed_precise} < {eta_precise} (ETA) [{bar}] {percent}%");
    let pb = ProgressBar::new(size);
    pb.set_style(style);
    pb
}

/// Render the scene, given a configuration. This method takes care of the bulk of the core code to
/// generate the scene as well as parallelize the render.
///
/// Params: - nx: the width of the image - ny: the height of hte image - ns: the antialiasing
/// factor for each pixel - out: the relative output filename for the rendered picture
fn render_scene(nx: usize, ny: usize, ns: usize, out: &str) -> std::io::Result<()> {
    // initialize scene objects
    let primitives = scene();
    let camera = Pinhole::<f32>::default();

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
            let mut rng = rand::thread_rng();

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

            // these assertions ensure that the values are above 0 and we aren't losing any
            // information (the values should never get below 0 anyways)
            assert!(col.x >= 0.0 && col.x <= 1.0);
            assert!(col.y >= 0.0 && col.y <= 1.0);
            assert!(col.z >= 0.0 && col.z <= 1.0);

            // writing colors as u16 instead of u8 because this allows us to sanity check whether
            // colors would wrap/be invalid
            let ir = (col.x * 255.99) as u16;
            let ig = (col.y * 255.99) as u16;
            let ib = (col.z * 255.99) as u16;

            // write to file with some sanity checking
            if ir > 256 || ig > 256 || ib > 256 {
                println!(
                    "ERROR: generated invalid color value
            ({}, {}, {})",
                    ir, ig, ib
                );
                return [1 as u8; 3];
            }
            [ir as u8, ig as u8, ib as u8]
        })
        .collect_into_vec(&mut buffer);
    pb.finish();

    println!("Writing buffer to file");
    let start_time = Instant::now();

    // flatten the image buffer so it can be saved using the image crate Note that this is a
    // performance issue as it doubles the memory necessary
    let image_buffer: Vec<u8> = buffer.iter().flat_map(|n| n.iter().cloned()).collect();
    create_render_dir("renders")?;
    image::save_buffer(out, &image_buffer, nx as u32, ny as u32, image::RGB(8))?;
    let elapsed = start_time.elapsed().as_secs();
    println!("File took {} seconds to write to disk\n", elapsed);
    Ok(())
}

fn main() -> std::io::Result<()> {
    // load the args from a yaml file
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // Check for args or get default values We'll use relatively cheap default values
    let width = value_t!(matches.value_of("width"), usize).unwrap_or(200);
    let height = value_t!(matches.value_of("height"), usize).unwrap_or(100);
    let aa = value_t!(matches.value_of("aa"), usize).unwrap_or(50);
    let output_fname: &str = matches.value_of("out").unwrap_or("renders/render.png");

    render_scene(width, height, aa, output_fname)
}
