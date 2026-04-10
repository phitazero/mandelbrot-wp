use crate::ColorVec;
use crate::{math, utils};
use color_space::{Rgb, Lab};
use std::error::Error;
use std::ops::{Add, Mul};
use std::str::FromStr;
use std::fmt;

pub type GradientRgb = Gradient<ColorVec<Rgb>>;

#[derive(Debug)]
pub enum GradientColorSpace {
	Rgb(Gradient<ColorVec<Rgb>>),
	Lab(Gradient<ColorVec<Lab>>),
}

#[derive(Debug, Clone)]
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
				.position(|(lower, upper)| lower.pos <= pos && pos <= upper.pos)
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

impl FromStr for GradientRgb {
	type Err = GradientParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut gradient = Gradient::new();

		for positioned_color_str in s
			.split(';')
			.filter(|s| !s.is_empty())
		{
			let (pos_str, color_str) = positioned_color_str.split_once(':')
				.ok_or(GradientParseError::PositionedColorFormat(positioned_color_str.to_string()))?;

			let pos: f64 = pos_str.trim().parse()
				.map_err(|_| GradientParseError::ParseFloatError(pos_str.to_string()))?;

			if pos < 0.0 || pos > 1.0 {
				return Err(GradientParseError::PointOutOfBounds(pos));
			}

			let color_vec = utils::parse_hex_color(color_str.trim())?;

			let opt = gradient.insert(pos, color_vec);

			if opt.is_none() {
				return Err(GradientParseError::DuplicatePoint(pos));
			}
		}

		if let Some(missing) = gradient.has_missing_edge_point() {
			return Err(GradientParseError::MissingEdgePoint(missing));
		}

		Ok(gradient)
	}
}

#[derive(Debug)]
pub enum GradientParseError {
	MissingEdgePoint(u8), // 0 or 1
	DuplicatePoint(f64),
	ParseFloatError(String),
	PositionedColorFormat(String),
	PointOutOfBounds(f64),
	ParseColorError(utils::ParseColorError),
}

impl Error for GradientParseError {}
impl fmt::Display for GradientParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GradientParseError::MissingEdgePoint(missing) =>
				write!(f, "missing edge point at {missing}.0"),

			GradientParseError::DuplicatePoint(pos) =>
				write!(f, "point at {pos} is duplicate"),

			GradientParseError::ParseFloatError(float_str) =>
				write!(f, "couldn't parse '{float_str}' as float64"),

			GradientParseError::PositionedColorFormat(string) =>
				write!(f, "can't parse '{string}' as a pair of position and color. Expected 'POS: COLOR'"),

			GradientParseError::PointOutOfBounds(pos) =>
				write!(f, "can't place a color point at {pos}, out of bounds [0, 1]"),

			GradientParseError::ParseColorError(err) =>
				write!(f, "{err}"),
		}
	}
}

impl From<utils::ParseColorError> for GradientParseError {
	fn from(value: utils::ParseColorError) -> Self {
		GradientParseError::ParseColorError(value)
	}
}
