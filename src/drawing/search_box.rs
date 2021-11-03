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
    let ICON_SEARCH = "🔍";
    #[allow(non_snake_case)]
    let ICON_CIRCLED_CROSS = "✖";
    let icon_color = Color::rgba_i(255, 255, 255, 32);
    let icon_align = Align::CENTER|Align::MIDDLE;
    let corner_radius = h/2.1;
    let bg = Gradient::Box {
        rect: Rect::new(
            Point::new(x, y+1.5),
            Extent::new(w, h)
        ),
        radius: h/2.0,
        feather: 5.0,
        inner_color: Color::rgba_i(0, 0, 0, 16),
        outer_color: Color::rgba_i(0, 0, 0, 92)
    };

    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x, y),
            Extent::new(w, h)
    ), corner_radius);
    ctx.fill_paint(bg);
    ctx.fill().unwrap();

    ctx.font_size(h*1.3);
    ctx.font("icons");
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 64));
    ctx.text_align(icon_align.clone());
    ctx.text(Point::new(x+h*0.55, y+h*0.55), ICON_SEARCH).unwrap();
    
    ctx.font_size(17.0);
    ctx.font("sans");
    ctx.fill_paint(icon_color.clone());

    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.text(Point::new(x+h*1.05, y+h*0.5), text).unwrap();

    ctx.font_size(h*1.3);
    ctx.font("icons");
    ctx.fill_paint(icon_color.clone());
    ctx.text_align(icon_align.clone());
    ctx.text(Point::new(x+w-h*0.55, y+h*0.55), ICON_CIRCLED_CROSS).unwrap();

    ctx
}