use gl;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use nvg::{Context, Extent, Color};
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

pub fn draw(frame_info: &mut FrameInfo, perf_graph: &perf::PerfGraph, t: f32) {
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
        ctx = render_demo(ctx, (w, h), frame_info.mouse_pos, frame_info.blowup, t);
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

mod caps;
mod color_wheel;
mod eyes;
//mod label
mod lines;
mod paragraph; // TODO
mod widths;
mod window;
mod scissor;
mod search_box;
mod drop_down;
mod label;
mod edit_box;
mod check_box;
mod button;

fn render_demo(
    mut ctx: Context<Renderer>,
    (width, height): (u32, u32),
    (mx, my): (f64, f64),
    blowup: bool,
    //
    // TODO : keyboard status
    //
    t: f32
) -> Context<Renderer> {
    //
    //
    // println!("{:?}", ctx);
    //
    ctx.save();
    ctx = eyes::draw(
        ctx, (width - 250) as f32, 50.0, 150.0, 100.0,
        mx as f32, my as f32, t
    );
    ctx = paragraph::draw(
        ctx, (width - 450) as f32, 50.0, 150.0, 100.0,
        mx as f32, my as f32
    );
    //ctx = draw_graph(ctx);
    ctx = color_wheel::draw(
        ctx, (width - 300) as f32, (height - 300) as f32,
        250.0, 250.0, t
    );
    ctx = lines::draw(ctx, 120.0, (height - 50) as f32, 600.0, 50.0, t);
    ctx = widths::draw(ctx, 10.0, 50.0, 30.0);
    ctx = caps::draw(ctx, 10.0, 300.0, 30.0);
    ctx = scissor::draw(ctx, 50.0, height as f32 - 80.0, t);
    //
    ctx.restore();
    //blowup
    ctx.save();
    if blowup {
        
        ctx.rotate(((t*0.3).sin()*5.0).to_radians());
        ctx.scale(2.0, 2.0);
    }
    //
    // widgets
    ctx = window::draw(ctx, "Widgets 'n Stuff", 50.0, 50.0, 300.0, 400.0);
    //
    let x = 60.0;
    let y = 95.0;
    ctx = search_box::draw(ctx, "Search", x, y, 280.0, 25.0);
    let y = y + 40.0;
    ctx = drop_down::draw(ctx, "Effects", x, y, 280.0, 28.0);
    let thumb_y = y + 14.0;

    // forms
    let y = y + 45.0;
    ctx = label::draw(ctx, "Login", x, y, 280.0, 20.0);
    let y = y + 25.0;
    ctx = edit_box::draw(ctx, "Email", x, y, 280.0, 28.0);
    let y = y + 35.0;
    ctx = edit_box::draw(ctx, "Password", x, y, 280.0, 28.0);
    let y = y + 38.0;
    ctx = check_box::draw(ctx, "Remember me", x, y, 140.0, 28.0);
    #[allow(non_snake_case)]
    let ICON_LOGIN = "";
    ctx = button::draw(ctx, Some(ICON_LOGIN), "Sign in", x+138.0, y, 140.0, 28.0, Color::rgba_i(0, 96, 128, 255));

    let y = y + 45.0;
    ctx = label::draw(ctx, "Diameter", x, y, 280.0, 20.0);
    let y = y + 25.0;
    
    //
    ctx.restore();
    ctx
}
