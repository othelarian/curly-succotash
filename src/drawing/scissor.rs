use nvg::{Context, Point, Extent, Rect, Color};
use nvg_gl::Renderer;


pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, t: f32
) -> Context<Renderer>{
    
    // Draw first rect and set scissor to it's area.
    ctx.translate(x, y);
    ctx.rotate((5 as f32).to_radians());
    ctx.save();
    ctx.begin_path();
    ctx.rect(Rect::new(
        Point::new(-20.0, -20.0),
        Extent::new(60.0, 40.0)
    ));
    ctx.fill_paint(Color::rgba_i(255, 0, 0, 255));
    ctx.fill().unwrap();
    ctx.scissor(Rect::new(
        Point::new(-20.0, -20.0),
        Extent::new(60.0,40.0))
    );

    // Draw second rectangle with ofset and rotation.

    ctx.translate(40.0, 0.0);
    ctx.rotate(t);

    let rect = Rect::new(Point::new(-20.0, -10.0), Extent::new(60.0, 30.0));

    // Draw the intended second rectangle without any scissoring.
    ctx.save();
    ctx.reset_scissor();
    ctx.begin_path();
    ctx.rect(rect.clone());
    ctx.fill_paint(Color::rgba_i(255, 128, 0, 64));
    ctx.fill().unwrap();
    ctx.restore();

    // Draw second rectangle with combined scissoring.
    ctx.intersect_scissor(rect.clone());
    ctx.begin_path();
    ctx.rect(rect.clone());
    ctx.fill_paint(Color::rgba_i(255, 128, 0, 255));
    ctx.fill().unwrap();
    ctx.restore();

    ctx
}