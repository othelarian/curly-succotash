use nvg::{Context, Color, Align, Point, Gradient, Rect, Extent};
use nvg_gl::Renderer;

fn isBlack(col: Color)
{

	// if( col.r == 0.0f && col.g == 0.0f && col.b == 0.0f && col.a == 0.0f )
	// {
	// 	return 1;
	// }
	// return 0;
}

trait ColorUtil {
    fn is_black(self) -> bool;
}
impl ColorUtil for Color {
    fn is_black(self) -> bool {
        self.r == 0.0 && self.g == 0.0 && self.g == 0.0 && self.a == 0.0
    }
}

pub fn draw(
    mut ctx: Context<Renderer>,
    icon: Option<&str>,
    text: &str,
    x: f32, y:f32, w: f32, h: f32,
    color: Color
) -> Context<Renderer> {
    let corner_radius = 4.0;
    let text_font_size = 17.0;
    let icon_font_size = h*1.3;
    let iw = 25.0;
    let bg_alpha = if color.is_black() {16} else {32};
    let bg = Gradient::Linear {
        start: Point::new(x, y),
        end: Point::new(x, y+h),
        start_color: Color::rgba_i(255, 255, 255, bg_alpha),
        end_color: Color::rgba_i(0, 0, 0, bg_alpha)
    };

    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x+1.0, y+1.0),
            Extent::new(w-2.0, h-2.0)
        ), corner_radius-1.0);
    
    if !color.is_black() {
        ctx.fill_paint(color);
        ctx.fill().unwrap();
    }
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

    ctx.font_size(text_font_size);
    ctx.font("sans-bold");

    // TODO: implement nvgTextBounds from nanovg as a trait for Context
    // tw = nvgTextBounds(vg, 0,0, text, NULL, NULL);
    let tw = text.chars().count() as f32 * text_font_size/2.3; // TODO: replace when nvgTextBounds is implemented
    if let Some(icon) = icon {
        ctx.font_size(icon_font_size);
        ctx.font("icons");
        // iw = nvgTextBounds(vg, 0,0, cpToUTF8(preicon,icon), NULL, NULL);
        let iw = icon.chars().count() as f32 * icon_font_size/2.3; // TODO: replace when nvgTextBounds is implemented
        let iw = iw+h*0.15;
        ctx.fill_paint(Color::rgba_i(255, 255, 255, 96));
        ctx.text_align(Align::LEFT|Align::MIDDLE);
        ctx.text(Point::new(x+w*0.5-tw*0.5-iw*0.75, y+h*0.5), icon).unwrap();
    }

    ctx.font_size(text_font_size);
    ctx.font("sans-bold");
    ctx.text_align(Align::LEFT|Align::MIDDLE);
    ctx.fill_paint(Color::rgba_i(0, 0, 0, 160));
    ctx.text(
        Point::new(x+w*0.5-tw*0.5+iw*0.25, y+h*0.5-1.0),
        text
    ).unwrap();
    ctx.fill_paint(Color::rgba_i(255, 255, 255, 160));
    ctx.text(
        Point::new(x+w*0.5-tw*0.5+iw*0.25, y+h*0.5),
        text
    ).unwrap();

    ctx
}