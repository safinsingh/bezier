use crate::easing::{Easing, Linear};

/// General 2D point structure
pub struct Point {
	/// X-coordinate
	pub x: f64,
	/// Y-coordinate
	pub y: f64,
}

impl Point {
	/// See [`Point::interpolate_weighted`] The rate function is set to linear.
	pub fn interpolate(this: &Point, other: &Point, time: f64) -> Point {
		Point::interpolate_weighted(this, other, time, Linear)
	}

	/// Interpolate position of point when transforming to another one given the
	/// progress of the interpolation and the easing fuction.
	pub fn interpolate_weighted<E: Easing>(
		this: &Point,
		other: &Point,
		time: f64,
		easing: E,
	) -> Point {
		let progress = easing.ease(time);
		let x = this.x + (other.x - this.x) * progress;
		let y = this.y + (other.y - this.y) * progress;

		Point { x, y }
	}
}
