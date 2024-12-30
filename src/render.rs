use std::borrow::Cow;
use std::collections::HashMap;
use wgpu::SurfaceError;

use sdl2::video::{Window as SdlWindow};
use crate::Window;
use crate::pipeline::Pipeline;

pub struct Render<'r> {
    // destroy surface before instance
    surface: wgpu::Surface<'r>,
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    pipeline: Pipeline,

}



impl<'r> Render<'r> {
    pub fn new(window: &SdlWindow) -> Result<Self, String> {

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

        let pipeline = Pipeline::new(&device);

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let (width,height) = window.size();

        let mut config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: Vec::default(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);


        Ok(Self {
            surface,
            instance,
            device,
            queue,
            config,

            pipeline,
        })
    }

    pub fn refresh_surface(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn draw(&mut self) {

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
                self.refresh_surface();
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

            self.pipeline.set(&mut rpass);

            rpass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
