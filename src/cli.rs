use clap::{Parser};

#[derive(Debug, Parser)]
#[command(disable_help_flag = true)] // to rebind help to -H
pub struct Cli {
	/// output file, - for stdout
	pub file: String,

	/// side length of the zoom buffer (a square)
	#[arg(short = 'b', long)]
	#[arg(default_value_t = 360)]
	pub zoom_buffer_size: u16,

	/// width of the output
	#[arg(short = 'w', long)]
	#[arg(requires = "output_height")]
	#[arg(default_value_t = 1920)]
	pub output_width: u16,

	/// height of the output
	#[arg(short = 'h', long)]
	#[arg(requires = "output_width")]
	#[arg(default_value_t = 1080)]
	pub output_height: u16,

	/// minimum number of zooms
	#[arg(short = 'm', long)]
	#[arg(default_value_t = 5)]
	pub min_zooms: u8,

	/// maximum number of zooms
	#[arg(short = 'M', long)]
	#[arg(default_value_t = 8)]
	pub max_zooms: u8,

	/// zoom factor
	#[arg(short = 'z', long)]
	#[arg(default_value_t = 1.7)]
	pub zoom_factor: f64,

	/// number of z² + c iterations when computing points to zoom into
	#[arg(short = 'i', long)]
	#[arg(default_value_t = 100)]
	pub zoom_iterations: u32,

	/// number of z² + c iterations when computing final image
	#[arg(short = 'I', long)]
	#[arg(default_value_t = 250)]
	pub iterations: u32,

	/// instead of Mandelbrot, generate a Julia set
	#[arg(short = 'J', long)]
	pub julia: bool,

	/// debug save zoom steps
	#[arg(short = 'D', long)]
	pub save_zoom_steps: bool,

	#[arg(short = 'H', long = "help", action = clap::ArgAction::Help)]
	_help: Option<bool>,
}
