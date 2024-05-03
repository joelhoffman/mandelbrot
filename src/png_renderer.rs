use colorgrad;
use image;
use image::{ImageBuffer, Rgb};

use crate::frame::MandelbrotFrame;
use crate::renderer::Renderer;

pub struct PngRenderer {}

impl PngRenderer {
    pub fn new() -> PngRenderer {
        PngRenderer {}
    }
}

impl Renderer for PngRenderer {
    fn dimensions(&self) -> (usize, usize) {
        (3000, 3000)
    }

    fn render(&mut self, mut frame: MandelbrotFrame) {
        let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> =
            ImageBuffer::new(frame.width as u32, frame.height as u32);
        let gradient = colorgrad::inferno();
        let vec = gradient.colors(frame.iter_max as usize);

        let iter_max = frame.iter_max;
        let pixel_fn = |x: usize, y: usize, iter: u32| {
            let color = if iter >= iter_max {
                Rgb([0u8, 0u8, 0u8])
            } else {
                let x1: &[u8; 4] = &vec[iter as usize].to_rgba8();
                Rgb([x1[0], x1[1], x1[2]])
            };
            imgbuf.put_pixel(x as u32, y as u32, color);
            Ok::<(), &str>(())
        };

        frame.compute(pixel_fn).expect("unexpected error");

        imgbuf.save("mandelbrot.png").unwrap();
    }
}
