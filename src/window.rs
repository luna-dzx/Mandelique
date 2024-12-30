use sdl2::event::{Event};
use sdl2::video::{Window as SdlWindow};
use sdl2::{Sdl};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Window {
    sdl_context: Sdl,
    pub sdl_window: SdlWindow,
    event_pump: sdl2::EventPump,
    quit_requested: bool,
    max_fps: Option<u32>,
    pub width: u32,
    pub height: u32,
}

pub enum FullscreenMode {
    Windowed,
    Fullscreen,
    Borderless,
}

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    fullscreen: FullscreenMode,
    vsync: bool,
    max_fps: Option<u32>,
    decorated: bool,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: "Vulkan Window".to_string(),
            width: 800,
            height: 600,
            resizable: false,
            fullscreen: FullscreenMode::Windowed,
            vsync: true,
            max_fps: None,
            decorated: true,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_fullscreen(mut self, mode: FullscreenMode) -> Self {
        self.fullscreen = mode;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_max_fps(mut self, fps: u32) -> Self {
        self.max_fps = Some(fps);
        self
    }

    pub fn with_decorated(mut self, decorated: bool) -> Self {
        self.decorated = decorated;
        self
    }

    pub fn build(self) -> Result<Window, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut window_builder = video_subsystem.window(&self.title, self.width, self.height);
        let mut window_builder = window_builder.vulkan();
        window_builder = window_builder.resizable();

 
        if self.resizable {
            window_builder = window_builder.resizable();
        }

        if !self.decorated {
            window_builder = window_builder.borderless();
        }

        window_builder = match self.fullscreen {
            FullscreenMode::Windowed => window_builder,
            FullscreenMode::Fullscreen => window_builder.fullscreen_desktop(),
            FullscreenMode::Borderless => window_builder.borderless(),
        };

        let sdl_window = window_builder
            .build()
            .unwrap();
            //.map_err(|e| format!("{e}"))?;*/

        let event_pump = sdl_context.event_pump()?;

        println!("{}", self.width);
        println!("{}", self.height);

        Ok(Window {
            sdl_context,
            sdl_window,
            event_pump,
            quit_requested: false,
            max_fps: self.max_fps,
            width: self.width,
            height: self.height,
        })
    }
}

impl Window {
    pub fn quit(&mut self) {
        self.quit_requested = true;
    }

    pub fn swap_buffers(&self)
    {
        self.sdl_window.gl_swap_window();
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    pub fn is_open(&self) -> bool {
        !self.quit_requested
    }

    pub fn get_extensions(&mut self) -> Vec<&str> {
        self.sdl_window
            .vulkan_instance_extensions()
            .expect("Failed to get Vulkan instance extensions")
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_id(&mut self) -> u32 {
        self.sdl_window.id()
    }
}
