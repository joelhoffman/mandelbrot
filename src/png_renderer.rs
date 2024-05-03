use image;
use image::ImageBuffer;
use colorgrad;
use colorgrad::Color;
use crate::frame::MandelbrotFrame;
use crate::renderer::Renderer;

pub struct PngRenderer {

}

impl PngRenderer {
    pub fn new() -> PngRenderer {
        PngRenderer {}
    }
}

impl Renderer for PngRenderer {
    fn dimensions(&self) -> (usize, usize) {
        (3000,3000)
    }

    fn render(&mut self, mut frame: MandelbrotFrame) {
        let mut imgbuf=ImageBuffer::new(frame.width as u32,frame.height as u32);
        let gradient = colorgrad::inferno();
        let vec = gradient.colors(frame.iter_max as usize);

        let iter_max = frame.iter_max;
        let pixel_fn = |x, y, iter| {
            let color = if iter >= iter_max {
                image::Rgb([0, 0, 0])
            } else {
                let x1: &Color = &vec[iter as usize];
                image::Rgb([(x1.r * 255.0) as u8, (x1.g * 255.0) as u8, (x1.b * 255.0) as u8])
            };
            imgbuf.put_pixel(x as u32, y as u32, color);
            Ok::<(), &str>(())
        };

        frame.compute(pixel_fn).expect("unexpected error");

        imgbuf.save("mandelbrot.png").unwrap();
    }
}