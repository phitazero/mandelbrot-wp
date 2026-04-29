mod grid;
mod math;
mod image;
mod cli;
mod color_scheme;
mod utils;
mod subcommand;
mod seed;

use clap::Parser;
use cli::{Cli, Command};
use math::{ColorVec, ComplexPlaneView};

fn main() {
	let Cli {
		subcommand,
	} = Cli::parse();

	let result = match subcommand {
		Command::Generate(args) =>
			subcommand::generate(args),

		Command::Gradient(args) =>
			subcommand::gradient(args),

		Command::Log =>
			subcommand::log(),
	};

	result.unwrap_or_else(|err| {
		eprintln!("fatal: {err}");
		std::process::exit(1);
	});
}
