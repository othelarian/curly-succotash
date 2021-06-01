use nvg::{ClockWise, Color, Context, Extent, Gradient, Point, Rect, Solidity};
use nvg_gl::Renderer;
use std::f32::consts::PI;

pub fn draw(
  mut ctx: Context<Renderer>,
  x: f32, y: f32, w: f32, h: f32, t: f32
) -> Context<Renderer> {
  let cx = x + w*0.5;
  let cy = y + h*0.5;
  let r1 = (if w < h { w } else { h }) * 0.5 - 5.0;
  let r0 = r1 - 20.0;
  let rt = (r0+r1)*0.5;
  let aeps = 0.5 * r1;
  let hue = (t*0.12).sin();
  ctx.save();
  // wheel
  for i in 0..6 {
    let a0 = (i as f32)/6.0 * PI * 2.0 - aeps;
    let a1 = ((i+1) as f32)/6.0 * PI * 2.0 - aeps;
    ctx.begin_path();
    ctx.arc(Point::new(cx, cy), r0, a0, a1, ClockWise::CW);
    ctx.arc(Point::new(cx, cy), r1, a1, a0, ClockWise::CCW);
    ctx.close_path();
    let ax = cx + a0.cos() * rt;
    let ay = cy + a0.sin() * rt;
    let bx = cx + a1.cos() * rt;
    let by = cy + a1.sin() * rt;
    let paint = Gradient::Linear {
      start: Point::new(ax, ay),
      end: Point::new(bx, by),
      start_color: Color::hsl(a0/ (PI*2.0), 1.0, 0.55),
      end_color: Color::hsl(a1/ (PI*2.0), 1.0, 0.55)
    };
    ctx.fill_paint(paint);
    ctx.fill().unwrap();
  }
  // wheel borders
  ctx.begin_path();
  ctx.circle(Point::new(cx, cy), r0-0.5);
  ctx.circle(Point::new(cx, cy), r1+0.5);
  ctx.stroke_paint(Color::rgba_i(0, 0, 0, 64));
  ctx.stroke_width(1.0);
  ctx.stroke().unwrap();
  // selector
  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(hue*PI*2.0);
  ctx.stroke_width(2.0);
  ctx.stroke_paint(Color::rgba_i(255, 255, 255, 192));
  ctx.begin_path();
  ctx.rect(Rect::new(Point::new(r0-1.0, -3.0), Extent::new(r1-r0+2.0, 6.0)));
  ctx.stroke().unwrap();
  // selector borders
  ctx.fill_paint(Gradient::Box {
    rect: Rect::new(Point::new(r0-3.0, -5.0), Extent::new(r1-r0+6.0, 10.0)),
    radius: 2.0,
    feather: 4.0,
    inner_color: Color::rgba_i(0, 0, 0, 128),
    outer_color: Color::rgba_i(0, 0, 0, 0)
  });
  ctx.begin_path();
  ctx.rect(Rect::new(Point::new(r0-12.0,-14.0), Extent::new(r1-r0+24.0, 28.0)));
  ctx.rect(Rect::new(Point::new(r0-2.0, -4.0), Extent::new(r1-r0+4.0, 8.0)));
  ctx.path_solidity(Solidity::Hole);
  ctx.fill().unwrap();
  // center triangle
  let r = r0 - 6.0;
  let ang = 2.0/3.0*PI;
  let mut ax = ang.cos() * r;
  let mut ay = ang.sin() * r;
  let bx = (-ang).cos() * r;
  let by = (-ang).sin() * r;
  ctx.begin_path();
  ctx.move_to(Point::new(r, 0.0));
  ctx.line_to(Point::new(ax, ay));
  ctx.line_to(Point::new(bx, by));
  ctx.close_path();
  ctx.fill_paint(Gradient::Linear {
    start: Point::new(r, 0.0),
    end: Point::new(ax, ay),
    start_color: Color::hsl(hue, 1.0, 0.5),
    end_color: Color::rgb(1.0, 1.0, 1.0)
  });
  ctx.fill().unwrap();
  ctx.fill_paint(Gradient::Linear {
    start: Point::new((r+ax)*0.5, (0.0+ay)*0.5),
    end: Point::new(bx, by),
    start_color: Color::rgba_i(0, 0, 0, 0),
    end_color: Color::rgba_i(0, 0, 0, 255)
  });
  ctx.fill().unwrap();
  ctx.stroke_paint(Color::rgba_i(0, 0, 0, 64));
  ctx.stroke().unwrap();
  // select circle on triangle
  ax *= 0.3;
  ay *= 0.4;
  ctx.stroke_width(2.0);
  ctx.stroke_paint(Color::rgba_i(255, 255, 255, 192));
  ctx.begin_path();
  ctx.circle(Point::new(ax, ay), 5.0);
  ctx.stroke().unwrap();
  ctx.fill_paint(Gradient::Radial {
    center: Point::new(ax, ay),
    in_radius: 7.0,
    out_radius: 9.0,
    inner_color: Color::rgba_i(0, 0, 0, 64),
    outer_color: Color::rgba_i(0, 0, 0, 0)
  });
  ctx.begin_path();
  ctx.rect(Rect::new(Point::new(ax-20.0, ay-20.0), Extent::new(40.0, 40.0)));
  ctx.circle(Point::new(ax, ay), 7.0);
  ctx.path_solidity(Solidity::Hole);
  ctx.fill().unwrap();
  ctx.restore();
  ctx.restore();
  ctx
}