use crate::canvas::Canvas;
use crate::color::Color;
use crate::input::KeyboardMouseStates;

pub fn test() {
    let canvas = Canvas::new(500, 500)
        .title("Oxey Viewer")
        .state(KeyboardMouseStates::new())
        .input(KeyboardMouseStates::handle_input);

    canvas.render(|state, image| {
        let width = image.width();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - state.x;
                let dy = y as i32 - state.y;
                let dist = dx * dx + dy * dy;
                *pixel = Color {
                    r: if state.received_mouse_press { 255 } else { 0 },
                    g: if dist < 128 * 128 { dy as u8 } else { 0 },
                    b: if dist < 128 * 128 { dx as u8 } else { 0 },
                };
            }
        }
    });
}
