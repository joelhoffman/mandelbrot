use crate::frame::MandelbrotFrame;

pub trait Renderer {
    fn dimensions() -> (i32,i32);
    fn render(&mut self, frame: MandelbrotFrame);
}