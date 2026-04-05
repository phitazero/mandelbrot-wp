mod complex_plane_view;
mod grid;
mod math;
mod image;
mod cli;

use std::process::exit;
use std::fs;
use clap::Parser;
use complex_plane_view::ComplexPlaneView;
use cli::Cli;

fn main() {
	let Cli {
		zoom_buffer_size,
		output_width,
		output_height,
		min_zooms,
		max_zooms,
		zoom_factor,
		save_zoom_steps,
		zoom_iterations,
		iterations,
		..
	} = Cli::parse();

	let n_zooms = rand::random_range(min_zooms..=max_zooms);

	// the initial plane view
	// the whole set fits between x = -2 and x = 0.5
	// the center will be at x = -0.75
	// so zoom_buffer_size pixels map to 2.5 units
	let mut plane_view = ComplexPlaneView {
		center: (-0.75).into(),
		units_per_pixel: 2.5 / zoom_buffer_size as f64,
		rotation: 0.0,
		width: zoom_buffer_size,
		height: zoom_buffer_size,
	};

	for i in 0..n_zooms {
		let grid = plane_view
			.gen_grid()
			.map(|z| math::mandelbrot_iterate_bool(z, zoom_iterations));

		if save_zoom_steps {
			let file = fs::File::create(format!("mandelbrot_zoom_iteration_{i}.png"))
				.unwrap_or_else(|err| {
					eprintln!("fatal: couldn't create/open to 'mandelbrot_zoom_iteration_{i}.png': {err}");
					exit(1);
				});

			image::write_monochrome(
				file,
				zoom_buffer_size,
				zoom_buffer_size,
				&grid,
			);
		}

		let (x, y) = math::pick_border_point(&grid)
			.expect("no border points");

		plane_view.center = plane_view.xy_to_point(x, y);

		plane_view.units_per_pixel /= zoom_factor;
	}

	use std::f64::consts::TAU;
	plane_view.rotation = rand::random_range(0.0..TAU);

	plane_view.width = output_width;
	plane_view.height = output_height;

	let grid = plane_view
		.gen_grid()
		.map(|z| math::mandelbrot_iterate(z, iterations));

	let max = grid.data
		.iter()
		.flatten()
		.max()
		.unwrap();

	let max = *max as f64;

	let grid = grid.map(|x_opt| {
		if let Some(x) = x_opt {
			(x as f64 / max * 255.0) as u8
		} else {
			0
		}
	});

	let writer = std::io::stdout();

	image::write_grayscale(writer, output_width, output_height, &grid);
}

