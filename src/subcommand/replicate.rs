use crate::math::{self, ComplexPlaneView};
use crate::{utils, image, cli};
use crate::seed::Seed;
use crate::color_scheme::ColorScheme;
use std::error::Error;
use std::{fs, io};

pub fn replicate(
	args: cli::SubcommandReplicateArgs
) -> Result<(), Box<dyn Error>> {
	let cli::SubcommandReplicateArgs {
		file,
		hash,
		overrides,
	} = args;

	let writer = utils::output_writer(&file);

	let hash = match_hash(&hash)?;
	let seed = Seed::get_by_hash(&hash)?.override_with(overrides);

	let color_scheme_options = cli::ColorSchemeOptions::from(&seed);
	let color_scheme = ColorScheme::try_from(&color_scheme_options)?;

	let plane_view = ComplexPlaneView {
		center: seed.center,
		units_per_pixel: (seed.set_kind.width() / seed.width as f64) / seed.total_zoom,
		rotation: seed.rotation,
		width: seed.width,
		height: seed.height,
	};

	let grid = plane_view
		.gen_grid()
		.par_map(|z| math::julia::iterate(seed.set_kind, z, seed.iterations));

	let grid = color_scheme.apply_to(grid);

	image::write_colored(
		writer,
		seed.width,
		seed.height,
		&grid
	);

	Ok(())
}

fn match_hash(part_hash: &str) -> Result<String, String> {
	let seed_cache_dir = utils::cache_dir()?;

	let read_dir = fs::read_dir(seed_cache_dir)
		.map_err(|err| format!("couldn't read dir: {err}"))?;

	let mut hashes: Vec<String> = Vec::new();
	let mut errors: Vec<String> = Vec::new();

	for result in read_dir {
		match process_dir_entry_result(result) {
			Ok(seed) => hashes.push(seed),
			Err(err) => errors.push(err),
		}
	}

	errors.iter().for_each(|err| println!("error: {err}"));

	if hashes.len() == 0 && errors.len() > 0 {
		return Err(String::from("no seed dumps could be read"));
	}

	let mut matching_hashes: Vec<String> = hashes
		.iter()
		.filter(|hash| hash.starts_with(part_hash))
		.map(|s| s.to_string())
		.collect();

	if matching_hashes.len() == 0 {
		Err(format!("no seed hashes start with '{part_hash}'"))
	} else if matching_hashes.len() > 1 {
		Err(format!("ambiguous: mutiple seed hashes start with '{part_hash}'"))
	} else {
		Ok(matching_hashes.swap_remove(0))
	}
}

fn process_dir_entry_result(
	result: io::Result<fs::DirEntry>
) -> Result<String, String> {
	let path = result
		.map_err(|err| format!("couldn't read dir entry: {err}"))?
		.path();

	let hash = path.file_name().unwrap().to_str().unwrap().to_string();

	Ok(hash)
}
