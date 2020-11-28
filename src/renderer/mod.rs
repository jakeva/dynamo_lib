use crate::geometry::quad::*;
use crate::geometry::vertex::*;
use crate::geometry::Geometry;
use std::iter;
use winit::window::Window;

pub trait RenderGeometry {
    fn set_quads(&mut self, quads: Vec<Quad>);
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    quads: Vec<Quad>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
            push_constant_ranges: &[],
            label: Some("Pipeline Layout"),
        });
        let pipeline = create_render_pipeline(
            &device,
            &pipeline_layout,
            sc_desc.format,
            &[Vertex::DESC],
            wgpu::include_spirv!("../../res/shaders/textured.vert.spv"),
            wgpu::include_spirv!("../../res/shaders/textured.frag.spv"),
        );

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: Vertex::SIZE * 4 * 3,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: U32_SIZE * 6 * 3,
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let quads = Vec::new();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            pipeline,
            quads,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Renderer Encoder"),
            });

        let mut geometry = Geometry::new();

        for quad in self.quads.iter() {
            geometry = geometry.push_quad(quad);
        }

        let (stg_vertex, stg_index, num_indices) = geometry.build(&self.device);
        stg_vertex.copy_to_buffer(&mut encoder, &self.vertex_buffer);
        stg_index.copy_to_buffer(&mut encoder, &self.index_buffer);

        match self.swap_chain.get_current_frame() {
            Ok(frame) => {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations::default(),
                    }],
                    depth_stencil_attachment: None,
                });

                if num_indices != 0 {
                    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(self.index_buffer.slice(..));
                    render_pass.set_pipeline(&self.pipeline);
                    render_pass.draw_indexed(0..num_indices, 0, 0..1);
                }

                drop(render_pass);
                self.queue.submit(iter::once(encoder.finish()));
            }
            Err(wgpu::SwapChainError::Outdated) => {
                self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    vertex_descs: &[wgpu::VertexBufferDescriptor],
    vs_src: wgpu::ShaderModuleSource,
    fs_src: wgpu::ShaderModuleSource,
) -> wgpu::RenderPipeline {
    let vs_module = device.create_shader_module(vs_src);
    let fs_module = device.create_shader_module(fs_src);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: color_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: vertex_descs,
        },
    })
}

impl RenderGeometry for Renderer {
    fn set_quads(&mut self, quads: Vec<Quad>) {
        self.quads = quads;

        self.vertex_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: Vertex::SIZE * 4 * (self.quads.len() as u64),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        self.index_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: U32_SIZE * 6 * (self.quads.len() as u64),
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });
    }
}
