mod grid;
mod math;
mod image;
mod cli;
mod color_scheme;
mod utils;

use std::process::exit;
use std::{fs, io};
use clap::Parser;
use cli::{Cli, GenOptions, Command};
use color_scheme::{ColorScheme, Colors};
use grid::Grid;
use num::complex::Complex64;
use std::f64::consts::TAU;
use math::{ColorVec, ComplexPlaneView};

/// The Julia variant stores the coefficient
#[derive(Debug, Clone)]
enum Mode {
	Mandelbrot,
	Julia(Complex64),
}

fn main() {
	Cli::parse();

	let Cli {
		file,
		subcommand,
	} = Cli::parse();

	let writer = output_writer(&file);

	match subcommand {
		Command::Generate(args) => subcommand_generate(writer, args),
		Command::Gradient(args) => subcommand_gradient(writer, args),
	}
}

fn output_writer(file: &str) -> Box<dyn io::Write> {
	if file == "-" {
		Box::new(io::stdout())
	} else {
		Box::new(create_file(&file))
	}
}

fn subcommand_gradient(
	writer: Box<dyn io::Write>,
	args: cli::SubcommandGradientArgs,
) {
	let cli::SubcommandGradientArgs {
		color_spec,
		layout,
	} = args;

	let colors = Colors::try_from(color_spec)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't parse colors: {err}");
			exit(1);
		});

	let scheme_lin_rgb = ColorScheme::from_colors(
		colors.clone(),
		color_scheme::ColorSpace::Rgb,
		color_scheme::Interpolation::Linear,
	);

	let scheme_lin_lab = ColorScheme::from_colors(
		colors.clone(),
		color_scheme::ColorSpace::Lab,
		color_scheme::Interpolation::Linear,
	);

	let scheme_cub_rgb = ColorScheme::from_colors(
		colors.clone(),
		color_scheme::ColorSpace::Rgb,
		color_scheme::Interpolation::Cubic,
	);

	let scheme_cub_lab = ColorScheme::from_colors(
		colors,
		color_scheme::ColorSpace::Lab,
		color_scheme::Interpolation::Cubic,
	);

	const SET_COLOR_WIDTH: u16 = 100;
	const GRADIENT_WIDTH: u16 = 500;
	const GRADIENT_HEIGHT: u16 = 175;

	const GRADIENT_HEIGHT_0: u16 = 0;
	const GRADIENT_HEIGHT_1: u16 = GRADIENT_HEIGHT;
	const GRADIENT_HEIGHT_2: u16 = GRADIENT_HEIGHT * 2;
	const GRADIENT_HEIGHT_3: u16 = GRADIENT_HEIGHT * 3;

	let grid = Grid::new(
		GRADIENT_WIDTH + SET_COLOR_WIDTH,
		4 * GRADIENT_HEIGHT,
		|x, y| {
			if x <= GRADIENT_WIDTH {
				let t = Some(x as f64 / GRADIENT_WIDTH as f64);

				match layout {
					cli::GradientCompareLayout::ColorSpace =>
						match y {
							GRADIENT_HEIGHT_0..GRADIENT_HEIGHT_1 =>
								&scheme_lin_rgb,

							GRADIENT_HEIGHT_1..GRADIENT_HEIGHT_2 =>
								&scheme_lin_lab,

							GRADIENT_HEIGHT_2..GRADIENT_HEIGHT_3 =>
								&scheme_cub_rgb,

							_ => &scheme_cub_lab,
						}

					cli::GradientCompareLayout::Interpolation =>
						match y {
							GRADIENT_HEIGHT_0..GRADIENT_HEIGHT_1 =>
								&scheme_lin_rgb,

							GRADIENT_HEIGHT_1..GRADIENT_HEIGHT_2 =>
								&scheme_cub_rgb,

							GRADIENT_HEIGHT_2..GRADIENT_HEIGHT_3 =>
								&scheme_lin_lab,

							_ => &scheme_cub_lab,
						}
				}.get_at(t)

			} else {
				scheme_lin_rgb.get_at(None)
			}
		}
	);

	image::write_colored(
		writer,
		GRADIENT_WIDTH + SET_COLOR_WIDTH,
		4 * GRADIENT_HEIGHT,
		&grid
	);
}

fn subcommand_generate(
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

	let grid = gen_iterations_grid(&gen_options)
		.map_some(|n| n as f64);

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

	let grid = grid.map_some(|x| math::inv_lerp(x, min, max))
		.map(|x_opt| color_scheme.get_at(x_opt));

	image::write_colored(
		writer,
		gen_options.output_width,
		gen_options.output_height,
		&grid
	);
}

fn gen_iterations_grid(gen_options: &GenOptions) -> Grid<Option<u32>> {
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
		julia,
	} = *gen_options;

	let n_zooms = rand::random_range(min_zooms..=max_zooms);

	let mut plane_view;
	let mode;

	if julia {
		plane_view = ComplexPlaneView::initial_julia(zoom_buffer_size);

		let c = math::julia::gen_julia_coefficient(zoom_buffer_size, zoom_iterations);
		mode = Mode::Julia(c);

	} else {
		plane_view = ComplexPlaneView::initial_mandelbrot(zoom_buffer_size);
		mode = Mode::Mandelbrot;
	}

	for i in 0..n_zooms {
		let grid = plane_view.gen_grid();

		let grid = match &mode {
			Mode::Mandelbrot =>
				grid.map(|z| math::julia::iterate_bool(z, z, zoom_iterations)),

			Mode::Julia(c) =>
				grid.map(|z| math::julia::iterate_bool(z, *c, zoom_iterations)),
		};

		if save_zoom_steps {
			let file = create_file(&format!("mandelbrot_zoom_iteration_{i}.png"));

			image::write_monochrome(
				file,
				zoom_buffer_size,
				zoom_buffer_size,
				&grid,
			);
		}

		let (x, y) = math::julia::pick_border_point(&grid)
			.expect("no border points");

		plane_view.center = plane_view.xy_to_point(x, y);

		plane_view.units_per_pixel /= zoom_factor;
	}

	plane_view.rotation = rand::random_range(0.0..TAU);

	plane_view.width = output_width;
	plane_view.height = output_height;

	let grid = plane_view.gen_grid();

	match &mode {
		Mode::Mandelbrot =>
			grid.par_map(|z| math::julia::iterate(z, z, iterations)),

		Mode::Julia(c) =>
			grid.par_map(|z| math::julia::iterate(z, *c, iterations)),
	}
}

fn create_file(path: &str) -> fs::File {
	fs::File::create(path)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't create/open {path}: {err}");
			exit(1);
		})
}
