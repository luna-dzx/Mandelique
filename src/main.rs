use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::SurfaceError;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;


struct App<'app> {

    pub width: u32,
    pub height: u32,
    pub quitting: bool,

    // destroy surface before instance
    surface: wgpu::Surface<'app>,
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration,
    bind_group: wgpu::BindGroup,

    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
}



impl<'app> App<'app> {
    fn new() -> Result<Self, String> {

        // for setting window to tile by default on hyprland which i use :-)
        //std::env::set_var("SDL_VIDEODRIVER", "wayland");
        //std::env::set_var("SDL_WINDOW_ROLE", "normal");

        // Show logs from wgpu
        env_logger::init();

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut window = video
            .window("Mandelique", 800, 600)
        //    .position_centered()
            .resizable()
        //    .borderless()
            .metal_view()
            .build()
            .map_err(|e| e.to_string())?;

        window.set_minimum_size(100,100);
        let (width, height) = window.size();


        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });
        let surface = unsafe {
            match instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
            {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            }
        };
        let adapter_opt = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }));
        let adapter = match adapter_opt {
            Some(a) => a,
            None => return Err(String::from("No adapter found")),
        };

        let (device, queue) = match pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_limits: wgpu::Limits::default(),
                label: Some("device"),
                required_features: wgpu::Features::empty(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[],
            label: Some("bind_group_layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[],
            label: Some("bind_group"),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
            label: None,
            push_constant_ranges: &[],
        });

        let compilation_options = wgpu::PipelineCompilationOptions {
            constants: &HashMap::new(),
            zero_initialize_workgroup_memory: true,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &[],
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: compilation_options.clone(),
            },
            fragment: Some(wgpu::FragmentState {
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: compilation_options.clone(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            label: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None, //TODO cache pipeline
        });

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: Vec::default(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut event_pump = sdl.event_pump()?;

        let quitting = false;

        Ok(App {
            width,
            height,
            quitting,

            surface,
            instance,
            device,
            queue,
            pipeline,
            config,
            bind_group,

            sdl,
            video,
            event_pump,
            window,
        })
    }

    fn resize_window(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn get_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    fn process(&mut self) {
        for event in self.get_events() {

            match event {

                Event::Window {
                    window_id,
                    win_event, ..
                } if window_id == self.window.id() => {
                    match win_event {
                        WindowEvent::SizeChanged(width,height) |
                        WindowEvent::Resized(width,height)   => {
                            self.resize_window(width as u32, height as u32);
                            return;
                        }

                        WindowEvent::Moved(x,y) => {
                            self.surface.configure(&self.device, &self.config);
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
                }

                e => {
                    //dbg!(e);
                }
            }
        }

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                let reason = match err {
                    SurfaceError::Timeout => "Timeout",
                    SurfaceError::Outdated => "Outdated",
                    SurfaceError::Lost => "Lost",
                    SurfaceError::OutOfMemory => "OutOfMemory",
                };
                println!("Surface {}", reason);
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };


        let output = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                label: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        frame.present();
    }

}

fn main(){

    let mut app = App::new().unwrap();

    while !app.quitting {
        app.process();
    }


}
