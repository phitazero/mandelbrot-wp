use crate::color_scheme::ColorScheme;
use crate::cli::{self, GenOptions};
use crate::math::{self, ComplexPlaneView};
use crate::math::julia::SetKind;
use crate::grid::Grid;
use crate::image;
use crate::seed::Seed;
use num::complex::Complex64;
use std::process::exit;
use std::f64::consts::TAU;
use std::io;

pub fn generate(
	writer: Box<dyn io::Write>,
	args: cli::SubcommandGenerateArgs,
) {
	let cli::SubcommandGenerateArgs {
		gen_options,
		color_scheme_options,
	} = args;

	let color_scheme = ColorScheme::try_from(&color_scheme_options)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't parse color scheme: {err}");
			exit(1);
		});

	const N_MISSES_ALLOWED: u8 = 5;

	let grid_gen_result = (|| {
		for _ in 0..N_MISSES_ALLOWED {
			if let Some(grid) = gen_iterations_grid(&gen_options) {
				return grid;
			}

			eprintln!("Miss!")
		}

		eprintln!("fatal: couldn't locate any points in set after {N_MISSES_ALLOWED} attempts");
		eprintln!("may be caused by a low zoom buffer size");
		exit(1);
	})();

	let seed = Seed {
		created_at: chrono::Local::now(),
		width: gen_options.output_width,
		height: gen_options.output_height,
		center: grid_gen_result.center,
		total_zoom: grid_gen_result.total_zoom,
		rotation: grid_gen_result.rotation,
		set_kind: grid_gen_result.set_kind,
		gradient: color_scheme_options.color_spec.gradient,
		set_color: color_scheme_options.color_spec.set_color,
	};

	seed.save()
		.unwrap_or_else(|err| {
			eprintln!("error: couldn't save seed: {err}")
		});

	let mut grid = grid_gen_result.grid.map_some(|n| n as f64)
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

fn gen_iterations_grid(gen_options: &GenOptions) -> Option<GridGenResult> {
	let GenOptions {
		zoom_buffer_size,
		output_width,
		output_height,
		min_zooms,
		max_zooms,
		zoom_factor,
		iterations,
		mode,
		..
	} = *gen_options;

	let set_kind = SetKind::generate(mode, gen_options);

	let mut plane_view = ComplexPlaneView::initial(
		set_kind,
		zoom_buffer_size
	);

	let n_zooms = rand::random_range(min_zooms..=max_zooms);

	let center = math::julia::gen_border_point(
		set_kind,
		n_zooms,
		gen_options
	)?;

	plane_view.center = center;

	let total_zoom = zoom_factor.powi(n_zooms.into());
	plane_view.units_per_pixel /= total_zoom;

	let rotation = rand::random_range(0.0..TAU);
	plane_view.rotation = rotation;

	plane_view.width = output_width;
	plane_view.height = output_height;

	let grid = plane_view
		.gen_grid()
		.par_map(|z| math::julia::iterate(set_kind, z, iterations));

	Some(GridGenResult {
		grid,
		center,
		set_kind,
		rotation,
		total_zoom,
	})
}

#[derive(Debug)]
struct GridGenResult {
	grid: Grid<Option<u32>>,
	center: Complex64,
	set_kind: SetKind,
	rotation: f64,
	total_zoom: f64,
}
