use crate::util::size_of_slice;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
pub mod quad;
pub mod vertex;
use quad::Quad;

pub struct Geometry {
  vertex_data: Vec<vertex::Vertex>,
  index_data: Vec<u32>,
  current_quad: u32,
}

impl Geometry {
  pub fn new() -> Self {
    Self {
      vertex_data: Vec::new(),
      index_data: Vec::new(),
      current_quad: 0,
    }
  }

  pub fn push_quad(mut self, quad: &Quad) -> Self {
    let min_x = quad.position.x - quad.size.x * 0.5;
    let min_y = quad.position.y - quad.size.y * 0.5;
    let max_x = quad.position.x + quad.size.x * 0.5;
    let max_y = quad.position.y + quad.size.y * 0.5;

    self.vertex_data.extend(&[
      vertex::Vertex {
        position: (min_x, min_y).into(),
      },
      vertex::Vertex {
        position: (max_x, min_y).into(),
      },
      vertex::Vertex {
        position: (max_x, max_y).into(),
      },
      vertex::Vertex {
        position: (min_x, max_y).into(),
      },
    ]);
    self.index_data.extend(&[
      self.current_quad * 4 + 0,
      self.current_quad * 4 + 1,
      self.current_quad * 4 + 2,
      self.current_quad * 4 + 0,
      self.current_quad * 4 + 2,
      self.current_quad * 4 + 3,
    ]);
    self.current_quad += 1;
    self
  }

  pub fn build(self, device: &wgpu::Device) -> (StagingBuffer, StagingBuffer, u32) {
    (
      StagingBuffer::new(device, &self.vertex_data),
      StagingBuffer::new(device, &self.index_data),
      self.index_data.len() as u32,
    )
  }
}

pub struct StagingBuffer {
  buffer: wgpu::Buffer,
  size: wgpu::BufferAddress,
}

impl StagingBuffer {
  pub fn new<T: bytemuck::Pod + Sized>(device: &wgpu::Device, data: &[T]) -> StagingBuffer {
    StagingBuffer {
      buffer: device.create_buffer_init(&BufferInitDescriptor {
        contents: bytemuck::cast_slice(data),
        usage: wgpu::BufferUsage::COPY_SRC,
        label: Some("Staging Buffer"),
      }),
      size: size_of_slice(data) as wgpu::BufferAddress,
    }
  }

  pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, other: &wgpu::Buffer) {
    encoder.copy_buffer_to_buffer(&self.buffer, 0, other, 0, self.size)
  }
}
