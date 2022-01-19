use crate::canvas::Canvas;
use crate::color::Color;
use crate::input::KeyboardMouseStates;

pub fn test(
    pointer: *mut u32,
    data: &'static [u8],
    wid: i32,
    hgt: i32,
    start_pos_x: i32,
    start_pos_y: i32,
) {
    let brush_size: i32 = 10;
    let mut canvas_data = vec![0; data.len()];
    canvas_data.clone_from_slice(data);

    let canvas = Canvas::new(wid as usize, hgt as usize, start_pos_x, start_pos_y)
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
                if state.received_mouse_press && !state.mouse_clicked {
                    for i in (brush_size / -2)..(brush_size / 2) {
                        for j in 0..brush_size {
                            canvas_data[(height - state.y as usize + j as usize) * width
                                + i as usize
                                + state.x as usize] = 255;
                        }
                    }
                }
                let lux = canvas_data[(height - y) * width + x];
                *pixel = Color {
                    r: lux,
                    g: if dist < 128 * 128 {
                        dy as u8
                    } else {
                        if lux == 255 {
                            0
                        } else {
                            lux
                        }
                    },
                    b: if dist < 128 * 128 {
                        dx as u8
                    } else {
                        if lux == 255 {
                            0
                        } else {
                            lux
                        }
                    },
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
