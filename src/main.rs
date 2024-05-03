use std::env;

use arguments;
use arguments::Arguments;

use crate::frame::MandelbrotFrame;
use crate::png_renderer::PngRenderer;
use crate::renderer::Renderer;
use crate::text_renderer::TextRenderer;

pub mod frame;
mod png_renderer;
pub mod renderer;
pub mod text_renderer;

fn main() {
    let arguments = arguments::parse(env::args()).unwrap();

    let mut renderer = initialize(arguments);

    let (x, y) = renderer.dimensions();
    let frame = MandelbrotFrame::new(x, y);
    renderer.render(frame);
}

fn initialize(arguments: Arguments) -> Box<dyn Renderer> {
    let str = arguments.get::<String>("r").unwrap_or("text".to_string());
    let x = str.as_str();
    if x == "text" {
        return Box::new(TextRenderer::new());
    }
    if x == "png" {
        return Box::new(PngRenderer::new());
    }

    panic!("unknown renderer")
}
