use crate::grid::Grid;
use num::complex::Complex64;
use rand::seq::IndexedRandom;

/// Returns the absolute value of z after n iterations, or None if belongs to the set
pub fn mandelbrot_iterate(mut z: Complex64, n: u32) -> Option<f64> {
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
pub fn mandelbrot_iterate_bool(mut z: Complex64, n: u32) -> bool {
	let initial = z;

	if z.norm_sqr() > 4.0 {
		return false
	}

	for _ in 0..n {
		z = z * z + initial;

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

pub fn squash(x: f64, a: f64) -> f64 {
	x / (x + a)
}
