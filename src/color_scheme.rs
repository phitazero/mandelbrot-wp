use crate::math::{self, ColorVec, Gradient, GradientColorSpace};
use crate::cli::ColorSchemeOptions;
use crate::utils;
use color_space::Rgb;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug)]
pub struct ColorScheme {
	gradient: GradientColorSpace,
	set_color: [u8; 3],
	interpolation: Interpolation,
}

/// only for cli
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

impl TryFrom<ColorSchemeOptions> for ColorScheme {
	type Error = Box<dyn Error>;

	fn try_from(options: ColorSchemeOptions) -> Result<Self, Self::Error> {
		let ColorSchemeOptions {
			color_spec,
			color_space,
			interpolation,
		} = options;

		let set_color = utils::parse_hex_color(&color_spec.set_color)?.finish();

		let gradient: Gradient<ColorVec<Rgb>> = color_spec.gradient.parse()?;

		let gradient_color_space = match color_space {
			ColorSpace::Rgb => GradientColorSpace::Rgb(gradient),
			ColorSpace::Lab => GradientColorSpace::Lab(gradient.to_lab()),
		};

		Ok(ColorScheme {
			gradient: gradient_color_space,
			set_color,
			interpolation,
		})
	}
}

impl ColorScheme {
	// idk how to name the argument, i give up
	pub fn get_at(&self, value_opt: Option<f64>) -> [u8; 3] {
		let Some(value) = value_opt else { return self.set_color; };

		match self.interpolation {
			Interpolation::Linear => self.gradient.get_at(value),
			Interpolation::Cubic =>
				self.gradient.get_at(math::cubic_smooth_step(value)),
		}
	}
}
