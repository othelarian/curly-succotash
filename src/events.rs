use glutin::dpi::PhysicalSize;
use glutin::event::{WindowEvent as WE};

pub enum HandleResult {
    Close,
    Resize(PhysicalSize<u32>),
    //
    //
    Nothing
}

pub fn handle_events(event: &WE) -> HandleResult {
    match event {
        WE::CloseRequested => HandleResult::Close,
        WE::Resized(psize) => HandleResult::Resize(*psize),
        //
        _ => HandleResult::Nothing
    }
}

