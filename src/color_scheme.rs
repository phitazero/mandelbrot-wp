use crate::grid::Grid;
use crate::math::{self, GradientColorSpace, GradientRgb};
use crate::cli::{ColorSchemeOptions, ColorSpec};
use crate::utils;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug)]
pub struct ColorScheme {
	gradient: GradientColorSpace,
	interpolation: Interpolation,
	set_color: [u8; 3],
}

#[derive(Debug, Clone)]
pub struct Colors {
	gradient: GradientRgb,
	set_color: [u8; 3],
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize, Deserialize)]
pub enum ColorSpace {
	Rgb,
	Lab,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize, Deserialize)]
pub enum Interpolation {
	Linear,
	Cubic,
}

impl TryFrom<&ColorSchemeOptions> for ColorScheme {
	type Error = Box<dyn Error>;

	fn try_from(options: &ColorSchemeOptions) -> Result<Self, Self::Error> {
		let ColorSchemeOptions {
			color_spec,
			color_space,
			interpolation,
		} = options;

		let colors = Colors::try_from(color_spec)?;

		let gradient_color_space = match color_space {
			ColorSpace::Rgb => GradientColorSpace::Rgb(colors.gradient),
			ColorSpace::Lab => GradientColorSpace::Lab(colors.gradient.to_lab()),
		};

		Ok(ColorScheme {
			gradient: gradient_color_space,
			set_color: colors.set_color,
			interpolation: *interpolation,
		})
	}
}

impl ColorScheme {
	// idk how to name the argument, i give up
	pub fn get_at(&self, value_opt: Option<f64>) -> [u8; 3] {
		let Some(value) = value_opt else {
			return self.set_color;
		};

		match self.interpolation {
			Interpolation::Linear => self.gradient.get_at(value),
			Interpolation::Cubic =>
				self.gradient.get_at(math::cubic_smooth_step(value)),
		}
	}

	pub fn from_colors(
		colors: Colors,
		color_space: ColorSpace,
		interpolation: Interpolation,
	) -> Self {
		let gradient_color_space = match color_space {
			ColorSpace::Rgb => GradientColorSpace::Rgb(colors.gradient),
			ColorSpace::Lab => GradientColorSpace::Lab(colors.gradient.to_lab()),
		};

		Self {
			gradient: gradient_color_space,
			set_color: colors.set_color,
			interpolation,
		}
	}

	pub fn apply_to(&self, grid: Grid<Option<u32>>) -> Grid<[u8; 3]> {
		let mut grid = grid
			.map_some(|n| n as f64)
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

		grid.map(|x_opt| self.get_at(x_opt))
	}
}

impl TryFrom<&ColorSpec> for Colors {
	type Error = Box<dyn Error>;

	fn try_from(color_spec: &ColorSpec) -> Result<Self, Self::Error> {
		let set_color = utils::parse_hex_color(&color_spec.set_color)?.finish();

		let gradient: GradientRgb = color_spec.gradient.parse()?;

		Ok(Self {
			gradient,
			set_color,
		})
	}
}
