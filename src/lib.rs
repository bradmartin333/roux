#[macro_use]
extern crate glium;
use crate::glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::{glutin, Surface};

const MIN_TILE_SIZE: f32 = 3.0;
const VERTEX_SHADER: &str = r#"
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
const FRAG_SHADER: &str = r#"
    #version 140
    in vec3 vColor;
    out vec3 color;
    void main() {
        color = vColor;
    }
"#;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    score: f32,
}

#[no_mangle]
pub extern "C" fn test_window(
    array_size: u32,
    array_pointer: *const u8,
    wid: u32,
    hgt: u32,
    start_x: u32,
    start_y: u32,
) {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let size = glutin::dpi::LogicalSize::new(wid, hgt);
    let start_pos = glutin::dpi::LogicalPosition::new(start_x, start_y);
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_position(start_pos);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

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
    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAG_SHADER, None).unwrap();

    let mut scale_factor: f32 = 1.0;
    let mut tile_size: f32 = MIN_TILE_SIZE;
    let mut last_tile_size: f32 = 0.0;
    let mut last_data: Vec<u8> = vec![];
    event_loop.run_return(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::MouseInput { button, .. } => match button {
                    glutin::event::MouseButton::Right => scale_factor = 0.9,
                    glutin::event::MouseButton::Left => scale_factor = 1.1,
                    _ => (),
                },
                _ => (),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => (),
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseWheel { delta } => match delta {
                    glutin::event::MouseScrollDelta::LineDelta(_, s) => {
                        scale_factor = (((s as f64).signum() * 0.1) + 1.0) as f32;
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        if scale_factor != 1.0 {
            let new_size = tile_size * scale_factor;
            if new_size >= MIN_TILE_SIZE
                && size.width as f32 / new_size >= 2.0
                && size.height as f32 / new_size >= 2.0
            {
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

        let data = unsafe { std::slice::from_raw_parts(array_pointer, array_size as usize) };
        if tile_size != last_tile_size || data.to_vec() != last_data {
            last_tile_size = tile_size;
            last_data = data.to_vec();
            vertices.clear();

            let mut entropies: Vec<(u32, u32, f64)> = vec![];
            let mut min_entropy: f64 = std::f64::MAX;
            let mut max_entropy: f64 = std::f64::MIN;
            for j in (0..(data.len() as u32 / wid as u32)).step_by(tile_size as usize) {
                'block: for i in (0..wid as u32).step_by(tile_size as usize) {
                    let mut entropy = 0.0;
                    let mut counts = [0; 256];

                    for k in 0..tile_size as u32 {
                        for l in 0..tile_size as u32 {
                            let idx = ((j + l) * wid as u32) + (i + k);
                            if data.len() > idx as usize {
                                let val: u8 = data[(((j + l) * wid as u32) + (i + k)) as usize];
                                counts[val as usize] += 1;
                            } else {
                                break 'block;
                            }
                        }
                    }

                    for &count in &counts {
                        if count == 0 {
                            continue;
                        }
                        let p: f64 = (count as f64) / (data.len() as f64);
                        entropy -= p * p.log(2.0);
                    }

                    if entropy > max_entropy {
                        max_entropy = entropy
                    };
                    if min_entropy > entropy {
                        min_entropy = entropy
                    };
                    entropies.push((
                        i / tile_size as u32,
                        (grid_size[1] - (j as f32 / tile_size)).abs() as u32,
                        entropy,
                    ));
                }
            }

            let virtual_tile_size: [f32; 2] = [2.0 / grid_size[0], 2.0 / grid_size[1]];
            for entropy in entropies {
                let row = (entropy.1 as f32 * virtual_tile_size[1]) - 1.0;
                let col = (entropy.0 as f32 * virtual_tile_size[0]) - 1.0;
                let mapped = (entropy.2 - min_entropy) * 1.0 / (max_entropy - min_entropy);
                vertices.append(&mut new_shape(
                    [col, row],
                    [virtual_tile_size[0], virtual_tile_size[1]],
                    mapped as f32,
                ));
            }

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
        }
    });
}
