use crate::color_scheme::{self, Colors, ColorScheme};
use crate::grid::Grid;
use crate::{cli, image, utils};
use std::error::Error;

pub fn gradient(args: cli::SubcommandGradientArgs) -> Result<(), Box<dyn Error>> {
	let cli::SubcommandGradientArgs {
		color_spec,
		layout,
		file,
	} = args;

	let writer = utils::output_writer(&file);

	let colors = Colors::try_from(&color_spec)?;

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

	Ok(())
}
