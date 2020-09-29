use gl;
use glutin::dpi::PhysicalSize;
use nvg::{Context, Extent};
use nvg_gl::Renderer;
use std::path::Path;

pub struct WindowInfo {
    context: Context<Renderer>,
    size: (u32, u32),
    scale_factor: f32
}

impl WindowInfo {
    pub fn new(
        context: Context<Renderer>,
        psize: PhysicalSize<u32>,
        scale_factor: f64
    ) -> WindowInfo {
        WindowInfo {
            context,
            size: (psize.width, psize.height),
            scale_factor: scale_factor as f32
        }
    }

    pub fn size(&self) -> (u32, u32) { self.size.clone() }

    pub fn set_size(&mut self, psize: PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(
                0, 0,
                (psize.width as f32 * self.scale_factor) as i32,
                (psize.height as f32 * self.scale_factor) as i32
            );
        }
        self.size = (psize.width, psize.height);
    }
}

pub fn draw(win_info: &mut WindowInfo) {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
    }
    let (w, h) = win_info.size;
    let ctx = &mut win_info.context;
    //
    ctx.begin_frame(
        Extent {width: w as f32, height: h as f32},
        win_info.scale_factor
    ).unwrap();
    ctx.save();
    render_demo(ctx);
    // 
    // TODO : render graph
    //
    ctx.restore();
    ctx.end_frame().unwrap();
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
}

// ##############################################################
// PRIVATE ZONE #################################################

fn render_demo(ctx: &mut Context<Renderer>) {
    //
    ctx.begin_path();
    ctx.rect(nvg::Rect::new(nvg::Point::new(20.0, 20.0), Extent::new(50.0, 50.0)));
    ctx.close_path();
    ctx.fill_paint(nvg::Color::rgb_i(255, 0, 0));
    ctx.fill().unwrap();
    //
    // TODO : everything!!
    //
    drawEyes(&mut ctx);
    //
}

fn drawEyes(ctx: &mut Context<Renderer>) {
    //
    ctx.begin_path();
    ctx.rect(nvg::Rect::new(nvg::Point::new(30.0, 30.0), Extent::new(60.0, 20.0)));
    ctx.close_path();
    ctx.fill_paint(nvg::Color::rgb_i(255, 0, 0));
    ctx.fill().unwrap();
    //
}

