extern crate nalgebra as na;

use crate::na::Vector3;
use clap::{load_yaml, value_t, App};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn};
use pretty_env_logger;
use rand::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::time::Instant;
use std::vec::Vec;
use trtlib::camera::pinhole::Pinhole;
use trtlib::camera::Camera;
use trtlib::hittable::{any_hit, ObjVec};
use trtlib::scene;
use trtlib::typedefs::*;

/// Calculate the background color that corresponds to an outgoing camera ray. Creates a blend of
/// blue and white.
///
/// `r` is the outgoing ray from the camera to the world `objects` is a list of tuple(geometric
/// primitives, materials) that are in the scene `depth` is the recursion depth for global
/// illumination `depth_limit` is the recursion depth limit for global illumination
fn color(r: &Ray3f, primitives: &ObjVec<f>, depth: u, depth_limit: u) -> Color3f {
    let possible_hit_record = any_hit(&primitives, r, Some(0.001), None);

    if let Some(hit_record) = possible_hit_record {
        let hr = hit_record.0;
        let obj = hit_record.1;
        let bsdf = &obj.material;

        // if depth is less than depth limit, then global illumination
        if depth < depth_limit {
            let bsdf_record = bsdf.scatter(r, &hr);
            let attenuation: Vector3f = bsdf_record.attenuated;
            let scattered_ray: Ray3f = bsdf_record.out_scattered;
            // FIXME I think that the scattered rays are getting messed up somehow and hitting the
            // depth limit. Maybe they're going inside the sphere? In any case, whenever an object
            // is hit, this method is returning (0,0,0). The background works fine which leads me
            // to believe that there is an issue with the outgoing ray
            let tmp_color = color(&scattered_ray, primitives, depth + 1, depth_limit)
                .component_mul(&attenuation);
            info!(
                "current color: {}, {}, {}",
                tmp_color.x, tmp_color.y, tmp_color.z
            );
            return tmp_color;
        } else {
            // TODO(afnan) change back to 0, 0, 0
            return Color3f::new(0.0, 0.0, 0.0);
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
        .template("[{bar}] {elapsed_precise} < {eta_precise} {percent}%");
    let pb = ProgressBar::new(size);
    pb.set_style(style);
    pb
}

/// Render the scene, given a configuration. This method takes care of the bulk of the core code to
/// generate the scene as well as parallelize the render.
///
/// Params:
/// - nx: the width of the image
/// - ny: the height of hte image
/// - ns: the antialiasing factor for each pixel
/// - out: the relative output filename for the rendered picture
fn render_scene(
    primitives: &ObjVec<f>,
    nx: usize,
    ny: usize,
    ns: usize,
    out: &str,
) -> std::io::Result<()> {
    let camera = Pinhole::default();

    // recursion limit
    let depth_limit = 50;
    info!("Using a depth limit of {}", depth_limit);

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
                let u = (i as f + rng.gen::<f>()) / (nx as f);
                let v = (j as f + rng.gen::<f>()) / (ny as f);
                let r = camera.get_ray(u, v);
                col += color(&r, &primitives, 0, depth_limit);
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
            // We use black as the default value if there is some incorrect value
            if ir > 256 || ig > 256 || ib > 256 {
                error!(
                    "ERROR: generated invalid color value ({}, {}, {})",
                    ir, ig, ib
                );
                return [0 as u8; 3];
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
    image::save_buffer(out, &image_buffer, nx as u32, ny as u32, image::RGB(8))?;
    let elapsed = start_time.elapsed().as_secs();
    println!("File took {} seconds to write to disk\n", elapsed);
    Ok(())
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    // load the args from a yaml file
    let yaml = load_yaml!("cli.yaml");
    info!("Parsing command line parameters");
    let matches = App::from_yaml(yaml).get_matches();

    // Check for args or get default values
    // The default values should be relatively cheap
    let width = value_t!(matches.value_of("width"), usize).unwrap_or(200);
    let height = value_t!(matches.value_of("height"), usize).unwrap_or(100);
    let aa = value_t!(matches.value_of("aa"), usize).unwrap_or(50);
    let output_fname: &str = matches.value_of("out").unwrap_or("render.png");
    let scene = scene::test_scene();

    info!("Preparing to render scene");
    render_scene(&scene, width, height, aa, output_fname)
}
