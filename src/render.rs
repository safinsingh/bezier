use crate::bezier::Bezier;
use crate::point::Point;
use crate::BEZIER_PRECISION_ROUNDS;
use anyhow::Result;
use cairo::{Context, Format, ImageSurface};
use std::io::{BufWriter, Write};

/// Generic renderer implementation
pub trait Renderer: Sized {
	/// Rendering context
	type Cx;

	/// Instantiate new renderer
	fn new(width: i32, height: i32) -> Self;
	/// Render bezier curve
	fn render_bezier(&self, curve: Bezier);
	/// Emit frame to some [`BufWriter`]
	///
	/// # Errors
	///
	/// See [`Write::write_all`]
	fn emit_frame<W: Write>(&mut self, writer: &mut BufWriter<W>)
		-> Result<()>;
}

/// Cairo-based renderer
pub struct CairoRenderer {
	surface: ImageSurface,
}

impl CairoRenderer {
	fn get_cx(&self) -> <CairoRenderer as Renderer>::Cx {
		let cx = Context::new(&self.surface);
		cx.set_source_rgb(1., 1., 1.);
		cx.paint();
		cx.set_line_width(1.);
		cx.set_source_rgb(0., 0., 0.);
		cx
	}
}

impl Renderer for CairoRenderer {
	type Cx = Context;

	fn new(width: i32, height: i32) -> Self {
		let surface =
			ImageSurface::create(Format::ARgb32, width, height).unwrap();
		Self { surface }
	}

	fn render_bezier(&self, curve: Bezier) {
		let cx = self.get_cx();
		for time in 0..BEZIER_PRECISION_ROUNDS {
			let time = f64::from(time) / f64::from(BEZIER_PRECISION_ROUNDS);
			let Point { x, y } = curve.point_at(time);

			cx.line_to(x, y);
			cx.move_to(x, y);
		}

		cx.stroke();
	}

	fn emit_frame<W: Write>(
		&mut self,
		writer: &mut BufWriter<W>,
	) -> Result<()> {
		let data = self.surface.get_data()?;
		writer.write_all(&data).map_err(Into::into)
	}
}
