mod window;

use window::{FullscreenMode, WindowBuilder, Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;



fn main() {
    let mut app = App::new();

    app.run();

}

struct App {
    window: Window,
}

impl App {



    fn new() -> App {
        let mut window = WindowBuilder::new()
            .with_title("Vulkan Application")
            .with_size(1024, 768)
            .with_resizable(true)
            .with_fullscreen(FullscreenMode::Windowed)
            .with_vsync(false)
            .with_max_fps(60)
            .build()
            .expect("Failed to create window");


        // init:
        let required_extensions = window.get_extensions();

        Self {
            window,
        }
    }

    fn process(&mut self) {
        println!("Process logic here.");
        self.window.quit();
    }

    fn render(&mut self) {
        println!("Render logic here.");
        //self.window.swap_buffers();
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                println!("Escape key pressed. Requesting quit.");
                // Handle quit externally.
            }
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                println!("Key pressed: {:?}", key);
            }
            Event::Quit { .. } => {
                println!("Quit event received.");
            }
            _ => {}
        }
    }

    fn run(&mut self) {


        while self.window.is_open() {

            let events: Vec<Event> = self.window.get_events();

            for event in events {
                self.handle_event(event);
            }

            self.process();

            self.render();
        }

    }
}
