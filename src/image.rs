use crate::grid::Grid;
use image_crate::{ColorType, ImageEncoder};
use image_crate::codecs::png::PngEncoder;
use std::io::Write;

pub fn write_monochrome<W>(
	writer: W,
	width: u16,
	height: u16,
	grid: &Grid<bool>
)
where
	W: Write
{
	let encoder = PngEncoder::new(writer);

	let data: Vec<u8> = grid.data
		.iter()
		.map(|b| if *b { 255 } else { 0 })
		.collect();

	encoder.write_image(
		&data,
		width as u32,
		height as u32,
		ColorType::L8.into(),
	).unwrap();
}
