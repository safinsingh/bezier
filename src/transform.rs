use crate::bezier::Bezier;
use crate::easing::{Easing, Linear};

/// Curve transformation between two bezier curves
pub struct CurveTransform {
	/// Bezier curve to transform from
	pub curve1: Bezier,
	/// Bezier curve to transform to
	pub curve2: Bezier,
}

impl CurveTransform {
	/// See [`CurveTransform::interpolate_weighted`] The rate function is set to
	/// linear.
	pub fn interpolate(&self, time: f64) -> Bezier {
		self.interpolate_weighted(time, Linear)
	}

	/// Interpolate position of bezier curve when transforming to another one
	/// given the progress of the interpolation and the easing fuction.
	pub fn interpolate_weighted<E: Easing>(
		&self,
		time: f64,
		easing: E,
	) -> Bezier {
		Bezier::interpolate_weighted(&self.curve1, &self.curve2, time, easing)
	}
}
