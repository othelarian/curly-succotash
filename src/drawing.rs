use gl;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use nvg::{Context, Extent};
use nvg_gl::Renderer;
use std::path::Path;

pub struct WindowInfo {
    context: Option<Context<Renderer>>,
    size: (u32, u32),
    scale_factor: f32,
    mouse_pos: (f64, f64),
    blowup: bool,
    premult: bool
}

impl WindowInfo {
    pub fn new(
        context: Context<Renderer>,
        psize: PhysicalSize<u32>,
        scale_factor: f64
    ) -> WindowInfo {
        WindowInfo {
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
                (psize.width as f32 * self.scale_factor) as i32,
                (psize.height as f32 * self.scale_factor) as i32
            );
        }
        self.size = (psize.width, psize.height);
    }

    pub fn update_mouse_pos(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = (position.x, position.y);
    }
}

pub fn draw(win_info: &mut WindowInfo) {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
    }
    let (w, h) = win_info.size;
    if let Some(mut ctx) = win_info.context.take() {
        ctx.begin_frame(
            Extent {width: w as f32, height: h as f32},
            win_info.scale_factor
        ).unwrap();
        ctx.save();
        ctx = render_demo(ctx, (w, h), win_info.mouse_pos,    0);
        // 
        // TODO : render graph
        //
        ctx.restore();
        ctx.end_frame().unwrap();
        win_info.context = Some(ctx);
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
        size: (u32, u32),
        mouse_pos: (f64, f64),
        //
        // TODO : keyboard status
        //
        t: usize
    ) -> Context<Renderer> {
    //
    ctx.begin_path();
    ctx.rect(nvg::Rect::new(nvg::Point::new(20.0, 20.0), Extent::new(50.0, 50.0)));
    ctx.close_path();
    ctx.fill_paint(nvg::Color::rgb_i(255, 0, 0));
    ctx.fill().unwrap();
    //
    // TODO : everything!!
    //
        //x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32, t: f32
    //ctx = draw_eyes(ctx, width -250, );
    //ctx = draw_paragraph(ctx);
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
    //
    ctx
}

fn draw_eyes(
        mut ctx: Context<Renderer>,
        x: f32, y: f32, w: f32, h: f32, mx: f32, my: f32, t: f32
    ) -> Context<Renderer> {
    //
    //
    ctx.begin_path();
    ctx.rect(nvg::Rect::new(nvg::Point::new(30.0, 30.0), Extent::new(60.0, 20.0)));
    ctx.close_path();
    ctx.fill_paint(nvg::Color::rgb_i(0, 255, 0));
    ctx.fill().unwrap();
    //
    //
    ctx
}

