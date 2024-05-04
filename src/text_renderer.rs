use crate::frame::MandelbrotFrame;
use crate::renderer::Renderer;
use array2d::Array2D;
use rangemap::{range_map, RangeMap};

pub struct TextRenderer<'a> {
    mapping: RangeMap<u32, &'a str>,
}

impl Renderer for TextRenderer<'static> {
    fn render(&mut self) -> () {
        let (w, h) = (120, 60);
        let mut frame = MandelbrotFrame::new(w, h);

        let mut results = Array2D::filled_with(0, frame.height as usize, frame.width as usize);
        let x1 = |x: u32, y: u32, i| results.set(y as usize, x as usize, i);
        frame.compute(x1).unwrap();
        for row in results.as_rows() {
            println!(
                "{}",
                row.iter()
                    .map(|f| self.mapping.get(f).unwrap().to_string())
                    .collect::<String>()
            )
        }
    }
}
impl TextRenderer<'static> {
    pub fn new() -> TextRenderer<'static> {
        TextRenderer {
            mapping: range_map! {
                0..1 => "#",
                1..2 => "~",
                2..3 => "⬌",
                3..4 => "⬍",
                4..5 => "⨯",
                5..6 => "❖",
                6..8 => "✦",
                8..12 => "✧",
                12..20 => "✩",
                20..35 => "✴",
                35..70 => "⋅",
                70..1001 => " ",
                //✪✫✬✭✻⊗∼
            },
        }
    }
}
