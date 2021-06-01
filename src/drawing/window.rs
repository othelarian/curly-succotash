use nvg::{Context, Rect, Point, Color, Gradient, Solidity, Align};
use nvg_gl::Renderer;

pub fn draw (
    mut ctx: Context<Renderer>,
    title: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32
) -> Context<Renderer> {
    let corner_radius = 3.0;

    // Window
    ctx.save();
    ctx.begin_path();
    ctx.rounded_rect(
        Rect::from((x, y, w, h)),
        corner_radius
    );
    ctx.fill_paint(Color::rgba_i(28, 30, 34, 192));
    ctx.fill().unwrap();

    // Drop Shadow
    let shadow_paint = Gradient::Box {
        rect: Rect::from((x, y+2.0, w, h)),
        radius: corner_radius*2.0,
        feather: 10.0,
        outer_color: Color::rgba_i(0, 0, 0, 0),
        inner_color: Color::rgba_i(0, 0, 0, 128)
    };
    ctx.begin_path();
    ctx.rect(Rect::from((x-10.0, y-10.0, w+20.0, h+30.0)));
    ctx.rounded_rect(
        Rect::from((x, y, w, h)),
        corner_radius-1.0
    );
    ctx.path_solidity(Solidity::Hole);
    ctx.fill_paint(shadow_paint);
    ctx.fill().unwrap();


    // Header
    let header_paint = Gradient::Linear {
        start: Point::new(x, y),
        end: Point::new(x, y+15.0),
        start_color: Color::rgba_i(255, 255, 255, 5),
        end_color: Color::rgba_i(0, 0, 0, 16)
    };
    ctx.begin_path();
    ctx.rounded_rect(
        Rect::from((x+1.0, y+1.0, w-2.0, 30.0)),
        corner_radius-1.0
    );
    ctx.fill_paint(header_paint);
    ctx.fill().unwrap();
    ctx.begin_path();
    ctx.move_to((x+0.5, y+0.5+30.0));
    ctx.line_to((x+0.5+w-1.0, y+0.5+30.0));
    ctx.stroke_paint(Color::rgba_i(0,0,0,32));
    ctx.stroke().unwrap();

    ctx.font_size(15.0);
    ctx.font("sans-bold");
    ctx.text_align(Align::CENTER|Align::MIDDLE);

    // nvgFontBlur(vg,2) : TODO: implement nvgFontBlur in nvg
    ctx.fill_paint(Color::rgba_i(0, 0, 0, 128));
    ctx.text((x+w/2.0, y+16.0), title).unwrap();

    // nvgFontBlur(vg,0) : TODO: implement nvgFontBlur in nvg
    ctx.fill_paint(Color::rgba_i(220,220,220,160));
    ctx.text((x+w/2.0, y+16.0), title).unwrap();

    ctx.restore();

    ctx
}