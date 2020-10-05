use gl;
use glutin::ContextBuilder as CtxtB;
use glutin::dpi::{LogicalSize};
use glutin::event::{Event, StartCause};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder as WinB;

mod drawing;
use drawing::{WindowInfo, draw, take_screenshot};

mod events;
use events::{handle_events, HandleResult as HR};

mod perf;
use perf::{GPUTimer};

fn main() {
    let el = EventLoop::new();
    let wb = WinB::new()
        .with_title("Curly Succotash")
        .with_inner_size(LogicalSize::new(800.0, 600.0));
    let wc = CtxtB::new().build_windowed(wb, &el).unwrap();
    let wc = unsafe { wc.make_current().unwrap() };
    gl::load_with(|p| wc.get_proc_address(p) as *const _);

    let renderer = nvg_gl::Renderer::create().unwrap();

    let mut win_info = WindowInfo::new(
        nvg::Context::create(renderer).unwrap(),
        wc.window().inner_size(),
        wc.window().scale_factor()
    );

    let mut ask_refresh = false;

    let mut gpu_timer = GPUTimer::new();
    gpu_timer.start();

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match handle_events(&event) {
                HR::Blowup => win_info.toggle_blowup(),
                HR::Close => *ctrl_flow = ControlFlow::Exit,
                HR::Mouse(position) => win_info.update_mouse_pos(position),
                HR::Nothing => (),
                HR::Premult => win_info.toggle_premult(),
                HR::Screenshot => { take_screenshot(win_info.size()); ask_refresh = true }
                HR::Resize(psize) => { win_info.update_size(psize); wc.resize(psize); }
            }
            Event::MainEventsCleared => if ask_refresh { wc.window().request_redraw(); ask_refresh = false; },
            Event::RedrawRequested(_) => {
                draw(&mut win_info);
                //
                // TODO : add here perf graph update
                //
                wc.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
