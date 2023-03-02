use std::num::NonZeroU32;

use wgpu::{TextureDescriptor, ImageDataLayout, ImageCopyTexture, Origin3d, CommandEncoderDescriptor, BufferDescriptor, BufferUsages, ImageCopyBuffer};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::window::Window;

struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
}

impl State {
    async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        // let modes = &surface_caps.present_modes;
        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
    
}


pub fn main(){
    
    let mut state = pollster::block_on(State::new());
    
    let mut data = Vec::<u8>::new();
    for _ in 0..16{
        data.push(0x00);
        data.push(0xFF);
        data.push(0xFF);
        data.push(0xFF);
    }
    
    let texture_size = wgpu::Extent3d {
        width: 4,
        height: 4,
        depth_or_array_layers: 1,
    };
    let texture = state.device.create_texture(
        &wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC,
            label: Some("diffuse_texture"),
            view_formats: &[],
        }
    );
    let copy_tex = ImageCopyTexture{
        texture : &texture,
        mip_level : 0,
        origin : Origin3d::ZERO,
        aspect : wgpu::TextureAspect::All
    };
    state.queue.write_texture(
        copy_tex, 
        data.as_slice(), 
        ImageDataLayout {
            bytes_per_row : NonZeroU32::new(16),
            rows_per_image : NonZeroU32::new(4),
            offset : 0
        }, 
        texture_size);
    let buf_desc = BufferDescriptor{
        label : Some("buffer"),
        mapped_at_creation : true,
        size : 16*4,
        usage : BufferUsages::MAP_READ | BufferUsages::COPY_DST
    };
    let mut staging_buffer = state.device.create_buffer(
        &buf_desc
    );
    let enc_desc = CommandEncoderDescriptor{
        label: Some("my encoder")
    };
    let mut encoder = state.device.create_command_encoder(
        &enc_desc
    );
    let copy_buf = ImageCopyBuffer {
        buffer : &staging_buffer,
        layout : ImageDataLayout { offset: 0, bytes_per_row: NonZeroU32::new(16), rows_per_image: NonZeroU32::new(4) }
    };
    encoder.copy_texture_to_buffer(copy_tex, copy_buf, texture_size);

    for x in staging_buffer.slice(0..5).get_mapped_range().iter() {
        println!("{}", x)
    }
    
}