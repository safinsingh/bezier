use anyhow::Result;
use std::process::{Child, Command, Stdio};

/// FFMPEG abstraction
pub struct Ffmpeg {
	fps: u32,
	width: i32,
	height: i32,
}

impl Ffmpeg {
	/// Create a new [`Ffmpeg`] instance
	#[must_use]
	pub fn new(fps: u32, width: i32, height: i32) -> Self {
		Self { fps, width, height }
	}

	/// Spawn FFMPEG as a child process
	///
	/// # Errors
	/// See [`std::process::Command`]
	pub fn spawn(&self) -> Result<Child> {
		Command::new("ffmpeg")
			.args(&[
				"-y",
				"-an",
				"-f",
				"rawvideo",
				"-pix_fmt",
				"bgra",
				"-video_size",
				&format!("{}x{}", self.width, self.height),
				"-framerate",
				&self.fps.to_string(),
				"-i",
				"-",
				"-c:v",
				"libx264",
				"-pix_fmt",
				"yuv420p",
				"output.mp4",
			])
			.stdin(Stdio::piped())
			.stderr(Stdio::piped())
			.stdout(Stdio::null())
			.spawn()
			.map_err(Into::into)
	}
}
