use crate::frame::MandelbrotFrame;
use crate::renderer::Renderer;
use crate::text_renderer::TextRenderer;

pub mod frame;
pub mod renderer;
pub mod text_renderer;

use std::env;
use arguments;
use arguments::Arguments;

fn main() {
    let arguments = arguments::parse(env::args()).unwrap();

    let mut renderer = initialize(arguments);

    let (x, y) = renderer.dimensions();
    let frame = MandelbrotFrame::new(x, y);
    renderer.render(frame);
}

fn initialize(arguments: Arguments) -> &'static dyn Renderer {
    match arguments.get::<&str>("r")? {
        "text" => &TextRenderer::new(),
        // "png" => &PngRenderer::new(),
        _ => panic!("unknown renderer")
    }
}
