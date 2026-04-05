use crate::grid::Grid;
use crate::ComplexPlaneView;
use num::complex::Complex64;
use rand::seq::IndexedRandom;

/// Returns the number of iterations the point has passed before going to infinity, or None if belongs to the set
pub fn julia_iterate(mut z: Complex64, c: Complex64, n: u32) -> Option<u32> {
	for i in 0..n {
		if z.norm_sqr() > 4.0 {
			return Some(i);
		}

		z = z * z + c;
	}

	None
}

/// Returns true if the point belongs to the set (after n iterations), else false
pub fn julia_iterate_bool(mut z: Complex64, c: Complex64, n: u32) -> bool {
	if z.norm_sqr() > 4.0 {
		return false
	}

	for _ in 0..n {
		z = z * z + c;

		if z.norm_sqr() > 4.0 {
			return false
		}
	}

	true
}

pub fn pick_border_point(grid: &Grid<bool>) -> Option<(u16, u16)> {
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

pub fn gen_julia_coefficient(zoom_buffer_size: u16, zoom_iterations: u32) -> Complex64 {
	// the best coeffs for Julia set are near the Mandelbrot set boundary

	let plane_view = ComplexPlaneView::initial_mandelbrot(zoom_buffer_size);

	let grid = plane_view
		.gen_grid()
		.map(|z| julia_iterate_bool(z, z, zoom_iterations));

	let (x, y) = pick_border_point(&grid)
			.expect("no border points");

	plane_view.xy_to_point(x, y)
}
