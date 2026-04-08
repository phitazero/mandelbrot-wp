use std::ops::{Add, Mul};
use std::convert::From;
use std::marker::PhantomData;
use color_space::{Rgb, Lab};

#[derive(Debug, Clone, Copy)]
pub struct ColorVec<T> (
	pub f64,
	pub f64,
	pub f64,
	PhantomData<T>,
);

impl ColorVec<Rgb> {
	pub fn finish(self) -> [u8; 3] {
		[
			self.0 as u8,
			self.1 as u8,
			self.2 as u8,
		]
	}
}

impl ColorVec<Lab> {
	pub fn finish(self) -> [u8; 3] {
		let lab: Lab = self.into();
		let rgb: Rgb = lab.into();
		let math_rgb: ColorVec<Rgb> = rgb.into();

		math_rgb.finish()
	}
}

impl From<Rgb> for ColorVec<Rgb> {
	fn from(value: Rgb) -> Self {
		Self (
			value.r,
			value.g,
			value.b,
			PhantomData,
		)
	}
}

impl From<Lab> for ColorVec<Lab> {
	fn from(value: Lab) -> Self {
		Self (
			value.l,
			value.a,
			value.b,
			PhantomData,
		)
	}
}

impl From<ColorVec<Rgb>> for Rgb {
	fn from(value: ColorVec<Rgb>) -> Self {
		Self {
			r: value.0,
			g: value.1,
			b: value.2,
		}
	}
}

impl From<ColorVec<Lab>> for Lab {
	fn from(value: ColorVec<Lab>) -> Self {
		Self {
			l: value.0,
			a: value.1,
			b: value.2,
		}
	}
}

impl<T> Add<ColorVec<T>> for ColorVec<T> {
	type Output = ColorVec<T>;

	fn add(self, other: ColorVec<T>) -> Self::Output {
		Self (
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
			PhantomData,
		)
	}
}

impl<T> Mul<f64> for ColorVec<T> {
	type Output = ColorVec<T>;

	fn mul(self, other: f64) -> Self::Output {
		Self (
			self.0 * other,
			self.1 * other,
			self.2 * other,
			PhantomData,
		)
	}
}
