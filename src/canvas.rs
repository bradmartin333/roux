use crate::image::Image;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::{
    glutin::{
        self,
        event::{Event, StartCause},
        event_loop::ControlFlow,
    },
    Rect, Surface,
};
use std::time::{Duration, Instant};

/// A type that represents an event handler.
///
/// It returns true if the state is changed.
pub type EventHandler<State> = fn(&CanvasInfo, &mut State, &Event<()>) -> bool;

/// Information about the [`Canvas`](struct.Canvas.html).
pub struct CanvasInfo {
    /// The width of the canvas, in virtual pixels.
    pub width: usize,
    /// The height of the canvas, in virtual pixels.
    pub height: usize,
    /// The starting left position of the canvas, in virtual pixels.
    pub start_pos_x: i32,
    /// The starting top position canvas, in virtual pixels.
    pub start_pos_y: i32,
    /// The base title for the window.
    pub title: String,
    /// Only call the render callback if there's a state change.
    /// Defaults to `false`, which means it will instead render at a fixed framerate.
    pub render_on_change: bool,
}

/// A [`Canvas`](struct.Canvas.html) manages a window and event loop, handing
/// the current state to the renderer, and presenting its image on the screen.
pub struct Canvas<State, Handler = EventHandler<State>> {
    info: CanvasInfo,
    image: Image,
    state: State,
    event_handler: Handler,
}

impl Canvas<()> {
    /// Create a new canvas with a given virtual window dimensions.
    pub fn new(width: usize, height: usize, start_pos_x: i32, start_pos_y: i32) -> Canvas<()> {
        Canvas {
            info: CanvasInfo {
                width,
                height,
                start_pos_x,
                start_pos_y,
                title: "Canvas".into(),
                render_on_change: false,
            },
            image: Image::new(width, height),
            state: (),
            event_handler: |_, (), _| false,
        }
    }
}

#[allow(dead_code)]
impl<State, Handler> Canvas<State, Handler>
where
    Handler: FnMut(&CanvasInfo, &mut State, &Event<()>) -> bool + 'static,
    State: 'static,
{
    /// Set the attached state.
    ///
    /// Attaching a new state object will reset the input handler.
    pub fn state<NewState>(self, state: NewState) -> Canvas<NewState, EventHandler<NewState>> {
        Canvas {
            info: self.info,
            image: self.image,
            state,
            event_handler: |_, _, _| false,
        }
    }

    /// Set the title on the canvas window.
    pub fn title(self, text: impl Into<String>) -> Self {
        Self {
            info: CanvasInfo {
                title: text.into(),
                ..self.info
            },
            ..self
        }
    }

    /// Whether to render a new frame only on state changes.
    ///
    /// Defaults to `false`, which means it will render at a fixed framerate.
    pub fn render_on_change(self, enabled: bool) -> Self {
        Self {
            info: CanvasInfo {
                render_on_change: enabled,
                ..self.info
            },
            ..self
        }
    }

    /// Attach an input handler.
    ///
    /// Your input handler must be compatible with any state that you've set
    /// previously. Your event handler will be called for each event with the
    /// canvas information, the current state, and the inciting event.
    pub fn input<NewHandler>(self, callback: NewHandler) -> Canvas<State, NewHandler>
    where
        NewHandler: FnMut(&CanvasInfo, &mut State, &Event<()>) -> bool + 'static,
    {
        Canvas {
            info: self.info,
            image: self.image,
            state: self.state,
            event_handler: callback,
        }
    }

    /// Provide a rendering callback.
    ///
    /// The canvas will call your rendering callback on demant, with the
    /// current state and a reference to the image. Depending on settings,
    /// this will either be called at 60fps, or only called when state changes.
    /// See [`render_on_change`](struct.Canvas.html#method.render_on_change).
    pub fn render(mut self, mut callback: impl FnMut(&mut State, &mut Image) + 'static) {
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(&self.info.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                self.info.width as f64,
                self.info.height as f64,
            ))
            .with_resizable(false)
            .with_position(glutin::dpi::PhysicalPosition::new(
                self.info.start_pos_x,
                self.info.start_pos_y,
            ));
        let cb = glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        display.gl_window().window().set_cursor_visible(false);

        let width = self.info.width;
        let height = self.info.height;
        self.image = Image::new(width, height);

        let mut texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            width as u32,
            height as u32,
        )
        .unwrap();

        let mut next_frame_time = Instant::now();
        let mut should_render = true;
        event_loop.run_return(move |event, _, control_flow| match event {
            Event::NewEvents(StartCause::ResumeTimeReached { .. })
            | Event::NewEvents(StartCause::Init) => {
                next_frame_time = next_frame_time + Duration::from_nanos(16_666_667);
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
                if !should_render {
                    return;
                }
                if self.info.render_on_change {
                    should_render = false;
                }

                callback(&mut self.state, &mut self.image);
                let width = self.image.width() as u32;
                let height = self.image.height() as u32;
                if width != texture.width() || height != texture.height() {
                    texture = glium::Texture2d::empty_with_format(
                        &display,
                        glium::texture::UncompressedFloatFormat::U8U8U8,
                        glium::texture::MipmapsOption::NoMipmap,
                        width,
                        height,
                    )
                    .unwrap();
                    display
                        .gl_window()
                        .window()
                        .set_inner_size(glutin::dpi::LogicalSize::new(width as f64, height as f64));
                }
                texture.write(
                    Rect {
                        left: 0,
                        bottom: 0,
                        width: width as u32,
                        height: height as u32,
                    },
                    &self.image,
                );

                let target = display.draw();
                texture
                    .as_surface()
                    .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
                target.finish().unwrap();
            }
            glutin::event::Event::WindowEvent {
                event: glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            event => {
                let changed = (self.event_handler)(&self.info, &mut self.state, &event);
                should_render = changed || !self.info.render_on_change;
            }
        })
    }
}
