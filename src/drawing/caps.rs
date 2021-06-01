use nvg::{Color, Context, Extent, LineCap, Point, Rect};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, width: f32
) -> Context<Renderer> {
    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];
    let line_width = 8.0;
    ctx.save();
    ctx.begin_path();
    ctx.rect(Rect::new(
        Point::new(x-line_width/2.0, y),
        Extent::new(width+line_width, 40.0)
    ));
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 32));
    ctx.fill().unwrap();
    ctx.begin_path();
    ctx.rect(Rect::new(Point::new(x, y), Extent::new(width, 40.0)));
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 32));
    ctx.fill().unwrap();
    ctx.stroke_width(line_width);
    ctx.stroke_paint(Color::rgba_i(0, 0, 0, 255));
    for n in 0..3 {
        ctx.line_cap(caps[n]);
        ctx.begin_path();
        ctx.move_to(Point::new(x, y+ ((n*10+5) as f32)));
        ctx.line_to(Point::new(x+width, y+ (n*10+5) as f32));
        ctx.stroke().unwrap();
    }
    ctx.restore();
    ctx
}