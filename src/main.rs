use gl;
use glutin::ContextBuilder as CtxtB;
use glutin::dpi::{LogicalSize};
use glutin::event::{Event, StartCause, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder as WinB;

mod drawing;
use drawing::draw;

mod events;
use events::{handle_events, HandleResult as HR};


/* TODO LIST
 *
 * example_gl3.c -> lines 72-74 -> don't forget about this
 * example_gl3.c -> line 97 -> handled with "unsafe" block in main(), when wc is shadowed
 * example_gl3.c -> lines 108_112 -> there is no obvious eq. in nvg, further investigation needed
 * example_gl3.c -> line 121 -> What this call do?
 * resizing -> handle nvg context resizing too
 *
 * Reading example_gl3.c -> lines 150-182
 *
 * create window -> OK
 * init nvg context -> OK
 * draw a single red square -> WP
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
    let mut ctxt = nvg::Context::create(renderer).unwrap();

    //
    // TODO : init graphics
    //

    let ask_refresh = false;

    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match handle_events(&event) {
                HR::Close => *ctrl_flow = ControlFlow::Exit,
                HR::Resize(psize) => wc.resize(psize),
                //
                //
                Nothing => ()
            }
            Event::MainEventsCleared => if ask_refresh { wc.window().request_redraw(); },
            Event::RedrawRequested(_) => {
                //
                draw();
                //
                wc.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
