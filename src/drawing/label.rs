use nvg::{Context, Point, Color, Align};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    text: &str,
    x: f32,
    y:f32,
    #[allow(unused)]
    w: f32,
    h: f32
) -> Context<Renderer> {
    ctx.font_size(15.0);
    ctx.font("sans");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 128));

    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.text(Point::new(x, y+h*0.5), text).unwrap();

    ctx
}