use crate::canvas::Canvas;
use crate::color::Color;
use crate::input::MouseStates;
use glium::{glutin, Surface};

pub fn simple_window() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

pub fn test(
    num_clicks: *mut u32,
    array_pointer: *const u8,
    size: usize,
    wid: i32,
    hgt: i32,
    start_pos_x: i32,
    start_pos_y: i32,
) {
    let brush_size: i32 = 10;
    let mut canvas_data: Vec<u8> = vec![0; size];

    let canvas = Canvas::new(wid as usize, hgt as usize, start_pos_x, start_pos_y)
        .title("Roux Viewer")
        .state(MouseStates::new())
        .input(MouseStates::handle_input);

    canvas.render(move |state, image| {
        let data = unsafe { std::slice::from_raw_parts(array_pointer, size) };
        let width = image.width();
        let height = image.height() - 1;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - state.x;
                let dy = y as i32 - state.y;
                if state.received_mouse_press && !state.mouse_clicked {
                    for i in (brush_size / -2)..(brush_size / 2) {
                        for j in (brush_size / -2)..(brush_size / 2) {
                            let cursor_idx =
                                (height as i32 - state.y + j) * width as i32 + i + state.x;
                            canvas_data[cursor_idx as usize] = 1;
                        }
                    }
                }
                let idx = (height - y) * width + x;
                let lux = data[idx];
                let crosshair = if canvas_data[idx] == 1 {
                    0
                } else {
                    if dx.abs() < brush_size / 2 || dy.abs() < brush_size / 2 {
                        200
                    } else {
                        lux
                    }
                };
                *pixel = Color {
                    r: if canvas_data[idx] == 1 { 255 } else { lux },
                    g: crosshair,
                    b: crosshair,
                };
            }
        }

        if state.received_mouse_press && !state.mouse_clicked {
            unsafe {
                *num_clicks += 1;
            }
            state.mouse_clicked = true;
        }
    });
}
