use std::cell::RefCell;
use std::rc::Rc;

use colorgrad;
use fltk::{
    app,
    draw::{draw_point, draw_rect_fill},
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    surface::ImageSurface,
    window::Window,
};
use fltk::draw::set_color_rgb;
use image;

use crate::frame::MandelbrotFrame;
use crate::renderer::Renderer;

struct Canvas {
    frame: Frame,
    #[allow(dead_code)]
    surf: Rc<RefCell<ImageSurface>>,
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Self {
        let mut frame = Frame::default().with_size(w, h).center_of_parent();
        frame.set_color(Color::White);
        frame.set_frame(FrameType::DownBox);

        let surf = ImageSurface::new(frame.width(), frame.height(), false);
        ImageSurface::push_current(&surf);
        let mut mandel = MandelbrotFrame::new(w as u32,h as u32);
        let iter_max=mandel.iter_max;
        let gradient = colorgrad::inferno();
        let vec = gradient.colors(mandel.iter_max as usize);

        let pixel_fn = |x: u32, y: u32, iter: u32| {
            if iter >= iter_max {
                set_color_rgb(0, 0, 0);
            } else {
                let x1: &[u8; 4] = &vec[iter as usize].to_rgba8();
                set_color_rgb(x1[0], x1[1], x1[2])
            };
            draw_point(x as i32, y as i32);
            Ok::<(), &str>(())
        };
        mandel.compute(pixel_fn).expect("TODO: panic message");

        ImageSurface::pop_current();

        let surf = Rc::from(RefCell::from(surf));

        frame.draw({
            let surf = surf.clone();
            move |f| {
                let surf = surf.borrow();
                let mut img = surf.image().unwrap();
                img.draw(f.x(), f.y(), f.w(), f.h());
            }
        });

        //
        // frame.handle({
        //     let mut x = 0;
        //     let mut y = 0;
        //     let surf = surf.clone();
        //     move |f, ev| {
        //         // println!("{}", ev);
        //         // println!("coords {:?}", app::event_coords());
        //         // println!("get mouse {:?}", app::get_mouse());
        //         let surf = surf.borrow_mut();
        //         match ev {
        //             Event::Push => {
        //                 ImageSurface::push_current(&surf);
        //                 set_draw_color(Color::Red);
        //                 set_line_style(LineStyle::Solid, 3);
        //                 let coords = app::event_coords();
        //                 x = coords.0;
        //                 y = coords.1;
        //                 draw_point(x, y);
        //                 ImageSurface::pop_current();
        //                 f.redraw();
        //                 true
        //             }
        //             Event::Drag => {
        //                 ImageSurface::push_current(&surf);
        //                 set_draw_color(Color::Red);
        //                 set_line_style(LineStyle::Solid, 3);
        //                 let coords = app::event_coords();
        //                 draw_line(x, y, coords.0, coords.1);
        //                 x = coords.0;
        //                 y = coords.1;
        //                 ImageSurface::pop_current();
        //                 f.redraw();
        //                 true
        //             }
        //             _ => false,
        //         }
        //     }
        // });
        Self { frame, surf }
    }
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fltk::widget_extends!(Canvas, Frame, frame);
pub struct FltkRenderer {}

impl FltkRenderer {
    pub fn new() -> FltkRenderer {
        FltkRenderer {}
    }
}

impl Renderer for FltkRenderer {
    fn render(&mut self) {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);

        let mut wind = Window::default()
            .with_size(WIDTH, HEIGHT)
            .with_label("Mandelbrot");

        Canvas::new(WIDTH - 10, HEIGHT - 10);

        wind.end();
        wind.show();

        app.run().unwrap();
    }
}

