mod gradient;
mod color;
mod complex_plane_view;
pub mod julia;

pub use gradient::{Gradient, GradientColorSpace};
pub use color::ColorVec;
pub use complex_plane_view::ComplexPlaneView;

use std::ops::{Add, Mul};

pub fn inv_lerp(x: f64, min: f64, max: f64) -> f64 {
	(x - min) / (max - min)
}

pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
	T: Copy + Add<T, Output = T> + Mul<f64, Output = T>
{
	start * (1.0 - t) + end * t
}

pub fn cubic_smooth_step(t: f64) -> f64 {
	3.0 * t.powi(2) - 2.0 * t.powi(3)
}
