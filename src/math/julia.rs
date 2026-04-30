use crate::cli::{GenOptions, Mode};
use crate::grid::Grid;
use crate::ComplexPlaneView;
use crate::utils;
use num::complex::Complex64;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

/// The Julia variant stores the coefficient
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum SetKind {
	Mandelbrot,
	Julia(Complex64),
}

impl SetKind {
	/// Takes a z value and returns c (used in z² + c): z itself for Mandelbrot,
	pub fn c_for(self, z: Complex64) -> Complex64 {
		match self {
			SetKind::Mandelbrot => z,
			SetKind::Julia(c) => c,
		}
	}

	pub fn generate(mode: Mode, gen_options: &GenOptions) -> Self {
		let GenOptions {
			zoom_buffer_size,
			zoom_iterations,
			..
		} = *gen_options;

		match mode {
			Mode::Mandelbrot => Self::Mandelbrot,
			Mode::Julia =>
				Self::Julia(gen_julia_coefficient(zoom_buffer_size, zoom_iterations)),

			Mode::Random => {
				let mode = [Mode::Mandelbrot, Mode::Julia]
					.choose(&mut rand::rng())
					.unwrap()
					.to_owned();

				Self::generate(mode, gen_options)
			}
		}
	}

	pub fn width(&self) -> f64 {
		match self {
			Self::Mandelbrot => 2.5,
			Self::Julia(_) => 4.0,
		}
	}
}

/// Returns the number of iterations the point has passed before going to infinity, or None if belongs to the set
pub fn iterate(set: SetKind, mut z: Complex64, n: u32) -> Option<u32> {
	let c = set.c_for(z);

	for i in 0..n {
		if z.norm_sqr() > 4.0 {
			return Some(i);
		}

		z = z * z + c;
	}

	None
}

/// Returns true if the point belongs to the set (after n iterations), else false
pub fn iterate_bool(set: SetKind, mut z: Complex64, n: u32) -> bool {
	if z.norm_sqr() > 4.0 {
		return false
	}

	let c = set.c_for(z);

	for _ in 0..n {
		z = z * z + c;

		if z.norm_sqr() > 4.0 {
			return false
		}
	}

	true
}

fn pick_border_point(grid: &Grid<bool>) -> Option<(u16, u16)> {
	let mut border_points: Vec<(u16, u16)> = Vec::new();

	for x in 0..grid.width {
		for y in 0..grid.height {
			// skips bottom row and right column, introducting some false negatives
			// but we can afford that
			if let (
				Some(this),
				Some(right),
				Some(down),
			) = (
				grid.get(x, y),
				grid.get(x + 1, y),
				grid.get(x, y + 1),
			) {
				if this != right || this != down {
					border_points.push((x, y));
				}
			}
		}
	}

	border_points.choose(&mut rand::rng()).copied()
}

fn gen_julia_coefficient(zoom_buffer_size: u16, zoom_iterations: u32) -> Complex64 {
	// the best coeffs for Julia set are near the Mandelbrot set boundary

	let plane_view = ComplexPlaneView::initial_mandelbrot(zoom_buffer_size);

	let grid = plane_view
		.gen_grid()
		.map(|z| iterate_bool(SetKind::Mandelbrot, z, zoom_iterations));

	let (x, y) = pick_border_point(&grid)
			.expect("no border points");

	plane_view.xy_to_point(x, y)
}

pub fn gen_border_point(
	set: SetKind,
	n_zooms: u8,
	gen_options: &GenOptions,
	
) -> Option<Complex64> {
	let GenOptions {
		zoom_factor,
		zoom_iterations,
		zoom_buffer_size,
		save_zoom_steps,
		..
	} = *gen_options;

	let mut plane_view = ComplexPlaneView::initial(set, zoom_buffer_size);

	for i in 0..n_zooms {
		let grid = plane_view
			.gen_grid()
			.par_map(|z| iterate_bool(set, z, zoom_iterations));

		if save_zoom_steps {
			utils::save_zoom_step(&grid, zoom_buffer_size, i);
		}

		let (x, y) = pick_border_point(&grid)?;

		plane_view.center = plane_view.xy_to_point(x, y);

		plane_view.units_per_pixel /= zoom_factor;
	}

	Some(plane_view.center)
}
