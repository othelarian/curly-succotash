use gl;
use glutin::ContextBuilder as CtxtB;
use glutin::dpi::{LogicalSize};
use glutin::event::{Event, StartCause};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder as WinB;
use nvg;
use nvg_gl;
use std::time::Instant;

mod drawing;
use drawing::{FrameInfo, draw, take_screenshot};

mod events;
use events::{handle_events, HandleResult as HR};

mod perf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Mode { Limited, Polled }

fn main() {
    let el = EventLoop::new();
    let wb = WinB::new()
        .with_title("Curly Succotash")
        .with_inner_size(LogicalSize::new(800.0, 600.0));
    let wc = CtxtB::new().build_windowed(wb, &el).unwrap();
    let wc = unsafe { wc.make_current().unwrap() };
    gl::load_with(|p| wc.get_proc_address(p) as *const _);

    let renderer = nvg_gl::Renderer::create().unwrap();
    let mut nvg_ctx = nvg::Context::create(renderer).unwrap();
    nvg_ctx.create_font_from_file("sans", "resources/Roboto-Regular.ttf")
        .unwrap();
    nvg_ctx.create_font_from_file("icons", "resources/entypo.ttf")
        .unwrap();
    nvg_ctx.create_font_from_file("sans-bold", "resources/Roboto-Bold.ttf")
        .unwrap();

    // scale factor set to 1 to force the pixel-perfect settings of the demo
    let mut frame_info = FrameInfo::new(
        nvg_ctx,
        wc.window().inner_size(),
        //wc.window().scale_factor()
        1.0
    );

    let mut fps_graph = perf::PerfGraph::new(
        perf::GraphRenderStyle::FPS, "FPS", (5.0, 5.0)
    );
    let _cpu_graph = perf::PerfGraph::new(
        perf::GraphRenderStyle::MS, "CPU Time", (0.0, 0.0)
    );
    let gpu_timer = perf::GPUTimer::new();
    gpu_timer.start();

    let mut mode = Mode::Polled;

    let elap_t = Instant::now();
    let mut prevt = elap_t.clone();

    el.run(move |evt, _, ctrl_flow| {
        if mode == Mode::Polled { wc.window().request_redraw(); }
        match evt {
            Event::NewEvents(StartCause::Init) => {
                println!("start waiting");
                *ctrl_flow = ControlFlow::Wait;
                mode = Mode::Limited;
            }
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => {
                match handle_events(&event) {
                    HR::Blowup => frame_info.toggle_blowup(),
                    HR::ChangeFlow => {
                        match mode {
                            Mode::Limited => {
                                println!("polled on");
                                *ctrl_flow = ControlFlow::Poll;
                                mode = Mode::Polled;
                            }
                            Mode::Polled => {
                                println!("polled off");
                                *ctrl_flow = ControlFlow::Wait;
                                mode = Mode::Limited;
                            }
                        }
                    }
                    HR::Close => *ctrl_flow = ControlFlow::Exit,
                    HR::Mouse(position) => frame_info.update_mouse_pos(position),
                    HR::Nothing => (),
                    HR::Premult => frame_info.toggle_premult(),
                    HR::Screenshot => {
                        take_screenshot(frame_info.size());
                        //wc.window().request_redraw();
                    }
                    HR::Resize(psize) => {
                        frame_info.update_size(psize); wc.resize(psize); }
                }
                wc.window().request_redraw();
            }
            //Event::MainEventsCleared => if ask_refresh {
            //    wc.window().request_redraw(); ask_refresh = false; },
            Event::RedrawRequested(_) => {
                let t = Instant::now();
                fps_graph.update((t - prevt).as_secs_f32());
                prevt = t;
                draw(&mut frame_info, &fps_graph, (t - elap_t).as_secs_f32());
                wc.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
