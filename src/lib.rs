extern crate fps_clock;
extern crate sdl2;

use std::default::Default;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::EventPump;
pub use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::INIT_PNG;
use sdl2::rect::{Point, Rect};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;

pub type Result<T> = ::std::result::Result<T, Error>;

/// All of the different errors that can happen in the running of
/// a doodle.
#[derive(Debug)]
pub enum Error {
    SdlError(sdl2::Error),
    /// Integer Overflow happened in the SDL2 library.
    IntegerOrSdlError(sdl2::IntegerOrSdlError),
    /// An error happened while creating the TTF context.
    TtfInitError(sdl2::ttf::InitError),
    /// A font releated error happened.
    FontError(sdl2::ttf::FontError),
    /// An error occurred while building the window.
    WindowBuildError(sdl2::video::WindowBuildError),
    /// Something bad happened.
    Error(String),
}
impl From<sdl2::Error> for Error {
    fn from(error: sdl2::Error) -> Self {
        Error::SdlError(error)
    }
}
impl From<sdl2::ttf::InitError> for Error {
    fn from(error: sdl2::ttf::InitError) -> Self {
        Error::TtfInitError(error)
    }
}
impl From<sdl2::ttf::FontError> for Error {
    fn from(error: sdl2::ttf::FontError) -> Self {
        Error::FontError(error)
    }
}
impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(error: sdl2::IntegerOrSdlError) -> Self {
        Error::IntegerOrSdlError(error)
    }
}
impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}
impl From<sdl2::video::WindowBuildError> for Error {
    fn from(error: sdl2::video::WindowBuildError) -> Self {
        Error::WindowBuildError(error)
    }
}

/// A Handler is a callback function that takes no arguments
/// other than the state and renderer.
pub type Handler<T> = Box<Fn(&mut T, &mut Renderer) -> ()>;

/// A DoodleBuilder provides a simple api to create a Doodle
///
/// # Example
/// ```
/// let mut doodle = DoodleBuilder::new()
///         .name("Test")
///         .width(600)
///         .height(400)
///         .fps(30)
///         .state(State {})
///         .setup(Box::new(|_, r| {
///             r.set_draw_color(Color::RGB(100, 100, 100));
///             r.clear();
///         }))
///         .draw(Box::new(|_, r| {
///             r.clear();
///         }))
///         .build()
///         .unwrap();
/// ```
pub struct DoodleBuilder<'a, T: Default> {
    name: &'a str,
    width: u32,
    height: u32,
    state: T,
    fps: u32,
    setup: Handler<T>,
    draw: Handler<T>,
}

impl<'a, T: Default> DoodleBuilder<'a, T> {
    /// Creates a DoodleBuilder with the default settings.
    pub fn new() -> Self {
        DoodleBuilder {
            name: "Doodle",
            state: Default::default(),
            fps: 30,
            width: 800,
            height: 600,
            setup: Box::new(|_, _| ()),
            draw: Box::new(|_, _| ()),
        }
    }
    /// Sets the name of the doodle.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }
    /// Sets the fps of the doodle.
    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }
    /// Sets the width of the doodle.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    /// Sets the height of the doodle.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    /// Sets the object used to store the doodle's state.
    pub fn state(mut self, state: T) -> Self {
        self.state = state;
        self
    }
    /// Sets the callback used to setup the doodle. 
    pub fn setup(mut self, setup: Handler<T>) -> Self {
        self.setup = setup;
        self
    }
    /// Sets the callback used to draw each frame of the doodle.
    pub fn draw(mut self, draw: Handler<T>) -> Self {
        self.draw = draw;
        self
    }
    /// Builds the doodle using the settings stored in this DoodleBuilder.
    pub fn build(self) -> Result<Doodle<T>> {
        Ok(Doodle {
            state: self.state,
            fps: self.fps,
            setup: self.setup,
            draw: self.draw,
            renderer: Renderer::new(self.name, self.width, self.height)?,
        })
    }
}

/// A Doodle is responsible for actually starting the doodle and running the
/// proper callbacks in response to events.
/// 
/// The only way to create a Doodle is with a DoodleBuilder.
pub struct Doodle<T> {
    state: T,
    fps: u32,
    setup: Handler<T>,
    draw: Handler<T>,
    renderer: Renderer,
}

impl<T> Doodle<T> {
    /// Starts running the doodle.
    /// This method only returns if an error occurs or the doodle
    /// has finished executing.
    pub fn run(&mut self) -> Result<()> {
        let mut clock = fps_clock::FpsClock::new(self.fps);
        (self.setup)(&mut self.state, &mut self.renderer);
        self.renderer.present();
        'main: loop {
            for event in self.renderer.pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'main,
                    _ => (),
                }
            }
            (self.draw)(&mut self.state, &mut self.renderer);
            self.renderer.present();
            clock.tick();
        }
        Ok(())
    }
}

/// A Renderer is responsible for drawing to the screen.
pub struct Renderer {
    canvas: Canvas<Window>,
    pump: EventPump,
    ttf_context: Sdl2TtfContext,
}
impl Renderer {
    fn new(app_name: &str, width: u32, height: u32) -> Result<Renderer> {
        let (canvas, pump, ttf_context) = init_sdl(app_name, width, height)?;
        Ok(Renderer {
            canvas,
            pump,
            ttf_context,
        })
    }
    /// Sets the color that the renderer uses for drawing shapes, text, background, etc.
    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }
    /// Returns the current drawing color.
    pub fn draw_color(&self) -> Color {
        self.canvas.draw_color()
    }
    /// Clears the screen with the current drawing color.
    pub fn clear(&mut self) {
        self.canvas.clear();
    }
    /// Calling this function makes all the drawing operations
    /// performed actually appear on the screen.
    pub fn present(&mut self) {
        self.canvas.present();
    }
}

fn init_sdl(
    app_name: &str,
    width: u32,
    height: u32,
) -> Result<(Canvas<Window>, EventPump, Sdl2TtfContext)> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(INIT_PNG)?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(app_name, width, height)
        .position_centered()
        .opengl()
        .build()?;

    let canvas = window.into_canvas().build()?;
    let event_pump = sdl_context.event_pump()?;
    let ttf_context = sdl2::ttf::init()?;
    Ok((canvas, event_pump, ttf_context))
}

