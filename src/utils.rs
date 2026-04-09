use crate::math::ColorVec;
use color_space::Rgb;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseColorError {
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

pub fn parse_hex_color(mut string: &str) -> Result<ColorVec<Rgb>, ParseColorError> {
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

impl ParseColorError {
	fn new(string: &str) -> Self {
		Self { string: string.to_string() }
	}
}

