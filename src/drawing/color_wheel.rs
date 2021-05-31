use nvg::{ClockWise, Context, Point};
use nvg_gl::Renderer;
use std::f32::consts::PI;

pub fn draw(
  mut ctx: Context<Renderer>,
  x: f32, y: f32, w: f32, h: f32, t: f32
) -> Context<Renderer> {
  //
  //
  let cx = x + w*0.5;
  let cy = y + h*0.5;
  let r1 = (if w < h { w } else { h }) * 0.5 - 5.0;
  let r0 = r1 - 20.0;
  let rt = (r0+r1)*0.5;
  let aeps = 0.5 * r1;
  //
  ctx.save();
  //
  for i in 0..6 {
    let a0 = (i as f32)/6.0 * PI * 2.0 - aeps;
    let a1 = ((i+1) as f32)/6.0 * PI * 2.0 - aeps;
    ctx.begin_path();
    ctx.arc(Point::new(cx, cy), r0, a0, a1, ClockWise::CW);
    ctx.arc(Point::new(cx, cy), r0, a1, a0, ClockWise::CCW);
    ctx.close_path();
    let ax = cx + a0.cos() * rt;
    let ay = cy + a0.sin() * rt;
    let bx = cx + a1.cos() * rt;
    let by = cy + a1.sin() * rt;
    //
    // let paint = ??
    //
    //
  }
  //
  //
  ctx.save();
  //
  //
  //
  //
  ctx.restore();
  ctx.restore();
  ctx
}