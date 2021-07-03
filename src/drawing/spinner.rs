use nvg::{Context, Gradient, Point, Color, ClockWise};
use nvg_gl::Renderer;
use std::f32::consts::PI;

pub trait Spinner {
    fn spinner(&mut self, cx: f32, cy: f32, r: f32, t: f32);
}

impl Spinner for Context<Renderer> {
    fn spinner(&mut self, cx: f32, cy: f32, r: f32, t: f32) {
        let a0 = 0.0 + t*6.0;
        let a1 = PI + t*6.0;
        let r0 = r;
        let r1 = r*0.75;


        self.save();

        self.begin_path();
        self.arc(Point::new(cx, cy), r0, a0, a1, ClockWise::CW);
        self.arc(Point::new(cx,  cy), r1, a1, a0, ClockWise::CCW);
        self.close_path();

        let ax = cx + a0.cos() * (r0+r1)*0.5;
        let ay = cy + a0.sin() * (r0+r1)*0.5;
        let bx = cx + a1.cos() * (r0+r1)*0.5;
        let by = cy + a1.sin() * (r0+r1)*0.5;
        let paint = Gradient::Linear {
            start: Point::new(ax, ay),
            end: Point::new(bx, by),
            start_color: Color::rgba_i(0, 0, 0, 0),
            end_color: Color::rgba_i(0, 0, 0, 128)
        };
        self.fill_paint(paint);
        self.fill().unwrap();

        self.restore();        
    }
}
