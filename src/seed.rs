use crate::math::julia::SetKind;
use num::complex::Complex64;
use serde::{Deserialize, Serialize};
use chrono::{Local, DateTime};
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Seed {
	pub created_at: DateTime<Local>,
	pub width: u16,
	pub height: u16,
	pub center: Complex64,
	pub total_zoom: f64,
	pub rotation: f64,
	pub set_kind: SetKind,

	// maybe someday i'll implement a way to (de)serialize colors and gradients
	pub gradient: String,
	pub set_color: String,
}

impl Seed {
	/// doesn't actually save yet, wip
	pub fn save(&self) -> Result<(), Box<dyn Error>> {
		let json_string = serde_json::to_string_pretty(self)?;

		let mut hasher = blake3::Hasher::new();
		hasher.update(json_string.as_bytes());

		let hash_string = &hasher.finalize().to_hex()[..16];
		let filename = format!("{hash_string}.json");

		let seed_cache_dir = dirs::cache_dir()
			.ok_or_else(|| String::from("couldn't find cache dir"))?
			.join("mandelbrot-wp");

		if !seed_cache_dir.exists() {
			fs::create_dir_all(&seed_cache_dir)?;
		}

		let file_path = seed_cache_dir
			.join(filename);
		
		fs::write(file_path, &json_string)?;

		Ok(())
	}
}
