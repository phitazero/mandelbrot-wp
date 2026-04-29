use crate::grid::Grid;
use crate::math::ColorVec;
use crate::image;
use color_space::Rgb;
use std::error::Error;
use std::path::PathBuf;
use std::{fmt, fs, io};

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

pub fn create_file(path: &str) -> fs::File {
	fs::File::create(path)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't create/open {path}: {err}");
			std::process::exit(1);
		})
}

pub fn save_zoom_step(
	grid: &Grid<bool>,
	zoom_buffer_size: u16,
	i: u8,
) {
	let file = create_file(
		&format!("mandelbrot_zoom_iteration_{i}.png")
	);

	image::write_monochrome(
		file,
		zoom_buffer_size,
		zoom_buffer_size,
		&grid,
	);
}

pub fn output_writer(file: &str) -> Box<dyn io::Write> {
	if file == "-" {
		Box::new(io::stdout())
	} else {
		Box::new(create_file(&file))
	}
}

pub fn cache_dir() -> Result<PathBuf, String> {
	dirs::cache_dir()
		.ok_or_else(|| String::from("couldn't find cache dir"))
		.map(|path| path.join("mandelbrot-wp"))
}
