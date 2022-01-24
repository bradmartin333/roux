#[macro_use]
extern crate glium;
use glium::{
    glutin::{
        dpi::{LogicalPosition, LogicalSize},
        event::{
            DeviceEvent, ElementState, Event, MouseScrollDelta, StartCause, VirtualKeyCode,
            WindowEvent,
        },
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
    },
    Surface,
};

const TILE_SIZE: f32 = 3.0;
const VERTEX_SHADER: &str = r#"
    #version 140
    in vec2 pos;
    in float score;
    out vec3 vColor;
    vec3 hsv2rgb(vec3 c)
    {
        if (c.x > 0.0) {
            vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
            vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
            return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
        } else {
            return vec3(0.0);
        }
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
    // Window
    let mut event_loop = EventLoop::new();
    let size = LogicalSize::new(wid, hgt);
    let start_pos = LogicalPosition::new(start_x, start_y);
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_position(start_pos);
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Shader
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

    // Grid parameters
    let mut scale_factor: i32 = 0;
    let mut tile_size: f32 = TILE_SIZE * 3.0;

    // Scoring parameters
    let mut flip_hue = false;
    let mut threshold_inc: f64 = 0.001;
    let mut threshold = 0.0;

    // Refresh parameters
    let mut last_tile_size: f32 = 0.0;
    let mut last_data: Vec<u8> = vec![];
    let mut last_flip_hue = flip_hue;
    let mut last_threshold = threshold;

    event_loop.run_return(move |event, _, control_flow| {
        // Event handlers
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ModifiersChanged(state) => {
                    if state.shift() {
                        threshold_inc = 0.01;
                    } else {
                        threshold_inc = 0.001;
                    }
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(keycode) => match keycode {
                                VirtualKeyCode::A => {
                                    flip_hue = !flip_hue;
                                }
                                VirtualKeyCode::Up => {
                                    if 1.0 >= threshold + threshold_inc {
                                        threshold += threshold_inc;
                                    }
                                }
                                VirtualKeyCode::Down => {
                                    if threshold - threshold_inc > 0.0 {
                                        threshold -= threshold_inc;
                                    }
                                }
                                _ => (),
                            },
                            None => (),
                        }
                    }
                }
                _ => (),
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => match delta {
                    MouseScrollDelta::LineDelta(_, s) => {
                        scale_factor = s as i32;
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        // Grid resizing
        if scale_factor != 0 {
            let new_size = tile_size + scale_factor as f32;
            if new_size >= TILE_SIZE && wid / new_size as u32 >= 2 && hgt / new_size as u32 >= 2 {
                tile_size = new_size;
            }
            scale_factor = 0;
        }
        let grid_size: [u32; 2] = [
            (size.width as f32 / tile_size).ceil() as u32,
            (size.height as f32 / tile_size).ceil() as u32,
        ];

        // FPS control
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        // Read image from heap
        let data = unsafe { std::slice::from_raw_parts(array_pointer, array_size as usize) };

        // todo: raise an event for these instead of checking last state
        // Rebuild shaders if new data, grid size or scoring params
        if tile_size != last_tile_size
            || data.to_vec() != last_data
            || last_flip_hue != flip_hue
            || last_threshold != threshold
        {
            last_tile_size = tile_size;
            last_data = data.to_vec();
            last_flip_hue = flip_hue;
            last_threshold = threshold;
            vertices.clear();

            // Store tiles by (x, y, entropy)
            let mut entropies: Vec<(u32, u32, f64)> = vec![];
            let mut min_entropy: f64 = std::f64::MAX;
            let mut max_entropy: f64 = std::f64::MIN;

            // Iterate through heap image in tiles
            for j in (0..(data.len() as u32 / wid as u32)).step_by(tile_size as usize) {
                'block: for i in (0..wid as u32).step_by(tile_size as usize) {
                    // Calculate shannon entropy of pixel data within tile
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

                    // Store min and max for hue scaling
                    if entropy > max_entropy {
                        max_entropy = entropy
                    };
                    if min_entropy > entropy {
                        min_entropy = entropy
                    };
                    entropies.push((
                        i / tile_size as u32,
                        (grid_size[1] - (j / tile_size as u32)) as u32,
                        entropy,
                    ));
                }
            }

            // Grid goes from -1 to 1 in X and Y
            let virtual_tile_size: [f32; 2] =
                [2.0 / grid_size[0] as f32, 2.0 / grid_size[1] as f32];

            if flip_hue {
                std::mem::swap(&mut max_entropy, &mut min_entropy);
            }

            for entropy in entropies {
                let row = (entropy.1 as f32 * virtual_tile_size[1]) - 1.0;
                let col = (entropy.0 as f32 * virtual_tile_size[0]) - 1.0;
                let mapped = (entropy.2 - min_entropy) * 1.0 / (max_entropy - min_entropy);
                let hue_val = {
                    if mapped >= threshold {
                        mapped
                    } else {
                        0.0
                    }
                };
                vertices.append(&mut new_shape(
                    [col, row],
                    [virtual_tile_size[0], virtual_tile_size[1]],
                    hue_val as f32,
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
