/// Easing fuction trait
pub trait Easing: Copy {
	/// an easing function that returns the current
	/// offset from the beginning of a position given
	/// the current time. time is between `0` and `1`
	/// and this function must return a value between `0`
	/// and `1`.
	fn ease(&self, time: f64) -> f64;
}

/// Quartic easing
#[derive(Clone, Copy)]
pub struct EaseInOutQuart;
impl Easing for EaseInOutQuart {
	fn ease(&self, time: f64) -> f64 {
		if time < 0.5 {
			8. * time.powi(4)
		} else {
			1. - (-2. * time + 2.).powi(4) / 2.
		}
	}
}

/// Quintic easing
#[derive(Clone, Copy)]
pub struct EaseInOutQuint;
impl Easing for EaseInOutQuint {
	fn ease(&self, time: f64) -> f64 {
		if time < 0.5 {
			16. * time.powi(5)
		} else {
			1. - (-2. * time + 2.).powi(5) / 2.
		}
	}
}

/// Linear easing
#[derive(Clone, Copy)]
pub struct Linear;
impl Easing for Linear {
	fn ease(&self, time: f64) -> f64 { time }
}
