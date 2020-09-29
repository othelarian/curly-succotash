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


/* TODO LIST
 *
 * example_gl3.c -> lines 72-74 -> don't forget about this
 * example_gl3.c -> line 97 -> handled with "unsafe" block in main(), when wc is shadowed
 * example_gl3.c -> lines 108_112 -> there is no obvious eq. in nvg, further investigation needed
 * example_gl3.c -> line 121 -> What this call do?
 * demo.c 
 *
 * screenshot -> https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/headless.rs
 *
 * Reading example_gl3.c -> DONE
 * Reading demo.h -> DONE
 * Reading demo.c -> 64-1213
 *
 * create window -> OK
 * init nvg context -> OK
 * clear gl buffers (example_gl3.c:150) -> OK
 * Setup windowInfo, and keep it up to date (resize) -> OK
 * draw a single red square -> OK
 * create "savescreenshot" fn -> OK
 * integrate drawEyes() -> WP
 * dive into demo.* -> WP
 * build perf graph -> TD
 * define STENCIL & ALIASING (example_gl3.c 108-112) -> AB
 *
 */


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

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match handle_events(&event) {
                HR::Close => *ctrl_flow = ControlFlow::Exit,
                HR::Nothing => (),
                HR::Screenshot => { take_screenshot(win_info.size()); ask_refresh = true }
                HR::Resize(psize) => { win_info.set_size(psize); wc.resize(psize); }
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
