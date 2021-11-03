use nvg::{Context, Color, Align, Point};
use nvg_gl::Renderer;

#[path = "edit_box_base.rs"]
mod edit_box_base;

pub fn draw(
    mut ctx: Context<Renderer>,
    text: &str,
    x: f32, y:f32, w: f32, h: f32
) -> Context<Renderer> {

    ctx = edit_box_base::draw(ctx,x,y,w,h);

    ctx.font_size(17.0);
    ctx.font("sans");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 64));
    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.text(Point::new(x+h*0.3, y+h*0.5), text).unwrap();

    ctx
}