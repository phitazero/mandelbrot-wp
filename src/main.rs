mod complex_plane_view;
mod grid;
mod math;
mod image;
mod cli;

use std::process::exit;
use std::{fs, io};
use clap::Parser;
use complex_plane_view::ComplexPlaneView;
use cli::Cli;
use num::complex::Complex64;
use std::f64::consts::TAU;

/// The Julia variant stores the coefficient
#[derive(Debug, Clone)]
enum Mode {
	Mandelbrot,
	Julia(Complex64),
}

fn main() {
	let Cli {
		file,
		zoom_buffer_size,
		output_width,
		output_height,
		min_zooms,
		max_zooms,
		zoom_factor,
		save_zoom_steps,
		zoom_iterations,
		iterations,
		julia,
		..
	} = Cli::parse();

	let n_zooms = rand::random_range(min_zooms..=max_zooms);

	let mut plane_view;
	let mode;

	if julia {
		plane_view = ComplexPlaneView::initial_julia(zoom_buffer_size);

		let c = math::gen_julia_coefficient(zoom_buffer_size, zoom_iterations);
		mode = Mode::Julia(c);

	} else {
		plane_view = ComplexPlaneView::initial_mandelbrot(zoom_buffer_size);
		mode = Mode::Mandelbrot;
	}

	for i in 0..n_zooms {
		let grid = plane_view.gen_grid();

		let grid = match &mode {
			Mode::Mandelbrot =>
				grid.map(|z| math::julia_iterate_bool(z, z, zoom_iterations)),

			Mode::Julia(c) =>
				grid.map(|z| math::julia_iterate_bool(z, *c, zoom_iterations)),
		};

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

	plane_view.rotation = rand::random_range(0.0..TAU);

	plane_view.width = output_width;
	plane_view.height = output_height;

	let grid = plane_view.gen_grid();

	let grid = match &mode {
		Mode::Mandelbrot =>
			grid.map(|z| math::julia_iterate(z, z, iterations)),

		Mode::Julia(c) =>
			grid.map(|z| math::julia_iterate(z, *c, iterations)),
	};

	let max = *grid.data
		.iter()
		.flatten()
		.max()
		.unwrap() as f64;

	let min = *grid.data
		.iter()
		.flatten()
		.min()
		.unwrap() as f64;

	let writer: Box<dyn io::Write> =
		if file == "-" {
			Box::new(io::stdout())
		} else {
			Box::new(fs::File::create(&file).unwrap_or_else(|err| {
				eprintln!("fatal: couldn't open {file}: {err}");
				exit(1);
			}))
		};

	let grid = grid.map(|x_opt| {
		x_opt.map_or(
			0,
			|x| (math::inv_lerp(x as f64, min, max) * 255.0) as u8,
		)
	});

	image::write_grayscale(writer, output_width, output_height, &grid);
}
