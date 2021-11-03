use nvg::{Context, Gradient, Rect, Color, Point, Extent, Align};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32
) -> Context<Renderer> {
    #[allow(non_snake_case)]
    let ICON_CHEVRON_RIGHT = "";
    let corner_radius = 4.0;
    let bg = Gradient::Linear {
        start: Point::new(x, y),
        end: Point::new(x, y+h),
        start_color: Color::rgba_i(255, 255, 255, 16),
        end_color: Color::rgba_i(0, 0, 0, 16)
    };

    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x+1.0, y+1.0),
            Extent::new(w-2.0, h-2.0)
    ), corner_radius-1.0);
    ctx.fill_paint(bg);
    ctx.fill().unwrap();

    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x+0.5, y+0.5),
            Extent::new(w-1.0, h-1.0)
    ), corner_radius-0.5);
    ctx.stroke_paint(Color::rgba_i(0, 0, 0, 48));
    ctx.stroke().unwrap();

    ctx.font_size(17.0);
    ctx.font("sans");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 160));
    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.text(Point::new(x+h*0.3, y+h*0.5), text).unwrap();

    ctx.font_size(h*1.3);
    ctx.font("icons");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 64));
    ctx.text_align(Align::CENTER|Align::MIDDLE);
    ctx.text(Point::new(x+w-h*0.5, y+h*0.5), ICON_CHEVRON_RIGHT).unwrap();

    ctx
}