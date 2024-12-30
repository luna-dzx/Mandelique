use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

mod render;
use render::Render;

mod window;
use window::{Window, WindowBuilder};

struct App<'app> {
    pub quitting: bool,

    render: Render<'app>,
    window: Window,
}



impl<'app> App<'app> {
    fn new() -> Result<Self, String> {
        // for setting window to tile by default on hyprland which i use :-)
        //std::env::set_var("SDL_VIDEODRIVER", "wayland");
        //std::env::set_var("SDL_WINDOW_ROLE", "normal");

        // Show logs from wgpu
        env_logger::init();


        let quitting = false;

        let window = WindowBuilder::new()
            .build()
            .unwrap();

        let render = Render::new(&window.sdl_window).unwrap();


        Ok(Self {
            quitting,

            render,
            window,
        })
    }

    fn resize_window(&mut self, width: u32, height: u32) {
        self.window.resize(width, height);
        self.render.resize(width, height);
    }

    fn process(&mut self) {
        for event in self.window.get_events() {

            match event {

                Event::Window {
                    window_id,
                    win_event, ..
                } if window_id == self.window.get_id() => {
                    match win_event {
                        WindowEvent::SizeChanged(width,height) |
                        WindowEvent::Resized(width,height)   => {
                            self.resize_window(width as u32, height as u32);
                            return;
                        }

                        WindowEvent::Moved(x,y) => {
                            self.render.refresh_surface();
                            return;
                        }

                        e => {
                            dbg!(e);
                        }
                    }
                }


                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.quitting = true;
                    return;
                }

                e => {
                    //dbg!(e);
                }
            }
        }

        self.render.draw();
    }

}

fn main(){

    let mut app = App::new().unwrap();

    while !app.quitting {
        app.process();
    }


}
