use crate::canvas::Canvas;
use crate::color::Color;
use crate::input::KeyboardMouseStates;

pub fn test(a: u8, b: u8, pointer: *mut u32) {
    let canvas = Canvas::new(500, 500)
        .title("Roux Viewer")
        .state(KeyboardMouseStates::new())
        .input(KeyboardMouseStates::handle_input);

    canvas.render(move |state, image| {
        let width = image.width();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - state.x;
                let dy = y as i32 - state.y;
                let dist = dx * dx + dy * dy;
                *pixel = Color {
                    r: if state.received_mouse_press { a } else { b },
                    g: if dist < 128 * 128 { dy as u8 } else { 0 },
                    b: if dist < 128 * 128 { dx as u8 } else { 0 },
                };
            }
        }

        if state.received_mouse_press && !state.mouse_clicked
        {
            unsafe {
                *pointer += 1;
            }
            state.mouse_clicked = true;
        }
    });
}
