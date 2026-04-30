use crate::seed::Seed;
use crate::utils;
use crate::color_scheme::{ColorSpace, Interpolation};
use std::error::Error;
use std::{fs, io};

pub fn log() -> Result<(), Box<dyn Error>> {
	let seed_cache_dir = utils::cache_dir()?;

	let read_dir = fs::read_dir(seed_cache_dir)
		.map_err(|err| format!("couldn't read dir: {err}"))?;

	let mut seeds: Vec<HashAndSeed> = Vec::new();
	let mut errors: Vec<String> = Vec::new();

	for result in read_dir {
		match process_dir_entry_result(result) {
			Ok(seed) => seeds.push(seed),
			Err(err) => errors.push(err),
		}
	}

	seeds.sort_by_key(|hs| hs.seed.created_at);
	seeds.iter().for_each(print_seed);

	errors.iter().for_each(|err| println!("error: {err}"));

	if seeds.len() == 0 && errors.len() > 0 {
		Err(String::from("no seed dumps could be read").into())
	} else {
		Ok(())
	}
}

fn process_dir_entry_result(
	result: io::Result<fs::DirEntry>
) -> Result<HashAndSeed, String> {
	let path = result
		.map_err(|err| format!("couldn't read dir entry: {err}"))?
		.path();

	let filename = path.file_name().unwrap().to_str().unwrap().to_string();

	Ok(HashAndSeed {
		seed: Seed::get_by_hash(&filename)?,
		hash: filename,
	})
}

fn print_seed(hash_and_seed: &HashAndSeed) {
	let HashAndSeed { hash, seed } = hash_and_seed;

	println!("[ {hash} ]");
	println!("Date: {}", seed.created_at.to_rfc2822());
	println!("Gradient: '{}'", seed.gradient);
	println!("Set color: '{}'", seed.set_color);
	println!("Iterations: {}", seed.iterations);
	println!(
		"{}, {}",
		match seed.color_space {
		    ColorSpace::Rgb => "RGB",
		    ColorSpace::Lab => "LAB",
		},
		match seed.interpolation {
		    Interpolation::Linear => "Linear",
		    Interpolation::Cubic => "Cubic",
		},
	);
	println!();
}

struct HashAndSeed {
	hash: String,
	seed: Seed,
}
