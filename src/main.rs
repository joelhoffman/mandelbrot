use crate::frame::MandelbrotFrame;
use crate::text_renderer::TextRenderer;

pub mod frame;
pub mod text_renderer;

fn main() {
    let frame = MandelbrotFrame::new(140,80);
    TextRenderer::new(frame).render();
}
