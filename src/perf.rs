use nvg::Context;
use nvg_gl::Renderer;
use std::collections::VecDeque;

pub enum GraphRenderStyle {
    FPS,
    MS,
    //PERCENT
}

pub struct PerfGraph<'a> {
    style: GraphRenderStyle,
    name: &'a str,
    values: VecDeque<f32>,
    pos: (f32, f32)
}

impl<'a> PerfGraph<'a> {
    pub fn new(
        style: GraphRenderStyle, name: &'a str,
        pos: (f32, f32)
    ) -> PerfGraph {
        PerfGraph {
            style, name, pos,
            values: vec![0.0; 100].into_iter().collect()
        }
    }

    pub fn render(&self, mut ctx: Context<Renderer>) -> Context<Renderer> {
        let avg: f32 =
            self.values.iter().sum::<f32>() /
            self.values.len() as f32;
        let w = 200.0;
        let h = 35.0;
        let (x, y) = self.pos;
        ctx.begin_path();
        ctx.rect(nvg::Rect::new(
            nvg::Point::new(x, y),
            nvg::Extent::new(w, h)
        ));
        ctx.fill_paint(nvg::Color::rgba_i(0, 0, 0, 128));
        ctx.fill().unwrap();
        ctx.begin_path();
        ctx.move_to(nvg::Point::new(x, y+h));
        let cl = match self.style {
            GraphRenderStyle::FPS => {
                |i, v, x, y, w, h| {
                    let v = (80.0_f32).min(1.0 / (0.000001 + v));
                    let vx = x + (i / 99.0) * w;
                    let vy = y + h - (v / 80.0) * h;
                    (vx, vy)
                }
            }
            /*
            GraphRenderStyle::MS => {
                //
                |_, _, _, _, _, _| { (0.0, 0.0) }
                //
            }
            */
            //GraphRenderStyle::PERCENT => {
            _ => {
                //
                |_, _, _, _, _, _| { (0.0, 0.0) }
                //
            }
        };
        for (i, v) in self.values.iter().enumerate() {
            let (vx, vy) = cl(i as f32, v.clone(), x, y, w, h);
            ctx.line_to(nvg::Point::new(vx, vy));
        }
        ctx.line_to(nvg::Point::new(x+w, y+h));
        ctx.fill_paint(nvg::Color::rgba_i(255, 192, 0, 128));
        ctx.fill().unwrap();

        ctx.font("sans");
        ctx.font_size(12.0);
        ctx.text_align(nvg::Align::LEFT | nvg::Align::TOP);
        ctx.fill_paint(nvg::Color::rgba_i(240, 240, 240, 192));
        ctx.text(nvg::Point::new(x+3.0, y+3.0), &self.name).unwrap();
        let txt = match self.style {
            GraphRenderStyle::FPS => {
                ctx.font_size(13.0);
                ctx.text_align(nvg::Align::RIGHT | nvg::Align::BASELINE);
                ctx.fill_paint(nvg::Color::rgba_i(240, 240, 240, 160));
                ctx.text(
                    nvg::Point::new(x+w-3.0, y+h-3.0),
                    format!("{:.2} ms", avg * 1000.0)
                ).unwrap();
                let a = format!("{:.2} FPS", 1.0 / avg);
                a
            }
            GraphRenderStyle::MS => {
                let a = format!("{}", 0.0);
                a
            }
            /*
            GraphRenderStyle::PERCENT => {
                let a = format!("{}", 0.0);
                a
            }
            */
        };
        ctx.font_size(15.0);
        ctx.text_align(nvg::Align::RIGHT | nvg::Align::TOP);
        ctx.fill_paint(nvg::Color::rgba_i(240, 240, 240, 255));
        ctx.text(nvg::Point::new(x+w-3.0, y+3.0), txt).unwrap();
        ctx
    }

    pub fn update(&mut self, frame_time: f32) {
        self.values.pop_front();
        self.values.push_back(frame_time);
    }
}

pub struct GPUTimer {
    //
    // TODO : what to do here?
    //
    _cur: usize,
    _ret: usize
}

impl GPUTimer {
    pub fn new() -> GPUTimer { GPUTimer { _cur: 0, _ret: 0} }

    pub fn start(&self) {
        //
        // TODO : get the code here
        //
        //unsafe {
        //    gl::BeginQuery(gl::GL_E
        //
    }

    /*
    pub fn stop(&self) {
        //
        // TODO : get the code here
        //
    }
    */
}

