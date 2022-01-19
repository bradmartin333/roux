use crate::canvas::Canvas;
use crate::color::Color;
use crate::input::KeyboardMouseStates;

pub fn test(pointer: *mut u32, data: &'static [u8], wid: u32, hgt: u32) {
    let canvas = Canvas::new(wid as usize, hgt as usize)
        .title("Roux Viewer")
        .state(KeyboardMouseStates::new())
        .input(KeyboardMouseStates::handle_input);

    canvas.render(move |state, image| {
        let width = image.width();
        let height = image.height() - 1;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - state.x;
                let dy = y as i32 - state.y;
                let dist = dx * dx + dy * dy;
                let lux = data[(height - y) * width + x];
                *pixel = Color {
                    r: lux,
                    g: if dist < 128 * 128 { dy as u8 } else { lux },
                    b: if dist < 128 * 128 { dx as u8 } else { lux },
                };
            }
        }

        if state.received_mouse_press && !state.mouse_clicked {
            unsafe {
                *pointer += 1;
            }
            state.mouse_clicked = true;
        }
    });
}
