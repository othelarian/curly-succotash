use nvg::{Context, Gradient, Rect, Point, Extent, Color };
use nvg_gl::Renderer;

// #[path = "spinner.rs"]
// mod spinner;
// use crate::drawing::thumbnails::spinner::Spinner;

pub trait Graph {
    fn graph(&mut self, x: f32, y: f32, w: f32, h: f32, t: f32);
}

impl Graph for Context<Renderer> {
    fn graph(&mut self, x: f32, y: f32, w: f32, h: f32, t: f32) {
        let dx = w/5.0;
        let samples = [
            (1.0+(t*1.2345+(t*0.33457).cos()*0.44).sin())*0.5,
            (1.0+(t*0.68363+(t*1.3).cos()*1.55).sin())*0.5,
            (1.0+(t*1.1642+(t*0.33457).cos()*1.24).sin())*0.5,
            (1.0+(t*0.56345+(t*1.63).cos()*0.14).sin())*0.5,
            (1.0+(t*1.6245+(t*0.254).cos()*0.3).sin())*0.5,
            (1.0+(t*0.345+(t*0.03).cos()*0.6).sin())*0.5,
        ];
        let sx = [
            x+0.0*dx,
            x+1.0*dx,
            x+2.0*dx,
            x+3.0*dx,
            x+4.0*dx,
            x+5.0*dx,
        ];
        let sy = [
            y+h*samples[0]*0.8,
            y+h*samples[1]*0.8,
            y+h*samples[2]*0.8,
            y+h*samples[3]*0.8,
            y+h*samples[4]*0.8,
            y+h*samples[5]*0.8
        ];

        // Graph background
        let bg = Gradient::Linear {
            start: Point::new(x, y),
            end: Point::new(x, y + h),
            start_color: Color::rgba_i(0, 160, 192, 0),
            end_color: Color::rgba_i(0, 160, 192, 64)
        };
        self.begin_path();
        self.move_to(Point::new(sx[0], sy[0]));
        for i in 1..6 {
            self.bezier_to(
                (sx[i-1]+dx*0.5, sy[i-1]),
                (sx[i]-dx*0.5, sy[i]),
                (sx[i], sy[i])
            );
        }
        self.line_to((x+w, y+h));
        self.line_to((x, y+h));
        self.fill_paint(bg);
        self.fill().unwrap();

        // Graph line
        self.begin_path();
        self.move_to((sx[0], sy[0]+2.0));
        for i in 1..6 {
            self.bezier_to(
                (sx[i-1]+dx*0.5, sy[i-1]+2.0),
                (sx[i]-dx*0.5, sy[i]+2.0),
                (sx[i], sy[i]+2.0)
            );
        }
        self.stroke_paint(Color::rgba_i(0, 0, 0, 32));
        self.stroke_width(3.0);
        self.stroke().unwrap();

        self.begin_path();
        self.move_to((sx[0], sy[0]));
        for i in 1..6 {
            self.bezier_to(
                (sx[i-1]+dx*0.5, sy[i-1]),
                (sx[i]-dx*0.5, sy[i]),
                (sx[i], sy[i]+2.0)
            );
        }
        self.stroke_paint(Color::rgba_i(0, 160, 192, 255));
        self.stroke_width(3.0);
        self.stroke().unwrap();
        
        // Graph sample pos
        for i in 1..6 {
            let bg = Gradient::Radial {
                center: Point::new(sx[i], sy[i]+2.0),
                in_radius: 3.0,
                out_radius: 8.0,
                inner_color: Color::rgba_i(0, 0, 0, 32),
                outer_color: Color::rgba_i(0, 0, 0, 0)
            };
            self.begin_path();
            self.rect(Rect::new(
                Point::new(sx[i]-10.0, sy[i]-8.0),
                Extent::new(20.0, 20.0)
            ));
            self.fill_paint(bg);
            self.fill().unwrap();
        }

        self.begin_path();
        for i in 0..6 {
            self.circle(
                (sx[i], sy[i]),
                4.0
            )
        }
        self.fill_paint(Color::rgba_i(0, 160, 192, 255));
        self.fill().unwrap();
        self.begin_path();
        for i in 0..6 {
            self.circle(
                (sx[i], sy[i]),
                2.0
            )
        }
        self.fill_paint(Color::rgba_i(220, 220, 220, 255));
        self.fill().unwrap();

        self.stroke_width(1.0);
    }
}