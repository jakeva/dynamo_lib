#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub position: cgmath::Vector2<f32>,
    pub size: cgmath::Vector2<f32>,
}

impl Quad {
    pub fn new(position: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>) -> Quad {
        Quad {
            position: position,
            size: size,
        }
    }
}
