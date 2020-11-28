use crate::geometry::quad::*;

pub struct RenderGeometry {
    pub quads: Vec<Quad>,
}

impl RenderGeometry {
    pub fn new() -> RenderGeometry {
        RenderGeometry { quads: Vec::new() }
    }

    pub fn set_quads(&mut self, quads: Vec<Quad>) {
        self.quads = quads;
    }
}
