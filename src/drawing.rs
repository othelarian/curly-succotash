use gl;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use nvg::{Context, Extent};
use nvg_gl::Renderer;
use std::path::Path;

use crate::perf;

pub struct FrameInfo {
    context: Option<Context<Renderer>>,
    size: (u32, u32),
    scale_factor: f32,
    mouse_pos: (f64, f64),
    blowup: bool,
    premult: bool
}

impl FrameInfo {
    pub fn new(
        context: Context<Renderer>,
        psize: PhysicalSize<u32>,
        scale_factor: f64
    ) -> FrameInfo {
        unsafe {
            gl::Viewport(
                0, 0,
                psize.width as i32,
                psize.height as i32
            );
        }
        FrameInfo {
            context: Some(context),
            size: (psize.width, psize.height),
            scale_factor: scale_factor as f32,
            mouse_pos: (0.0, 0.0),
            blowup: false,
            premult: false
        }
    }

    pub fn size(&self) -> (u32, u32) { self.size.clone() }

    pub fn toggle_blowup(&mut self) { self.blowup = !self.blowup; }

    pub fn toggle_premult(&mut self) { self.premult = !self.premult; }

    pub fn update_size(&mut self, psize: PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(
                0, 0,
                psize.width as i32,
                psize.height as i32
            );
        }
        self.size = (psize.width, psize.height);
    }

    pub fn update_mouse_pos(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = (position.x, position.y);
    }
}

pub fn draw(frame_info: &mut FrameInfo, perf_graph: &perf::PerfGraph) {
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.32, 1.0);
        gl::Clear(
            gl::COLOR_BUFFER_BIT |
            gl::DEPTH_BUFFER_BIT |
            gl::STENCIL_BUFFER_BIT
        );
    }
    let (w, h) = frame_info.size;
    if let Some(mut ctx) = frame_info.context.take() {
        ctx.begin_frame(
            Extent {width: w as f32, height: h as f32},
            frame_info.scale_factor
        ).unwrap();
        ctx = render_demo(ctx, (w, h), frame_info.mouse_pos,    0);
        ctx = perf_graph.render(ctx);
        ctx.end_frame().unwrap();
        frame_info.context = Some(ctx);
    }
}

pub fn take_screenshot(size: (u32, u32)) {
    let mut pixels: Vec<gl::types::GLubyte> = vec![];
    pixels.resize(3 * size.0 as usize * size.1 as usize, 0);
    unsafe { gl::ReadPixels(
        0, 0, size.0 as _, size.1 as _,
        gl::RGB, gl::UNSIGNED_BYTE,
        pixels.as_mut_ptr() as *mut _
    ); }
    let mut pixflip: Vec<gl::types::GLubyte> = vec![];
    for v in (0..size.1 as _).rev() {
        let s = 3 * v as usize * size.0 as usize;
        let o = 3 * size.0 as usize;
        pixflip.extend_from_slice(&pixels[s..(s + o)]);
    }
    image::save_buffer(
        &Path::new("screenshot.png"),
        &pixflip,
        size.0 as u32,
        size.1 as u32,
        image::ColorType::Rgb8
    ).unwrap();
    //
    println!("screenshot saved");
    //
}

// ##############################################################
// PRIVATE ZONE #################################################

fn render_demo(
    mut ctx: Context<Renderer>,
    (width, height): (u32, u32),
    (mx, my): (f64, f64),
    //
    // TODO : keyboard status
    //
    t: usize
) -> Context<Renderer> {
    ctx.save();
    ctx = draw_eyes(
        ctx, width as f32 - 250.0, 50.0, 150.0, 100.0,
        mx as f32, my as f32, t as f32
    );
    ctx = draw_paragraph(
        ctx, width as f32 - 450.0, 50.0, 150.0, 100.0,
        mx as f32, my as f32
    );
    //ctx = draw_graph(ctx);
    //ctx = draw_color_wheel(ctx);
    //ctx = draw_lines(); // line joints
    //ctx = draw_widths // line caps
    //ctx = draw_caps // line caps
    //ctx = draw_scissors(ctx);
    //
    //blowup
    //
    // widgets
    //ctx = draw_window
    //ctx = draw_search_bow
    //ctx = draw_drop_down
    // forms
    //ctx = draw_label
    //ctx = draw_edit_box
    //
    ctx.restore();
    ctx
}

fn draw_eyes(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32, t: f32
) -> Context<Renderer> {
    let ex = w * 0.23;
    let ey = h * 0.5;
    let lx = x + ex;
    let ly = y + ey;
    let rx = x + w  - ex;
    let ry = y + ey;
    let bg1 = nvg::Gradient::Linear {
        start: nvg::Point::new(x, y + h * 0.5),
        end: nvg::Point::new(x + w * 0.1, y + h),
        start_color: nvg::Color::rgba_i(0, 0, 0, 32),
        end_color: nvg::Color::rgba_i(0, 0, 0, 16)
    };
    ctx.begin_path();
    ctx.ellipse(nvg::Point::new(lx + 3.0, ly + 16.0), ex, ey);
    ctx.ellipse(nvg::Point::new(rx + 3.0, ry + 16.0), ex, ey);
    ctx.fill_paint(bg1);
    ctx.fill().unwrap();
    let bg2 = nvg::Gradient::Linear {
        start: nvg::Point::new(x, y + h * 0.25),
        end: nvg::Point::new(x + w * 0.1, y + h),
        start_color: nvg::Color::rgba_i(220, 220, 220, 255),
        end_color: nvg::Color::rgba_i(128, 128, 128, 255)
    };
    ctx.begin_path();
    ctx.ellipse(nvg::Point::new(lx, ly), ex, ey);
    ctx.ellipse(nvg::Point::new(rx, ry), ex, ey);
    ctx.fill_paint(bg2);
    ctx.fill().unwrap();
    let mut dx = (mx - rx) / (ex * 10.0);
    let mut dy = (my - ry) / (ey * 10.0);
    let mut d = (dx*dx + dy*dy).sqrt();
    let br = (if ex < ey { ex } else { ey }) * 0.5;
    let blink = 1.0 - ((t*0.5).sin()).powf(200.0) * 0.8;
    if d > 1.0 { dx /= d; dy /= d; }
    dx *= ex*0.4;
    dy *= ey*0.5;
    ctx.begin_path();
    ctx.ellipse(
        nvg::Point::new(lx+dx, ly+dy+ey*0.25*(1.0-blink)),
        br, br as f32 *blink
    );
    ctx.fill_paint(nvg::Color::rgba_i(32, 32, 32, 255));
    ctx.fill().unwrap();
    dx = (mx - rx) / (ex * 10.0);
    dy = (my - ry) / (ey * 10.0);
    d = (dx*dx + dy*dy).sqrt();
    if d > 1.0 { dx /= d; dy /= d; }
    dx *= ex*0.4;
    dy *= ey*0.5;
    ctx.begin_path();
    ctx.ellipse(
        nvg::Point::new(rx+dx, ry+dy+ey*0.25*(1.0-blink)),
        br, br as f32 *blink
    );
    ctx.fill_paint(nvg::Color::rgba_i(32, 32, 32, 255));
    ctx.fill().unwrap();
    let gloss_l = nvg::Gradient::Radial {
        center: nvg::Point::new(lx-ex*0.25, ly-ey*0.5),
        in_radius: ex*0.1,
        out_radius: ex*0.75,
        inner_color: nvg::Color::rgba_i(255, 255, 255, 128),
        outer_color: nvg::Color::rgba_i(255, 255, 255, 0)
    };
    ctx.begin_path();
    ctx.ellipse(nvg::Point::new(lx, ly), ex, ey);
    ctx.fill_paint(gloss_l);
    ctx.fill().unwrap();
    let gloss_r = nvg::Gradient::Radial {
        center: nvg::Point::new(rx-ex*0.25, ry-ey*0.5),
        in_radius: ex*0.1,
        out_radius: ex*0.75,
        inner_color: nvg::Color::rgba_i(255, 255, 255, 128),
        outer_color: nvg::Color::rgba_i(255, 255, 255, 0)
    };
    ctx.begin_path();
    ctx.ellipse(nvg::Point::new(rx, ry), ex, ey);
    ctx.fill_paint(gloss_r);
    ctx.fill().unwrap();
    ctx
}

const PARA_TEXT: &str = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.🎉";
const PARA_HOVER_TEXT: &str = "Hover your mouse over the text to see calculated caret position.";
const PARA_BOX_TEXT: &str = "Testing\nsome multiline\ntext.";

fn draw_paragraph(
    mut ctx: Context<Renderer>,
    x: f32, y: f32, w: f32, _h: f32, mx: f32, my: f32
) -> Context<Renderer> {
    ctx.save();
    ctx.font_size(15.0);
    ctx.font("sans");
    ctx.text_align(nvg::Align::LEFT | nvg::Align::TOP);
    //
    let lineh = ctx.text_metrics();
    //
    //
    //
    //
    //
    ctx.restore();
    ctx
}
