extern crate nalgebra as na;
extern crate trtlib;

use na::Vector3;
use std::fs::File;
use std::io::prelude::*;
use trtlib::typedefs::*;
use trtlib::*;

// fn color(r: &Ray3f) -> Vector3f {
// }

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let mut file = File::create("pic.P3")?;

    let lower_left = Vector3::new(-2.0, -1.0, -1.0);
    Ok(())
}
