use std::mem;

pub const U32_SIZE: wgpu::BufferAddress = std::mem::size_of::<u32>() as wgpu::BufferAddress;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: cgmath::Vector2<f32>,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    pub const DESC: wgpu::VertexBufferDescriptor<'static> = wgpu::VertexBufferDescriptor {
        stride: Self::SIZE,
        step_mode: wgpu::InputStepMode::Vertex,
        // What?
        // attributes: &wgpu::vertex_attr_array![
        //     0 => Float2
        // ],
        attributes: &[
            wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float2,
            },
            wgpu::VertexAttributeDescriptor {
                offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float2,
            },
            wgpu::VertexAttributeDescriptor {
                offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float2,
            },
        ],
    };
}
