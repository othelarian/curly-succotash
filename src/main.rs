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
            /*
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => *ctrl_flow = ControlFlow::Exit,
                WindowEvent::Resized(psize) => wc.resize(psize),
                //
                //
                _ => ()
            }
            */
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
