mod complex_plane_view;
mod grid;

use complex_plane_view::ComplexPlaneView;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;

fn main() {
	let plane_view = ComplexPlaneView {
		center: 0.5.into(),
		units_per_pixel: 4f64 / 1920f64,
		rotation: 3.14 * 0.2,
		width: 1920,
		height: 1080,
	};

	let grid = plane_view
		.gen_grid()
		.map(|z| (z * z + z).powu(2) + z)
		.map(|z| z.norm())
		.map(|x| (255f64 / (x + 1f64)).floor() as u8);

	let image = image::GrayImage::from_raw(
		1920,
		1080,
		grid.data.to_vec(),
	).unwrap();

	let writer = &mut std::io::stdout();

	let encoder = PngEncoder::new(writer);

	encoder.write_image(
		image.as_raw(),
		1920,
		1080,
		ColorType::L8.into(),
	).unwrap();
}
