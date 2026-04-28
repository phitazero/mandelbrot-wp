use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(disable_help_flag = true)] // to rebind help to -H
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	Generate(SubcommandGenerateArgs),
	Gradient(SubcommandGradientArgs),
}

#[derive(Args, Debug)]
pub struct SubcommandGenerateArgs {
	/// output file, - for stdout
	pub file: String,

	#[command(flatten)]
	pub gen_options: GenOptions,

	#[command(flatten)]
	pub color_scheme_options: ColorSchemeOptions,
}


#[derive(Args, Debug)]
pub struct SubcommandGradientArgs {
	/// output file, - for stdout
	pub file: String,

	/// layout the gradients are arrenged in
	/// color-space - easier to compare color spaces:
	/// RGB,linear; LAB,linear; RGB,cubic; LAB,cubic
	/// interpolation - easier to compare interpolations:
	/// RGB,linear; RGB,cubic; LAB,linear; LAB,cubic
	#[arg(short = 'l', long, verbatim_doc_comment)]
	#[arg(default_value = "color-space")]
	pub layout: GradientCompareLayout,

	#[command(flatten)]
	pub color_spec: ColorSpec,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GradientCompareLayout {
	ColorSpace,
	Interpolation,
}

#[derive(Debug, Args)]
pub struct GenOptions {
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
	#[arg(default_value_t = 6)]
	pub min_zooms: u8,

	/// maximum number of zooms
	#[arg(short = 'M', long)]
	#[arg(default_value_t = 10)]
	pub max_zooms: u8,

	/// zoom factor
	#[arg(short = 'z', long)]
	#[arg(default_value_t = 3.0)]
	pub zoom_factor: f64,

	/// number of z² + c iterations when computing points to zoom into
	#[arg(short = 'n', long)]
	#[arg(default_value_t = 100)]
	pub zoom_iterations: u32,

	/// number of z² + c iterations when computing final image
	#[arg(short = 'N', long)]
	#[arg(default_value_t = 300)]
	pub iterations: u32,

	/// fractal to use (random means select randomly, not random noise)
	#[arg(short = 'g', long)]
	#[arg(default_value = "mandelbrot")]
	pub mode: Mode,

	/// debug save zoom steps
	#[arg(short = 'D', long)]
	pub save_zoom_steps: bool,	
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Mode {
	Mandelbrot,
	Julia,
	Random,
}

#[derive(Debug, Args)]
pub struct ColorSchemeOptions {
	#[command(flatten)]
	pub color_spec: ColorSpec,

	/// color space to interpolate in
	#[arg(short = 'C', long)]
	#[arg(default_value = "rgb")]
	pub color_space: crate::color_scheme::ColorSpace,

	/// interpolation: linear or cubic smooth step (f' = 0 at each data point)
	#[arg(short = 'I', long)]
	#[arg(default_value = "linear")]
	pub interpolation: crate::color_scheme::Interpolation,
}

#[derive(Debug, Args, Clone)]
pub struct ColorSpec {
	/// list of RGB colors and positions belonging to [0, 1], separated with semicolons
	/// # is omittable
	/// it's neccessary to provide edge points: at 0 and 1
	/// e. g. "0: #000000; 0.8: #ff77aa; 1: #ffffff"
	#[arg(short = 'G', long, verbatim_doc_comment)]
	#[arg(default_value = "0: #000000; 1: #ffffff")]
	pub gradient: String,

	/// color of the points belonging to the set
	#[arg(short = 'S', long)]
	#[arg(default_value = "000000")]
	pub set_color: String,
}
