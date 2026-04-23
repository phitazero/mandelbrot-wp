mod grid;
mod math;
mod image;
mod cli;
mod color_scheme;
mod utils;
mod subcommand;
mod seed;

use std::io;
use clap::Parser;
use cli::{Cli, Command};
use math::{ColorVec, ComplexPlaneView};

fn main() {
	let Cli {
		file,
		subcommand,
	} = Cli::parse();

	let writer = output_writer(&file);

	match subcommand {
		Command::Generate(args) =>
			subcommand::generate(writer, args),

		Command::Gradient(args) =>
			subcommand::gradient(writer, args),
	}
}

fn output_writer(file: &str) -> Box<dyn io::Write> {
	if file == "-" {
		Box::new(io::stdout())
	} else {
		Box::new(utils::create_file(&file))
	}
}
