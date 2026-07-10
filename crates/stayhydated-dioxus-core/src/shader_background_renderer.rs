use std::cell::{Cell, RefCell};
use std::rc::Rc;

use wasm_bindgen::{JsCast as _, closure::Closure};
use web_sys::HtmlCanvasElement;
use wgpu::{
    Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferDescriptor,
    BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
    CompositeAlphaMode, Device, DeviceDescriptor, FragmentState, Instance, InstanceDescriptor,
    Limits, LoadOp, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface,
    SurfaceConfiguration, SurfaceTarget, TextureUsages, TextureViewDescriptor, VertexState,
};

const TARGET_FRAME_MS: f64 = 1000.0 / 30.0;
const MAX_CANVAS_PIXELS: f64 = 1280.0 * 720.0;
const MIN_CANVAS_SCALE: f64 = 0.4;
const MAX_CANVAS_SCALE: f64 = 1.25;
const SHADER_BACKGROUND_STATUS_ATTRIBUTE: &str = "data-shader-background";
const SHADER_BACKGROUND_STATUS_READY: &str = "ready";

type AnimationFrameCallback = Closure<dyn FnMut(f64)>;

pub(crate) struct ShaderBackgroundHandle {
    running: Rc<Cell<bool>>,
    frame_callback: Rc<RefCell<Option<AnimationFrameCallback>>>,
}

impl ShaderBackgroundHandle {
    pub(crate) fn stop(&self) {
        self.running.set(false);
        self.frame_callback.borrow_mut().take();
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    resolution: [f32; 2],
    time: f32,
    grid_opacity: f32,
}

pub(crate) fn start(canvas_id: String, grid_opacity: f32) -> ShaderBackgroundHandle {
    let handle = ShaderBackgroundHandle {
        running: Rc::new(Cell::new(true)),
        frame_callback: Rc::new(RefCell::new(None)),
    };
    let running = handle.running.clone();
    let frame_callback = handle.frame_callback.clone();

    dioxus::prelude::spawn(async move {
        if let Err(error) = run(&canvas_id, grid_opacity, running, frame_callback).await {
            log_error(&format!("failed to start shader background: {error}"));
        }
    });

    handle
}

async fn run(
    canvas_id: &str,
    grid_opacity: f32,
    running: Rc<Cell<bool>>,
    frame_callback: Rc<RefCell<Option<AnimationFrameCallback>>>,
) -> Result<(), String> {
    if !running.get() {
        return Ok(());
    }

    let window = web_sys::window().ok_or_else(|| "window unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "document unavailable".to_string())?;
    let element = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| format!("canvas #{canvas_id} not found"))?;
    let canvas = element
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| format!("#{canvas_id} is not a canvas"))?;
    if !wgpu::util::is_browser_webgpu_supported().await {
        return Ok(());
    }

    let renderer = ShaderBackgroundRenderer::new(canvas, grid_opacity).await?;

    start_render_loop(Rc::new(RefCell::new(renderer)), running, frame_callback)
}

fn start_render_loop(
    renderer: Rc<RefCell<ShaderBackgroundRenderer>>,
    running: Rc<Cell<bool>>,
    frame_callback: Rc<RefCell<Option<AnimationFrameCallback>>>,
) -> Result<(), String> {
    if !running.get() {
        return Ok(());
    }

    let is_ready = Rc::new(Cell::new(false));
    let callback_handle = frame_callback.clone();

    *callback_handle.borrow_mut() = Some(Closure::wrap(Box::new(move |time_ms| {
        if !running.get() {
            return;
        }

        let rendered_frame = renderer.borrow_mut().render(time_ms);
        if rendered_frame && !is_ready.get() {
            is_ready.set(true);
            mark_shader_background_ready();
        }

        let borrowed_callback = frame_callback.borrow();
        if running.get()
            && let Some(callback) = borrowed_callback.as_ref()
            && let Err(error) = request_animation_frame(callback)
        {
            log_error(&format!("failed to request animation frame: {error}"));
        }
    }) as Box<dyn FnMut(f64)>));

    {
        let borrowed_callback = callback_handle.borrow();
        let callback = borrowed_callback
            .as_ref()
            .ok_or_else(|| "animation callback missing".to_string())?;
        request_animation_frame(callback)?;
    }

    Ok(())
}

fn request_animation_frame(callback: &Closure<dyn FnMut(f64)>) -> Result<i32, String> {
    web_sys::window()
        .ok_or_else(|| "window unavailable".to_string())?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|error| format!("{error:?}"))
}

struct ShaderBackgroundRenderer {
    canvas: HtmlCanvasElement,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    uniform_buffer: Buffer,
    config: SurfaceConfiguration,
    last_frame_ms: f64,
    grid_opacity: f32,
}

impl ShaderBackgroundRenderer {
    async fn new(canvas: HtmlCanvasElement, grid_opacity: f32) -> Result<Self, String> {
        let mut instance_descriptor = InstanceDescriptor::new_without_display_handle();
        instance_descriptor.backends = Backends::BROWSER_WEBGPU;
        let instance = Instance::new(instance_descriptor);
        let surface = instance
            .create_surface(SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|error| error.to_string())?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                apply_limit_buckets: false,
            })
            .await
            .map_err(|error| error.to_string())?;
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_limits: Limits::downlevel_webgl2_defaults(),
                ..Default::default()
            })
            .await
            .map_err(|error| error.to_string())?;
        let size = canvas_size(&canvas);
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| "surface is not supported by the selected adapter".to_string())?;
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: config.format,
            color_space: config.color_space,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: preferred_present_mode(&surface, &adapter),
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader background"),
            source: ShaderSource::Wgsl(include_str!("shader_background.wgsl").into()),
        });
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("shader background uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("shader background bind group layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("shader background bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("shader background pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("shader background pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vertex_main"),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fragment_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            pipeline,
            bind_group,
            uniform_buffer,
            config,
            last_frame_ms: -TARGET_FRAME_MS,
            grid_opacity,
        })
    }

    fn render(&mut self, time_ms: f64) -> bool {
        if time_ms - self.last_frame_ms < TARGET_FRAME_MS {
            return false;
        }
        self.last_frame_ms = time_ms;

        let size = canvas_size(&self.canvas);
        if size.width == 0 || size.height == 0 {
            return false;
        }
        if size.width != self.config.width || size.height != self.config.height {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }

        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return false;
            },
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return false,
        };

        let uniforms = Uniforms {
            resolution: [self.config.width as f32, self.config.height as f32],
            time: (time_ms * 0.001) as f32,
            grid_opacity: self.grid_opacity,
        };
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("shader background encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("shader background pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        self.queue.present(frame);
        true
    }
}

fn mark_shader_background_ready() {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };

    if let Err(error) = root.set_attribute(
        SHADER_BACKGROUND_STATUS_ATTRIBUTE,
        SHADER_BACKGROUND_STATUS_READY,
    ) {
        log_error(&format!(
            "failed to mark shader background ready: {error:?}"
        ));
    }
}

fn preferred_present_mode(surface: &Surface<'_>, adapter: &wgpu::Adapter) -> PresentMode {
    surface
        .get_capabilities(adapter)
        .present_modes
        .into_iter()
        .find(|mode| *mode == PresentMode::Fifo)
        .unwrap_or(PresentMode::AutoVsync)
}

fn canvas_size(canvas: &HtmlCanvasElement) -> CanvasSize {
    let rect = canvas.get_bounding_client_rect();
    let css_width = rect.width().max(1.0);
    let css_height = rect.height().max(1.0);
    let device_scale = web_sys::window()
        .map(|window| window.device_pixel_ratio())
        .unwrap_or(1.0)
        .clamp(1.0, MAX_CANVAS_SCALE);
    let pixel_budget_scale = (MAX_CANVAS_PIXELS / (css_width * css_height)).sqrt();
    let scale = device_scale.min(pixel_budget_scale).max(MIN_CANVAS_SCALE);
    let width = (css_width * scale).floor().max(1.0) as u32;
    let height = (css_height * scale).floor().max(1.0) as u32;

    if canvas.width() != width {
        canvas.set_width(width);
    }
    if canvas.height() != height {
        canvas.set_height(height);
    }

    CanvasSize { width, height }
}

struct CanvasSize {
    width: u32,
    height: u32,
}

fn log_error(message: &str) {
    web_sys::console::error_1(&message.into());
}
