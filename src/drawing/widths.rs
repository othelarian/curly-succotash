use nvg::{Color, Context, Point};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, mut y: f32, width: f32
) -> Context<Renderer> {
    ctx.save();
    ctx.stroke_paint(Color::rgba_i(0, 0, 0, 255));
    for n in 0..20 {
        let w = (n as f32 + 0.5)*0.1;
        ctx.stroke_width(w);
        ctx.begin_path();
        ctx.move_to(Point::new(x, y));
        ctx.line_to(Point::new(x+width, y+width*0.3));
        ctx.stroke().unwrap();
        y += 10.0;
    }
    ctx.restore();
    ctx
}