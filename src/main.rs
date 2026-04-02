mod complex_plane_view;
mod grid;
mod math;
mod image;

use complex_plane_view::ComplexPlaneView;

use num::complex::Complex64;

fn main() {
	let mut plane_view = ComplexPlaneView {
		center: 0.0.into(),
		units_per_pixel: 4.0 / 480.0,
		rotation: 0.0,
		width: 480,
		height: 360,
	};

	for _ in 0..5 {
		let grid = plane_view
			.gen_grid()
			.map(|z| math::mandelbrot_iterate_bool(z, 25));

		let (x, y) = math::pick_border_point(&grid).unwrap();
		plane_view.center = plane_view.xy_to_point(x, y);

		plane_view.units_per_pixel /= 2.0;
	}

	let grid = plane_view
		.gen_grid()
		.map(|z| math::mandelbrot_iterate_bool(z, 50));

	let writer = &mut std::io::stdout();

	image::write_monochrome(writer, 480, 360, &grid);
}

