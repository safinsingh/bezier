use crate::easing::{Easing, Linear};
use crate::point::Point;

/// Quadratic bezier curve
pub struct Bezier {
	/// Curve start vertex
	pub start: Point,
	/// Curve control vertex
	pub control: Point,
	/// Curve end vertex
	pub end: Point,
}

impl Bezier {
	/// Calculates position based on the time using the parametric equation for
	/// quadratic Bezier curves.
	pub fn point_at(&self, time: f64) -> Point {
		let td = 1. - time;
		let t2 = time.powi(2);
		let td2 = td.powi(2);
		let t2t = 2. * td * time;

		Point {
			x: self.start.x * td2 + self.control.x * t2t + self.end.x * t2,
			y: self.start.y * td2 + self.control.y * t2t + self.end.y * t2,
		}
	}

	/// See [`Bezier::interpolate_weighted`]. The rate function is set to
	/// linear.
	pub fn interpolate(this: &Bezier, other: &Bezier, time: f64) -> Bezier {
		Bezier::interpolate_weighted(this, other, time, Linear)
	}

	/// Interpolate the bezier curve's position when transforming to another one
	/// given the current time and easing strategy.
	pub fn interpolate_weighted<E: Easing>(
		this: &Bezier,
		other: &Bezier,
		time: f64,
		easing: E,
	) -> Bezier {
		Bezier {
			start: Point::interpolate_weighted(
				&this.start,
				&other.start,
				time,
				easing,
			),
			control: Point::interpolate_weighted(
				&this.control,
				&other.control,
				time,
				easing,
			),
			end: Point::interpolate_weighted(
				&this.end, &other.end, time, easing,
			),
		}
	}
}
