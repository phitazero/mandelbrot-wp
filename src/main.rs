mod grid;
mod math;
mod image;
mod cli;
mod color_scheme;
mod utils;
mod subcommand;

use std::process::exit;
use std::{fs, io};
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
		Command::Generate(args) => subcommand::generate(writer, args),
		Command::Gradient(args) => subcommand::gradient(writer, args),
	}
}

fn output_writer(file: &str) -> Box<dyn io::Write> {
	if file == "-" {
		Box::new(io::stdout())
	} else {
		Box::new(create_file(&file))
	}
}



fn create_file(path: &str) -> fs::File {
	fs::File::create(path)
		.unwrap_or_else(|err| {
			eprintln!("fatal: couldn't create/open {path}: {err}");
			exit(1);
		})
}
