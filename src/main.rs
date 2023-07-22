use anyhow::{anyhow, bail, Result};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use libanim::bezier::Bezier;
use libanim::easing::EaseInOutQuint;
use libanim::ffmpeg::Ffmpeg;
use libanim::point::Point;
use libanim::render::{CairoRenderer, Renderer};
use libanim::transform::CurveTransform;
use std::io::BufWriter;

fn main() -> Result<()> {
	let transform_length = 5.;
	let fps = 60;
	#[allow(clippy::cast_sign_loss)]
	let frames = (transform_length * f64::from(fps)) as u32;
	let (width, height) = (1920, 1080);

	let curve1 = Bezier {
		start: Point { x: 120., y: 350. },
		control: Point { x: 600., y: 660. },
		end: Point { x: 680., y: 200. },
	};
	let curve2 = Bezier {
		start: Point { x: 500., y: 650. },
		control: Point { x: 500., y: 460. },
		end: Point { x: 500., y: 200. },
	};
	let transform = CurveTransform { curve1, curve2 };

	let mut ff_handle = Ffmpeg::new(fps, width, height).spawn()?;
	let mut child_stdin_stream = BufWriter::new(
		ff_handle
			.stdin
			.take()
			.ok_or_else(|| anyhow!("failed to grab ffmpeg stdin handle"))?,
	);

	let mut renderer = CairoRenderer::new(width, height);
	let pb = ProgressBar::new(u64::from(frames)).with_style(
		ProgressStyle::default_bar()
			.template(
				"{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] \
				 {pos}/{len} ({eta})",
			)
			.progress_chars("#>-"),
	);

	for frame in (1..frames).progress_with(pb) {
		let pos = f64::from(frame) / f64::from(frames);
		let curve = transform.interpolate_weighted(pos, EaseInOutQuint);

		renderer.render_bezier(curve);
		renderer.emit_frame(&mut child_stdin_stream)?;
	}

	drop(child_stdin_stream.into_inner());

	let out = ff_handle.wait_with_output()?;
	if !out.status.success() {
		bail!("FFMPEG Error: {}", String::from_utf8_lossy(&out.stderr));
	}

	Ok(())
}
