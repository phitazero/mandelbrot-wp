mod complex_plane_view;
mod grid;

use complex_plane_view::ComplexPlaneView;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use num::complex::Complex64;

/// Returns the absolute value of z after n iterations, or None if belongs to the set
fn mandelbrot_iterate(mut z: Complex64, n: u32) -> Option<f64> {
	let initial = z;


	for _ in 0..n {
		z = z * z + initial;

		if z.norm_sqr().is_nan() {
			return Some(f64::MAX);
		}
	}

	if z.norm_sqr() > 4.0 {
		Some(z.norm())
	} else {
		None
	}
}

/// Returns the absolute value of z after n iterations, or None if belongs to the set
fn mandelbrot_iterate_bool(mut z: Complex64, n: u32) -> bool {
	let initial = z;

	if z.norm_sqr() > 4f64 {
		return false
	}

	for _ in 0..n {
		z = z * z + initial;

		if z.norm_sqr() > 4f64 {
			return false
		}
	}

	true
}

fn squash(x: f64, a: f64) -> f64 {
	x / (x + a)
}

fn main() {
	let plane_view = ComplexPlaneView {
		center: 0.0.into(),
		units_per_pixel: 4.0 / 1920.0,
		rotation: 0.0,
		width: 1920,
		height: 1080,
	};

	let grid = plane_view
		.gen_grid()
		.map(|z| mandelbrot_iterate(z, 20))
		.map(|x_opt| {
			if let Some(x) = x_opt {
				((1.0 - squash(x, 100.0)) * 255.0).floor() as u8
			} else { 0 }
		});

	let writer = &mut std::io::stdout();

	let encoder = PngEncoder::new(writer);

	encoder.write_image(
		&grid.data,
		1920,
		1080,
		ColorType::L8.into(),
	).unwrap();
}

