use nvg::{Context, ImageId, Gradient, Rect, Point, Extent, Color, Solidity, ImagePattern};
use nvg_gl::Renderer;

#[path = "spinner.rs"]
mod spinner;
use crate::drawing::thumbnails::spinner::Spinner;

pub trait Thumbnails {
    fn thumbnails(&mut self, x: f32, y: f32, w: f32, h: f32, images: &Vec<ImageId>, t: f32);
}

impl Thumbnails for Context<Renderer> {
    fn thumbnails(&mut self, x: f32, y: f32, w: f32, h: f32, images: &Vec<ImageId>, t: f32) {
        let corner_radius = 3.0;
        let thumb = 60.0;
        let arry = 30.5;
        let stackh = (images.len() as f32 / 2.0 * (thumb+10.0) + 10.0).max(h);
        let u = (1.0+(t*0.5).cos())*0.5;
        let u2 = (1.0-(t*0.2).cos())*0.5;
        self.save();

        // Drop shadow
        let shadow_paint = Gradient::Box {
            rect: Rect::new(
                Point::new(x, y+4.0),
                Extent::new(w, h)
            ),
            radius: corner_radius * 2.0,
            feather: 20.0,
            inner_color: Color::rgba_i(0, 0, 0, 128),
            outer_color: Color::rgba_i(0, 0, 0, 0)
        };
        self.begin_path();
        self.rect(Rect::new(
            Point::new(x-10.0, y-10.0),
            Extent::new(w+20.0, h+30.0)
        ));
        self.rounded_rect(Rect::new(
            Point::new(x, y),
            Extent::new(w, h)
        ), corner_radius);
        self.path_solidity(Solidity::Hole);
        self.fill_paint(shadow_paint);
        self.fill().unwrap();

        // Window
        self.begin_path();
        self.rounded_rect(Rect::new(
            Point::new(x, y),
            Extent::new(w, h)
        ), corner_radius);
        self.move_to(Point::new(x-10.0, y+arry));
        self.line_to(Point::new(x+1.0, y+arry-11.0));
        self.line_to(Point::new(x+1.0, y+arry+11.0));
        self.fill_paint(Color::rgba_i(200, 200, 200, 255));
        self.fill().unwrap();

        self.save();
        self.scissor(Rect::new(
            Point::new(x, y),
            Extent::new(w, h)
        ));
        self.translate(0.0, -(stackh-h)*u);

        let dv = 1.0 / (images.len() as f32 - 1.0).max(1.0);

        for (i, image) in images.iter().enumerate() {
            let idx = i as f32;
            let tx = x+10.0 + idx % 2.0 * (thumb+10.0);
            let ty = y+10.0 + (idx as i32 / 2) as f32 * (thumb+10.0);
            
            let (imgw, imgh) = self.image_size(*image).unwrap();
            
            let (iw, ih, ix, iy) = if imgw < imgh {
                let ih = thumb * (imgh / imgw) as f32;
                (
                    thumb,
                    ih,
                    0.0,
                    -(ih-thumb)*0.5
                )
            } else {
                let iw = thumb * (imgw / imgh) as f32;
                (
                    iw,
                    thumb,
                    -(iw-thumb)*0.5,
                    0.0
                )
            };

            let v = idx * dv;
            let a = ((u2-v) / dv).clamp(0.0,1.0);

            if a < 1.0 {
                self.spinner(tx+thumb/2.0, ty+thumb/2.0, thumb*0.25, t);
            }

            let img_paint = ImagePattern {
                center: Point::new(tx+ix, ty+iy),
                size: Extent::new(iw, ih),
                angle: 0.0,
                img: *image,
                alpha: a
            };
            self.begin_path();
            self.rounded_rect(Rect::new(
                Point::new(tx, ty),
                Extent::new(thumb, thumb)
            ), 5.0);
            self.fill_paint(img_paint);
            self.fill().unwrap();

            let shadow_paint = Gradient::Box {
                rect: Rect::new(
                    Point::new(tx-1.0, ty),
                    Extent::new(thumb+2.0, thumb+2.0)
                ),
                radius: 5.0,
                feather: 3.0,
                inner_color: Color::rgba_i(0, 0, 0, 128),
                outer_color: Color::rgba_i(0, 0, 0, 0)
            };
            self.begin_path();
            self.rect(Rect::new(
                Point::new(tx-5.0, ty-5.0),
                Extent::new(thumb+10.0, thumb+10.0)
            ));
            self.rounded_rect(Rect::new(
                Point::new(tx, ty),
                Extent::new(thumb, thumb)
            ), 6.0);
            self.path_solidity(Solidity::Hole);
            self.fill_paint(shadow_paint);
            self.fill().unwrap();

            self.begin_path();
            self.rounded_rect(Rect::new(
                Point::new(tx+0.5, ty+0.5),
                Extent::new(thumb-1.0, thumb-1.0)
            ), 3.5);
            self.stroke_width(1.0);
            self.stroke_paint(Color::rgba_i(255, 255, 255, 192));
            self.stroke().unwrap();
        }
        self.restore();

        // Hide fades
        let fade_paint = Gradient::Linear {
            start: Point::new(x, y),
            end: Point::new(x, y+6.0),
            start_color: Color::rgba_i(200, 200, 200, 255),
            end_color: Color::rgba_i(200, 200, 200, 0)
        };
        self.begin_path();
        self.rect(Rect::new(
            Point::new(x+4.0, y),
            Extent::new(w-8.0, 6.0)
        ));
        self.fill_paint(fade_paint);
        self.fill().unwrap();

        let fade_paint = Gradient::Linear {
            start: Point::new(x, y+h),
            end: Point::new(x, y+h-6.0),
            start_color: Color::rgba_i(200, 200, 200, 255),
            end_color: Color::rgba_i(200, 200, 200, 0)
        };
        self.begin_path();
        self.rect(Rect::new(
            Point::new(x+4.0, y+h-6.0),
            Extent::new(w-8.0, 6.0)
        ));
        self.fill_paint(fade_paint);
        self.fill().unwrap();

        // Scroll bar
        let socket_paint = Gradient::Box {
            rect: Rect::new(
                Point::new(x+w-11.0, y+5.0),
                Extent::new(8.0, h-8.0)
            ),
            radius: 3.0,
            feather: 4.0,
            inner_color: Color::rgba_i(0, 0, 0, 32),
            outer_color: Color::rgba_i(0, 0, 0, 92)
        };
        self.begin_path();
        self.rounded_rect(Rect::new(
            Point::new(x+w-12.0, y+4.0),
            Extent::new(8.0, h-8.0)
        ), 3.0);
        self.fill_paint(socket_paint);
        self.fill().unwrap();

        let scrollh = (h/stackh) * (h-8.0);
        let bar_paint = Gradient::Box {
            rect: Rect::new(
                Point::new(x+w-11.0, y+3.0+(h-8.0-scrollh)*u),
                Extent::new(8.0, scrollh)
            ),
            radius: 3.0,
            feather: 4.0,
            inner_color: Color::rgba_i(220, 220, 220, 255),
            outer_color: Color::rgba_i(128, 128, 128, 255)
        };
        self.begin_path();
        self.rounded_rect(Rect::new(
            Point::new(x+w-11.0, y+5.0+(h-8.0-scrollh)*u),
            Extent::new(6.0, scrollh-2.0)
        ), 2.0);
        self.fill_paint(bar_paint);
        self.fill().unwrap();
        
        self.restore();
    }
}