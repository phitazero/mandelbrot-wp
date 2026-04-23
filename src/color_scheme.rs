use crate::math::{self, GradientColorSpace, GradientRgb};
use crate::cli::{ColorSchemeOptions, ColorSpec};
use crate::utils;
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

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ColorSpace {
	Rgb,
	Lab,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
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
