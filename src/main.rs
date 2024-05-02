use crate::frame::MandelbrotFrame;
use crate::text_renderer::TextRenderer;

pub mod frame;
pub mod text_renderer;

fn main() {
    let frame = MandelbrotFrame::new(120,60);
    TextRenderer::new(frame).render();
}
