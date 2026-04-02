use num::complex::Complex64;
use crate::grid::Grid;

pub struct ComplexPlaneView {
	pub center: Complex64,
	pub units_per_pixel: f64,
	pub rotation: f64,
	pub width: u16,
	pub height: u16,
}

impl ComplexPlaneView {
	pub fn xy_to_point(&self, x: u16, y: u16) -> Complex64 {
		let x = x as i16;
		let y = y as i16;

		let half_width = self.width as i16 / 2;
		let half_height = self.height as i16 / 2;

		// offsets are in units on the complex plane
		let offset_x = (x - half_width) as f64 * self.units_per_pixel;
		// on the image y axis is directed downwards, on the complex plane - upwards
		// hence the negation
		let offset_y = -(y - half_height) as f64 * self.units_per_pixel;

		let offset = Complex64 {
			re: offset_x,
			im: offset_y,
		};

		let offset = offset * Complex64::cis(self.rotation);

		self.center + offset
	}

	pub fn gen_grid(&self) -> Grid<Complex64> {
		Grid::new(
			self.width,
			self.height,
			|x, y| self.xy_to_point(x, y),
		)
	}
}
