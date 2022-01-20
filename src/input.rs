use crate::canvas::CanvasInfo;
use crate::input::glutin::event::{ElementState, Event, WindowEvent};
use glium::glutin;

pub struct MouseStates {
    pub received_mouse_press: bool,
    pub mouse_clicked: bool,
    pub x: i32,
    pub y: i32,
}

#[allow(deprecated)]
impl MouseStates {
    pub fn new() -> Self {
        Self {
            received_mouse_press: false,
            mouse_clicked: false,
            x: 0,
            y: 0,
        }
    }

    pub fn handle_input(
        info: &CanvasInfo,
        state_to_change: &mut MouseStates,
        event: &Event<()>,
    ) -> bool {
        match event {
            Event::WindowEvent {
                window_id: _window_id,
                event: window_event,
            } => match window_event {
                WindowEvent::CursorMoved {
                    device_id: _device_id,
                    position,
                    modifiers: _modifiers,
                } => {
                    let (x, y): (i32, i32) = (*position).into();
                    state_to_change.x = x;
                    state_to_change.y = info.height as i32 - y;
                    true
                }
                WindowEvent::MouseInput {
                    device_id: _device_id,
                    state,
                    button: _button,
                    modifiers: _modifiers,
                } => match state {
                    ElementState::Pressed => {
                        state_to_change.received_mouse_press = true;
                        return true;
                    }
                    ElementState::Released => {
                        state_to_change.received_mouse_press = false;
                        state_to_change.mouse_clicked = false;
                        return false;
                    }
                },
                _ => false,
            },
            _ => false,
        }
    }
}
