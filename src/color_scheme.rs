use crate::math::{ColorVec, Gradient, GradientColorSpace};
use crate::cli::ColorSchemeOptions;
use color_space::Rgb;
use core::fmt;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug)]
pub struct ColorScheme {
	gradient: GradientColorSpace,
	set_color: [u8; 3],
}

/// only for cli
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ColorSpace {
	Rgb,
	Lab,
}

impl TryFrom<ColorSchemeOptions> for ColorScheme {
	type Error = Box<dyn Error>;

	fn try_from(options: ColorSchemeOptions) -> Result<Self, Self::Error> {
		let set_color = parse_hex_color(&options.set_color)?.finish();

		let mut gradient = Gradient::new();

		for positioned_color_str in options.gradient
			.split(';')
			.filter(|s| !s.is_empty())
		{
			let (pos_str, color_str) = positioned_color_str.split_once(':')
				.ok_or(GradientParseError::PositionedColorFormat(positioned_color_str.to_string()))?;

			let pos: f64 = pos_str.trim().parse()
				.map_err(|_| GradientParseError::ParseFloatError(pos_str.to_string()))?;

			let color_vec = parse_hex_color(color_str.trim())?;

			let opt = gradient.insert(pos, color_vec);

			if opt.is_none() {
				return Err(Box::new(GradientParseError::DuplicatePoint(pos)));
			}
		}

		if let Some(missing) = gradient.has_missing_edge_point() {
			return Err(Box::new(GradientParseError::MissingEdgePoint(missing)));
		}

		let gradient_color_space = match options.color_space {
			ColorSpace::Rgb => GradientColorSpace::Rgb(gradient),
			ColorSpace::Lab => GradientColorSpace::Lab(gradient.to_lab()),
		};

		Ok(ColorScheme {
			gradient: gradient_color_space,
			set_color,
		})
	}
}

impl ColorScheme {
	// idk how to name the argument, i give up
	pub fn get_at(&self, value_opt: Option<f64>) -> [u8; 3] {
		match value_opt {
			Some(value) => self.gradient.get_at(value),
			None => self.set_color
		}
	}
}

fn parse_hex_color(mut string: &str) -> Result<ColorVec<Rgb>, ParseColorError> {
	if string.len() == 0 {
		return Err(ParseColorError::new(string));
	}

	if string.chars().nth(0).unwrap() == '#' {
		string = &string[1..];
	}

	if string.len() != 6 {
		return Err(ParseColorError::new(string));
	}

	let hex_color = u32::from_str_radix(string, 16)
		.map_err(|_| ParseColorError::new(string))?;

	Ok(ColorVec::<Rgb>::from(Rgb::from_hex(hex_color)))
}

#[derive(Debug)]
struct ParseColorError {
	string: String,
}

impl Error for ParseColorError {}
impl fmt::Display for ParseColorError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"couldn't parse '{}' as a color. A 6-digit hex number is required, optionally followed by a #.",
			self.string
		)
	}
}

impl ParseColorError {
	fn new(string: &str) -> Self {
		Self { string: string.to_string() }
	}
}

#[derive(Debug)]
enum GradientParseError {
	MissingEdgePoint(u8), // 0 or 1
	DuplicatePoint(f64),
	ParseFloatError(String),
	PositionedColorFormat(String),
}

impl Error for GradientParseError {}
impl fmt::Display for GradientParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GradientParseError::MissingEdgePoint(missing) =>
				write!(f, "missing edge point at {missing}.0"),

			GradientParseError::DuplicatePoint(pos) =>
				write!(f, "point at {pos} is duplicate"),

			GradientParseError::ParseFloatError(float_str) =>
				write!(f, "couldn't parse '{float_str}' as float64"),

			GradientParseError::PositionedColorFormat(string) =>
				write!(f, "can't parse '{string}' as a pair of position and color. Expected 'POS: COLOR'")
		}
	}
}
