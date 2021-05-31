use nvg::{Color, Context, LineCap, LineJoin, Point};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, w: f32, _h: f32, t: f32
) -> Context<Renderer> {
    let pad = 5.0;
    let s = w/9.0 - pad*2.0;
    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];
    let joins = [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel];
    let mut pts = [0.0; 8];
    pts[0] = -s*0.25 + (t*0.3).cos() * s*0.5;
    pts[1] = (t*0.3).sin() * s*0.5;
    pts[2] = -s*0.25;
    pts[3] = 0.0;
    pts[4] = s*0.25;
    pts[5] = 0.0;
    pts[6] = s*0.25 + (t*0.3).cos() * s*0.5;
    pts[7] = (-t*0.3).sin() * s*0.5;
    ctx.save();
    for i in 0..3 {
        for j in 0..3 {
            let fx = x + s*0.5 + ((i*3+j) as f32)/9.0*w + pad;
            let fy = y - s*0.5 + pad;
            ctx.line_cap(caps[i]);
            ctx.line_join(joins[j]);
            ctx.stroke_width(s*0.3);
            ctx.stroke_paint(Color::rgba_i(0, 0, 0, 160));
            ctx.begin_path();
            ctx.move_to(Point::new(fx + pts[0], fy + pts[1]));
            ctx.line_to(Point::new(fx + pts[2], fy + pts[3]));
            ctx.line_to(Point::new(fx + pts[4], fy + pts[5]));
            ctx.line_to(Point::new(fx + pts[6], fy + pts[7]));
            ctx.stroke().unwrap();
            ctx.line_cap(LineCap::Butt);
            ctx.line_join(LineJoin::Bevel);
            ctx.stroke_width(1.0);
            ctx.stroke_paint(Color::rgb_i(0, 192, 255));
            ctx.begin_path();
            ctx.move_to(Point::new(fx + pts[0], fy + pts[1]));
            ctx.line_to(Point::new(fx + pts[2], fy + pts[3]));
            ctx.line_to(Point::new(fx + pts[4], fy + pts[5]));
            ctx.line_to(Point::new(fx + pts[6], fy + pts[7]));
            ctx.stroke().unwrap();
        }
    }
    ctx.restore();
    ctx
}
