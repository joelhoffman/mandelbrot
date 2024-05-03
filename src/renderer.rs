use crate::frame::MandelbrotFrame;

pub trait Renderer {
    fn dimensions(&self) -> (usize, usize);
    fn render(&mut self, frame: MandelbrotFrame);
}
