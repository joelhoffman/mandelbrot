use crate::frame::MandelbrotFrame;
use rangemap::{range_map, RangeMap};
use crate::renderer::Renderer;

pub struct TextRenderer<'a> {
    mapping: RangeMap<u32, &'a str>,
}

impl Renderer for TextRenderer {
    fn dimensions() -> (i32, i32) {
        (120,60)
    }

    fn render(&mut self, mut frame: MandelbrotFrame) -> () {
        frame.compute();
        for row in frame.results.as_rows() {
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
