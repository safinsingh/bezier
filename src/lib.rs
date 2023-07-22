#![warn(clippy::pedantic)]
#![deny(clippy::all, missing_docs)]
#![allow(
	clippy::module_name_repetitions,
	clippy::cast_possible_truncation,
	clippy::must_use_candidate
)]

//! A modular and high-performance 2D animation crate

/// Bezier curve utilities
pub mod bezier;
/// Easing functions
pub mod easing;
/// FFMPEG abstractions
pub mod ffmpeg;
/// Point system
pub mod point;
/// Rendering structures
pub mod render;
/// Transformation methods
pub mod transform;

/// Number of segments to render bezier curves with
pub const BEZIER_PRECISION_ROUNDS: i32 = 35;
