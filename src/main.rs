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

	match subcommand {
		Command::Generate(args) =>
			subcommand::generate(args),

		Command::Gradient(args) =>
			subcommand::gradient(args),
	}
}
