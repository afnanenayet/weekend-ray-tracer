# weekend ray tracer

[![Build Status](https://travis-ci.org/afnanenayet/basic-ray-tracer.svg?branch=master)](https://travis-ci.org/afnanenayet/basic-ray-tracer)

## Synopsis

This is a toy ray tracer written in Rust, following "Ray Tracing in One Weekend"

## Pictures

![Spheres shaded by their normals with no antialiasing](sphere_lo_no_aa.png)

Spheres, shaded by their normals, with no antialiasing

![Spheres shaded by their normals with antialiasing](sphere_lo_aa.png)

Spheres, shaded by their normals, with antialiasing

## Sources

I used these sources for reference and inspiration to help me with this project.
For the more core ray-tracing part, I used the classic book
[Ray Tracing in One Weekend](https://www.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8).
I also referenced a [tutorial](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/)
on writing a ray tracer in Rust, since I wanted to
make sure I was using proper Rust conventions when writing this.

For the rendering side, I referenced the classic [pbrt](http://pbrt.org) book,
as well as information from Dartmouth's CS87 rendering course, taught
by [Wojciech Jarosz](https://cs.dartmouth.edu/~wjarosz/).

I'm also using [nalgebra](http://nalgebra.org) as my linear algebra library.

## Usage

To run tests:

    cargo test

To build:

    cargo build --release

To run:

    cargo run --release
