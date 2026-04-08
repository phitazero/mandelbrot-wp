use crate::ColorVec;
use crate::math;
use color_space::{Rgb, Lab};
use std::ops::{Add, Mul};

#[derive(Debug)]
pub enum GradientColorSpace {
	Rgb(Gradient<ColorVec<Rgb>>),
	Lab(Gradient<ColorVec<Lab>>),
}

#[derive(Debug)]
pub struct Gradient<T>
where
	T: Copy + Add<T, Output = T> + Mul<f64, Output = T>
{
	points: Vec<GradientPoint<T>>,
}

#[derive(Debug, Clone, Copy)]
pub struct GradientPoint<T>
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
	pub fn new() -> Self {
		Self {
			points: Vec::new(),
		}
	}

	/// Returns Some() on success and None if the pos is already taken
	pub fn insert(&mut self, pos: f64, value: T) -> Option<()> {
		let is_pos_free = self.points
			.iter()
			.all(|point| point.pos != pos);

		if !is_pos_free {
			return None;
		}

		let to_insert = GradientPoint { pos, value };

		let idx = if self.points.len() == 0 {
			0
		} else if pos < self.points.first().unwrap().pos {
			0
		} else if pos > self.points.last().unwrap().pos {
			self.points.len()
		} else {
			self.points
				.windows(2)
				.map(|pair| (pair[0], pair[1]))
				.position(|(lower, upper)| {eprintln!("{} {}", lower.pos, upper.pos); lower.pos <= pos && pos <= upper.pos})
				.unwrap() + 1 // shouldn't panic, as we're keeping self.points sorted by pos
		};

		self.points.insert(idx, to_insert);

		Some(())
	}

	/// Returns None if none are missing, else - Some(0) or Some(1), depending on what edge point is missing
	pub fn has_missing_edge_point(&self) -> Option<u8> {
		if self.points.len() == 0 {
			Some(0)
		}

		else if self.points.first().unwrap().pos != 0.0 {
			Some(0)
		}

		else if self.points.last().unwrap().pos != 1.0 {
			Some(1)
		}

		else { None }
	}

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

	pub fn to_lab(self) -> Gradient<ColorVec<Lab>> {
		let Gradient { points } = self;

		let points: Vec<GradientPoint<ColorVec<Lab>>> = points
			.into_iter()
			.map(|point| GradientPoint {
				pos: point.pos,
				value: point.value.into(),
			})
			.collect();

		Gradient { points }
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

impl GradientColorSpace {
	pub fn get_at(&self, pos: f64) -> [u8; 3] {
		match self {
			GradientColorSpace::Rgb(gradient) => gradient.get_at(pos),
			GradientColorSpace::Lab(gradient) => gradient.get_at(pos),
		}
	}
}
