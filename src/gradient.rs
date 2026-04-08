use crate::ColorVec;
use crate::math;
use color_space::{Rgb, Lab};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub enum GradientKind {
	Rgb(Gradient<ColorVec<Rgb>>),
	Lab(Gradient<ColorVec<Lab>>),
}

pub struct Gradient<T>
where
	T: Copy + Add<T, Output = T> + Mul<f64, Output = T>
{
	points: Vec<GradientPoint<T>>,
}

#[derive(Debug, Clone, Copy)]
struct GradientPoint<T>
where
	T: Copy + Add<T, Output = T> + Mul<f64, Output = T>
{
	pos: f64,
	value: T,
}

#[derive(Debug)]
struct Interval<T> {
	rel_pos: f64,
	lower: T,
	upper: T,
}

impl<T> Gradient<T>
where
	T: Copy + Add<T, Output = T> + Mul<f64, Output = T>
{
	fn get_interval(&self, pos: f64) -> Interval<T> {
		let (lower_point, upper_point) = self.points
			.windows(2)
			.map(|pair| (pair[0], pair[1]))
			.find(|(lower, upper)| lower.pos <= pos && pos <= upper.pos)
			.expect("in a gradient there should be at least 2 points, all ordered, together spanning [0; 1]");

		Interval {
			rel_pos: math::inv_lerp(
				pos,
				lower_point.pos,
				upper_point.pos,
			),
			lower: lower_point.value,
			upper: upper_point.value,
		}
	}
}

impl Gradient<ColorVec<Rgb>> {
	pub fn get_at(&self, pos: f64) -> [u8; 3] {
		let Interval {
			rel_pos,
			lower,
			upper,
		} = self.get_interval(pos);

		math::lerp(lower, upper, rel_pos).finish()
	}
}

impl Gradient<ColorVec<Lab>> {
	pub fn get_at(&self, pos: f64) -> [u8; 3] {
		let Interval {
			rel_pos,
			lower,
			upper,
		} = self.get_interval(pos);

		math::lerp(lower, upper, rel_pos).finish()
	}
}

impl GradientKind {
	pub fn get_at(&self, pos: f64) -> [u8; 3] {
		match self {
			GradientKind::Rgb(gradient) => gradient.get_at(pos),
			GradientKind::Lab(gradient) => gradient.get_at(pos),
		}
	}
}


// TO BE REMOVED:

impl Gradient<ColorVec<Rgb>> {
	pub fn test_new() -> Self {
		let points: Vec<GradientPoint<ColorVec<Rgb>>> = vec![
			GradientPoint {
				pos: 0.0,
				value: Rgb::from_hex(0xff00ff).into(),
			},
			GradientPoint {
				pos: 0.5,
				value: Rgb::from_hex(0x00ff00).into(),
			},
			GradientPoint {
				pos: 1.0,
				value: Rgb::from_hex(0x0000ff).into(),
			},
		];

		Gradient { points: points }
	}
}

impl Gradient<ColorVec<Lab>> {
	pub fn test_new() -> Self {
		let points: Vec<GradientPoint<ColorVec<Lab>>> = vec![
			GradientPoint {
				pos: 0.0,
				value: Lab::from(Rgb::from_hex(0xff00ff)).into(),
			},
			GradientPoint {
				pos: 0.5,
				value: Lab::from(Rgb::from_hex(0x00ff00)).into(),
			},
			GradientPoint {
				pos: 1.0,
				value: Lab::from(Rgb::from_hex(0x0000ff)).into(),
			},
		];

		Gradient { points: points }
	}
}
