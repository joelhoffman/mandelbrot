use crate::frame::MandelbrotFrame;
use rangemap::{range_map, RangeMap};

pub struct TextRenderer<'a> {
    frame: MandelbrotFrame,
    mapping: RangeMap<u32, &'a str>,
}

impl TextRenderer<'static> {
    pub fn new(frame: MandelbrotFrame) -> TextRenderer<'static> {
        TextRenderer {
            frame: frame,
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
    pub fn render(&mut self) -> () {
        self.frame.compute();
        for row in self.frame.results.as_rows() {
            println!(
                "{}",
                row.iter()
                    .map(|f| self.mapping.get(f).unwrap().to_string())
                    .collect::<String>()
            )
        }
    }
}
