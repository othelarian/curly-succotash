use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::{
    ElementState,
    KeyboardInput as KI,
    VirtualKeyCode as VKC,
    WindowEvent as WE
};

pub enum HandleResult {
    Blowup,
    ChangeFlow,
    Close,
    Mouse(PhysicalPosition<f64>),
    Nothing,
    Premult,
    Resize(PhysicalSize<u32>),
    Screenshot
}

pub fn handle_events(event: &WE) -> HandleResult {
    match event {
        WE::CloseRequested => HandleResult::Close,
        WE::CursorMoved { position, ..} => HandleResult::Mouse(*position),
        WE::KeyboardInput { input: KI {
            state: ElementState::Released,
            virtual_keycode: Some(keycode),
            ..
        }, ..} => {
            match keycode {
                VKC::Escape => HandleResult::Close,
                VKC::Space => HandleResult::Blowup,
                VKC::F => HandleResult::ChangeFlow,
                VKC::P => HandleResult::Premult,
                VKC::S => HandleResult::Screenshot,
                _ => HandleResult::Nothing
            }
        }
        WE::Resized(psize) => HandleResult::Resize(*psize),
        _ => HandleResult::Nothing
    }
}

