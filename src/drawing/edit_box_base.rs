use nvg::{Context, Color, Point, Gradient, Rect, Extent};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    x: f32, y:f32, w: f32, h: f32
) -> Context<Renderer> {
    let bg = Gradient::Box {
        rect: Rect::new(
            Point::new(x+1.0, y+1.0+1.5),
            Extent::new(w-2.0, h-2.0)
        ),
        radius: 3.0,
        feather: 4.0,
        inner_color: Color::rgba_i(255, 255, 255, 32),
        outer_color: Color::rgba_i(32, 32, 32, 32)
    };

    ctx.begin_path();
    ctx.rounded_rect(Rect::new(
        Point::new(x+1.0, y+1.0),
        Extent::new(w-2.0, h-2.0)
    ), 4.0-1.0);
    ctx.fill_paint(bg);
    ctx.fill().unwrap();

    ctx.begin_path();
    ctx.rounded_rect(Rect::new(
        Point::new(x+0.5, y+0.5),
        Extent::new(w-1.0, h-1.0)
    ), 4.0-0.5);
    ctx.stroke_paint(Color::rgba_i(0, 0, 0, 48));
    ctx.stroke().unwrap();

    ctx
}