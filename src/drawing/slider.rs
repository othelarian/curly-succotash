use nvg::{Context, Color, Solidity, Point, Gradient, Rect, Extent};
use nvg_gl::Renderer;

pub fn draw(
    mut ctx: Context<Renderer>,
    pos: f32, x: f32, y: f32, w: f32, h: f32
) -> Context<Renderer> {
    let cy = y+h*0.5;
    let kr = h*0.25;
    let slot_bg = Gradient::Box {
        rect: Rect::new(
            Point::new(x, cy-2.0+1.0),
            Extent::new(w, 4.0)
        ),
        radius: 2.0,
        feather: 2.0,
        inner_color: Color::rgba_i(0, 0, 0, 32),
        outer_color: Color::rgba_i(0, 0, 0, 128)
    };
    let knob_shadow_bg = Gradient::Radial {
        center: Point::new(x+pos*w, cy+1.0),
        in_radius: kr-3.0,
        out_radius: kr+3.0,
        inner_color: Color::rgba_i(0, 0, 0, 64),
        outer_color: Color::rgba_i(0, 0, 0, 0)
    };
    let knob_bg = Gradient:: Linear {
        start: Point::new(x, cy-kr),
        end: Point::new(x, cy+kr),
        start_color: Color::rgba_i(255, 255, 255, 16),
        end_color: Color::rgba_i(0, 0, 0, 16)
    };

    // Slot
    ctx.save();
    ctx.begin_path();
    ctx.rounded_rect(
        Rect::new(
            Point::new(x, cy-2.0),
            Extent::new(w, 4.0)
        ), 2.0);
    ctx.fill_paint(slot_bg);
    ctx.fill().unwrap();

    // Knob Shadow
    ctx.begin_path();
    ctx.rect( Rect::new(
                Point::new(x+pos*w-kr-5.0, cy-kr-5.0),
                Extent::new(kr*2.0+5.0+5.0, kr*2.0+5.0+5.0+3.0)
            ));
    ctx.circle(Point::new(x+pos*w, cy), kr);
    ctx.path_solidity(Solidity::Hole);
    ctx.fill_paint(knob_shadow_bg);
    ctx.fill().unwrap();

    // Knob
    ctx.begin_path();
    ctx.circle(Point::new(x+pos*w, cy), kr-1.0);
    ctx.path_solidity(Solidity::Hole);
    ctx.fill_paint(Color::rgba_i(40, 43, 48, 255));
    ctx.fill().unwrap();
    ctx.fill_paint(knob_bg);
    ctx.fill().unwrap();

    ctx.begin_path();
    ctx.circle(Point::new(x+pos*w, cy), kr-0.5);
    ctx.stroke_paint(Color::rgba_i(0, 0, 0, 92));
    ctx.stroke().unwrap();

    ctx.restore();

    ctx
}