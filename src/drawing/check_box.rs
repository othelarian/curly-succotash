use nvg::{Context, Color, Align, Point, Gradient, Rect, Extent};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    text: &str,
    x: f32, y:f32,
    #[allow(unused)]
    w: f32, h: f32
) -> Context<Renderer> {
    #[allow(non_snake_case)]
    let ICON_CHECK = "✓";
    let bg = Gradient::Box {
        rect: Rect::new(
            Point::new(x+1.0, y+h*0.5-9.0+1.0),
            Extent::new(18.0, 18.0)
        ),
        radius: 3.0,
        feather: 3.0,
        inner_color: Color::rgba_i(0, 0, 0, 32),
        outer_color: Color::rgba_i(0, 0, 0, 92)
    };

    ctx.font_size(15.0);
    ctx.font("sans");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 160));
    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.text(Point::new(x+28.0, y+h*0.5), text).unwrap();

    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x+1.0, y+h*0.5-9.0+1.0),
            Extent::new(18.0, 18.0)
        ), 3.0);
    ctx.fill_paint(bg);
    ctx.fill().unwrap();

    ctx.font_size(33.0);
    ctx.font("icons");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 128));
    ctx.text_align(Align::CENTER|Align::MIDDLE);
    ctx.text(Point::new(x+9.0+2.0, y+h*0.5), ICON_CHECK).unwrap();

    ctx
}