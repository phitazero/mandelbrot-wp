use crate::color_scheme::ColorScheme;
use crate::cli::{self, GenOptions, Mode};
use crate::math::{self, ComplexPlaneView};
use crate::grid::Grid;
use crate::image;
use num::complex::Complex64;
use rand::seq::IndexedRandom;
use std::process::exit;
use std::f64::consts::TAU;
use std::io;

/// The Julia variant stores the coefficient
#[derive(Debug, Clone, Copy)]
enum SetKind {
	Mandelbrot,
	Julia(Complex64),
}

pub fn generate(
	writer: Box<dyn io::Write>,
	args: cli::SubcommandGenerateArgs,
) {
	let cli::SubcommandGenerateArgs {
		gen_options,
		color_scheme_options,
	} = args;

	let color_scheme = ColorScheme::try_from(color_scheme_options)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't parse color scheme: {err}");
			exit(1);
		});

	const N_MISSES_ALLOWED: u8 = 5;

	let grid = (|| {
		for _ in 0..N_MISSES_ALLOWED {
			if let Some(grid) = gen_iterations_grid(&gen_options) {
				return grid;
			}

			eprintln!("Miss!")
		}

		eprintln!("fatal: couldn't locate any points in set after {N_MISSES_ALLOWED} attempts");
		eprintln!("may be cause by a low zoom buffer size");
		exit(1);
	})();

	let mut grid = grid.map_some(|n| n as f64)
		.map_some(|x| f64::log2(x + 1.0));

	let has_points_outside = grid.data
		.iter()
		.any(|opt| opt.is_some());

	if has_points_outside {
		let max = grid.data
			.iter()
			.flatten()
			.copied()
			.reduce(f64::max)
			.unwrap();

		let min = grid.data
			.iter()
			.flatten()
			.copied()
			.reduce(f64::min)
			.unwrap();

		grid = grid.map_some(|x| math::inv_lerp(x, min, max));
	}

	let grid = grid.map(|x_opt| color_scheme.get_at(x_opt));

	image::write_colored(
		writer,
		gen_options.output_width,
		gen_options.output_height,
		&grid
	);
}

fn gen_iterations_grid(gen_options: &GenOptions) -> Option<Grid<Option<u32>>> {
	let GenOptions {
		zoom_buffer_size,
		output_width,
		output_height,
		min_zooms,
		max_zooms,
		zoom_factor,
		save_zoom_steps,
		zoom_iterations,
		iterations,
		mode,
	} = *gen_options;

	let n_zooms = rand::random_range(min_zooms..=max_zooms);

	let (mut plane_view, set_kind) = initials(gen_options, mode);

	for i in 0..n_zooms {
		let grid = plane_view.gen_grid();

		let grid = match &set_kind {
			SetKind::Mandelbrot =>
				grid.map(|z| math::julia::iterate_bool(z, z, zoom_iterations)),

			SetKind::Julia(c) =>
				grid.map(|z| math::julia::iterate_bool(z, *c, zoom_iterations)),
		};

		if save_zoom_steps {
			let file = crate::create_file(
				&format!("mandelbrot_zoom_iteration_{i}.png")
			);

			image::write_monochrome(
				file,
				zoom_buffer_size,
				zoom_buffer_size,
				&grid,
			);
		}

		let (x, y) = math::julia::pick_border_point(&grid)?;

		plane_view.center = plane_view.xy_to_point(x, y);

		plane_view.units_per_pixel /= zoom_factor;
	}

	plane_view.rotation = rand::random_range(0.0..TAU);

	plane_view.width = output_width;
	plane_view.height = output_height;

	let grid = plane_view.gen_grid();

	let grid = match &set_kind {
		SetKind::Mandelbrot =>
			grid.par_map(|z| math::julia::iterate(z, z, iterations)),

		SetKind::Julia(c) =>
			grid.par_map(|z| math::julia::iterate(z, *c, iterations)),
	};

	Some(grid)
}

fn initials(gen_options: &GenOptions, mode: Mode) -> (ComplexPlaneView, SetKind) {
	let GenOptions {
		zoom_buffer_size,
		zoom_iterations,
		..
	} = *gen_options;

	match mode {
		Mode::Mandelbrot => {
			let plane_view = ComplexPlaneView::initial_mandelbrot(zoom_buffer_size);
			let set_kind = SetKind::Mandelbrot;

			(plane_view, set_kind)
		},
		Mode::Julia => {
			let plane_view = ComplexPlaneView::initial_julia(zoom_buffer_size);

			let c = math::julia::gen_julia_coefficient(
				zoom_buffer_size,
				zoom_iterations,
			);

			let set_kind = SetKind::Julia(c);

			(plane_view, set_kind)
		},
		Mode::Random => {
			let mode = [
				Mode::Mandelbrot,
				Mode::Julia
			].choose(&mut rand::rng())
				.copied()
				.unwrap();

			initials(gen_options, mode)
		}
	}
}
