use glutin::dpi::PhysicalSize;
use glutin::event::{
    ElementState,
    KeyboardInput as KI,
    VirtualKeyCode as VKC,
    WindowEvent as WE
};

pub enum HandleResult {
    Close,
    Nothing,
    Resize(PhysicalSize<u32>),
    Screenshot
}

pub fn handle_events(event: &WE) -> HandleResult {
    match event {
        WE::CloseRequested => HandleResult::Close,
        WE::Resized(psize) => HandleResult::Resize(*psize),
        WE::KeyboardInput { input: KI {
            state: ElementState::Released,
            virtual_keycode: Some(keycode),
            ..
        }, ..} => {
            match keycode {
                VKC::Escape => HandleResult::Close,
                VKC::Space => {
                    //
                    println!("space touched");
                    //
                    HandleResult::Nothing
                }
                VKC::P => {
                    //
                    println!("premult toggled");
                    //
                    HandleResult::Nothing
                }
                VKC::S => HandleResult::Screenshot,
                _ => HandleResult::Nothing
            }
        }
        _ => HandleResult::Nothing
    }
}

