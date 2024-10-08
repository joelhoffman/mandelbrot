use std::env;

use arguments;
use arguments::Arguments;

use crate::fltk_renderer::FltkRenderer;
use crate::png_renderer::PngRenderer;
use crate::renderer::Renderer;
use crate::text_renderer::TextRenderer;

pub mod frame;
mod png_renderer;
pub mod renderer;
pub mod text_renderer;
mod fltk_renderer;

fn main() {
    let arguments = arguments::parse(env::args()).unwrap();

    let mut renderer = initialize(arguments);

    renderer.render();
}

fn initialize(arguments: Arguments) -> Box<dyn Renderer> {
    let str = arguments.get::<String>("r").unwrap_or("text".to_string());
    let x = str.as_str();

    match x {
        "text" => Box::new(TextRenderer::new()),
        "png" => Box::new(PngRenderer::new()),
        "gui" => Box::new(FltkRenderer::new()),
        _ => panic!("unknown renderer")
    }
}
