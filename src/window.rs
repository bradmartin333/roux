use crate::glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::{glutin, Surface};

const MIN_TILE_SIZE: f32 = 5.0;

pub fn test(wid: f64, hgt: f64, start_x: f64, start_y: f64) {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let size = glutin::dpi::LogicalSize::new(wid, hgt);
    let start_pos = glutin::dpi::LogicalPosition::new(start_x, start_y);
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_position(start_pos);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        pos: [f32; 2],
        score: f32,
    }

    implement_vertex!(Vertex, pos, score);
    let mut vertices: Vec<Vertex> = vec![];

    fn new_shape(p: [f32; 2], size: [f32; 2], score: f32) -> Vec<Vertex> {
        vec![
            Vertex {
                pos: [p[0], p[1]],
                score,
            },
            Vertex {
                pos: [p[0], p[1] + size[1]],
                score,
            },
            Vertex {
                pos: [p[0] + size[0], p[1]],
                score,
            },
            Vertex {
                pos: [p[0] + size[0], p[1] + size[1]],
                score,
            },
            Vertex {
                pos: [p[0], p[1] + size[1]],
                score,
            },
            Vertex {
                pos: [p[0] + size[0], p[1]],
                score,
            },
        ]
    }

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;
        in float score;
        out vec3 vColor;
        vec3 hsv2rgb(vec3 c)
        {
            vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
            vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
            return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
        }
        void main() {
            gl_Position = vec4(pos, 0.0, 1.0);
            vColor = hsv2rgb(vec3(score, 1.0, 1.0));
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 vColor;
        out vec3 color;
        void main() {
            color = vColor;
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;
    let mut tile_size: f32 = MIN_TILE_SIZE;
    let mut scale_factor: f32 = 1.0;
    event_loop.run_return(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::MouseInput { button, .. } => {
                    match button {
                        glutin::event::MouseButton::Right => scale_factor = 0.9,
                        glutin::event::MouseButton::Left => scale_factor = 1.1,
                        _ => return,
                    }
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseWheel { delta } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_, s) => {
                        scale_factor = (((s as f64).signum() * 0.1) + 1.0) as f32;
                    }
                    _ => return,
                },
                _ => return,
            },
            _ => return,
        }

        if scale_factor != 1.0 {
            let new_size = tile_size * scale_factor;
            if new_size >= MIN_TILE_SIZE
                && size.width as f32 / new_size >= 2.0
                && size.height as f32 / new_size >= 2.0
            {
                t = 0.0;
                vertices.clear();
                tile_size = new_size;
            }
            scale_factor = 1.0;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let grid_size: [f32; 2] = [
            (size.width as f32 / tile_size).ceil(),
            (size.height as f32 / tile_size).ceil(),
        ];

        let mut idx: u32 = 0;
        loop {
            if idx < (grid_size[0] * grid_size[1]) as u32 {
                let virtual_tile_size: [f32; 2] = [2.0 / grid_size[0], 2.0 / grid_size[1]];
                let row = ((idx % grid_size[1] as u32) as f32 * virtual_tile_size[1]) - 1.0;
                let col = ((idx / grid_size[1] as u32) as f32 * virtual_tile_size[0]) - 1.0;
                vertices.append(&mut new_shape(
                    [col, row],
                    [virtual_tile_size[0], virtual_tile_size[1]],
                    (t + idx as f32) / (grid_size[0] * grid_size[1]) % 1.0,
                ));
                idx += 1;
            } else {
                break;
            }
        }
        t += 20.0 / tile_size;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &glium::VertexBuffer::new(&display, &vertices).unwrap(),
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
