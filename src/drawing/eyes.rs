use nvg::{Color, Context, Gradient, Point};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32, t: f32
) -> Context<Renderer> {
    let ex = w * 0.23;
    let ey = h * 0.5;
    let lx = x + ex;
    let ly = y + ey;
    let rx = x + w  - ex;
    let ry = y + ey;
    let bg1 = Gradient::Linear {
        start: Point::new(x, y + h * 0.5),
        end: Point::new(x + w * 0.1, y + h),
        start_color: Color::rgba_i(0, 0, 0, 32),
        end_color: Color::rgba_i(0, 0, 0, 16)
    };
    ctx.begin_path();
    ctx.ellipse(Point::new(lx + 3.0, ly + 16.0), ex, ey);
    ctx.ellipse(Point::new(rx + 3.0, ry + 16.0), ex, ey);
    ctx.fill_paint(bg1);
    ctx.fill().unwrap();
    let bg2 = Gradient::Linear {
        start: Point::new(x, y + h * 0.25),
        end: Point::new(x + w * 0.1, y + h),
        start_color: Color::rgba_i(220, 220, 220, 255),
        end_color: Color::rgba_i(128, 128, 128, 255)
    };
    ctx.begin_path();
    ctx.ellipse(Point::new(lx, ly), ex, ey);
    ctx.ellipse(Point::new(rx, ry), ex, ey);
    ctx.fill_paint(bg2);
    ctx.fill().unwrap();
    let mut dx = (mx - rx) / (ex * 10.0);
    let mut dy = (my - ry) / (ey * 10.0);
    let mut d = (dx*dx + dy*dy).sqrt();
    let br = (if ex < ey { ex } else { ey }) * 0.5;
    let blink = 1.0 - ((t*0.5).sin()).powf(200.0) * 0.8;
    if d > 1.0 { dx /= d; dy /= d; }
    dx *= ex*0.4;
    dy *= ey*0.5;
    ctx.begin_path();
    ctx.ellipse(
        Point::new(lx+dx, ly+dy+ey*0.25*(1.0-blink)),
        br, br as f32 *blink
    );
    ctx.fill_paint(Color::rgba_i(32, 32, 32, 255));
    ctx.fill().unwrap();
    dx = (mx - rx) / (ex * 10.0);
    dy = (my - ry) / (ey * 10.0);
    d = (dx*dx + dy*dy).sqrt();
    if d > 1.0 { dx /= d; dy /= d; }
    dx *= ex*0.4;
    dy *= ey*0.5;
    ctx.begin_path();
    ctx.ellipse(
        Point::new(rx+dx, ry+dy+ey*0.25*(1.0-blink)),
        br, br as f32 *blink
    );
    ctx.fill_paint(Color::rgba_i(32, 32, 32, 255));
    ctx.fill().unwrap();
    let gloss_l = Gradient::Radial {
        center: Point::new(lx-ex*0.25, ly-ey*0.5),
        in_radius: ex*0.1,
        out_radius: ex*0.75,
        inner_color: Color::rgba_i(255, 255, 255, 128),
        outer_color: Color::rgba_i(255, 255, 255, 0)
    };
    ctx.begin_path();
    ctx.ellipse(Point::new(lx, ly), ex, ey);
    ctx.fill_paint(gloss_l);
    ctx.fill().unwrap();
    let gloss_r = Gradient::Radial {
        center: Point::new(rx-ex*0.25, ry-ey*0.5),
        in_radius: ex*0.1,
        out_radius: ex*0.75,
        inner_color: Color::rgba_i(255, 255, 255, 128),
        outer_color: Color::rgba_i(255, 255, 255, 0)
    };
    ctx.begin_path();
    ctx.ellipse(Point::new(rx, ry), ex, ey);
    ctx.fill_paint(gloss_r);
    ctx.fill().unwrap();
    ctx
}
