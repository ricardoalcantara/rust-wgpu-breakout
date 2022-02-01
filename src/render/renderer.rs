use super::{
    render2d_pipeline::Render2DPineline, RenderQuad, RenderText, RenderTexture, RenderVertices,
};
use log::info;
use std::iter;
use winit::window::Window;

pub struct Renderer<'a> {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render2d_pipeline: Render2DPineline,
    clear_color: wgpu::Color,

    output: Option<wgpu::SurfaceTexture>,
    encoder: Option<wgpu::CommandEncoder>,
    render_pass: Option<wgpu::RenderPass<'a>>,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: &Window) -> Renderer<'a> {
        let size = window.inner_size();

        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let power_preference = wgpu::util::power_preference_from_env()
            .unwrap_or(wgpu::PowerPreference::HighPerformance);
        let force_fallback_adapter = std::env::var("WGPU_FORCE_FALLBACK")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter,
            })
            .await
            .unwrap();

        let adapter_info = adapter.get_info();
        info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: adapter.limits(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
        info!("Limits {:#?}", device.limits());

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let render2d_pipeline =
            Render2DPineline::new(size.width, size.height, 32, &device, &config, &queue);

        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render2d_pipeline,
            clear_color,

            output: None,
            encoder: None,
            render_pass: None,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn reconfigure(&mut self) {
        self.resize(self.size);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn clear_color(&mut self, color: glam::Vec3) {
        self.clear_color = wgpu::Color {
            r: color.x as f64 / 255.0,
            g: color.x as f64 / 255.0,
            b: color.x as f64 / 255.0,
            a: 1.0,
        }
    }

    pub fn begin_draw(&mut self, camera: Option<glam::Mat4>) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.output = Some(output);
        self.encoder = Some(encoder);
        self.render_pass = Some(render_pass);

        if let Some(camera) = camera {
            self.render2d_pipeline.set_camera(camera, &self.queue);
        } else {
            self.render2d_pipeline.default_camera(&self.queue);
        }
        self.render2d_pipeline.begin_batch();
    }

    pub fn end_draw(&mut self) {
        self.render2d_pipeline.end_batch(&self.queue);
        self.render2d_pipeline
            .flush(&mut self.render_pass.as_ref().unwrap());

        let render_pass = self.render_pass.take().unwrap();
        let encoder = self.encoder.take().unwrap();
        let output = self.output.take().unwrap();

        drop(render_pass);

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    pub fn draw_quad(&mut self, quad: RenderQuad) {
        self.render2d_pipeline.draw_quad(
            &mut self.render_pass.as_ref().unwrap(),
            &self.queue,
            quad,
        );
    }

    pub fn draw_texture(&mut self, texture: RenderTexture) {
        self.render2d_pipeline.draw_texture(
            &mut self.render_pass.as_ref().unwrap(),
            &self.queue,
            texture,
        );
    }

    pub fn draw_text(&mut self, _text: RenderText) {
        // _text.font.draw_vertices(
        //     _text.text,
        //     _text.position,
        //     _text.size,
        //     |texture, vertices, texture_coords| {
        //         self.draw_vertices(RenderVertices {
        //             texture: Some(texture),
        //             vertices: &vertices,
        //             texture_coords,
        //             color: _text.color,
        //         })
        //     },
        // )
    }

    pub fn draw_vertices(&mut self, _vertices: RenderVertices) {
        self.render2d_pipeline.draw_vertices(
            &mut self.render_pass.as_ref().unwrap(),
            &self.queue,
            _vertices.vertices,
            _vertices.color,
            _vertices.texture_coords,
            _vertices.texture,
        )
    }
}
