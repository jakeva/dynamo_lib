pub const UNBOUNDED_F32: f32 = std::f32::INFINITY;

#[derive(Debug)]
pub struct RenderText {
  pub position: cgmath::Vector2<f32>,
  pub bounds: cgmath::Vector2<f32>,
  pub color: cgmath::Vector4<f32>,
  pub text: String,
  pub size: f32,
  pub visible: bool,
  pub focused: bool,
  pub centered: bool,
}

impl Default for RenderText {
  fn default() -> Self {
    Self {
      position: (0.0, 0.0).into(),
      bounds: (UNBOUNDED_F32, UNBOUNDED_F32).into(),
      color: (1.0, 1.0, 1.0, 1.0).into(),
      text: String::new(),
      size: 16.0,
      visible: false,
      focused: false,
      centered: false,
    }
  }
}

pub struct TextRenderer<'a> {
  pub render_texts: Vec<&'a RenderText>,
}

impl<'a> TextRenderer<'a> {
  pub fn new() -> Self {
    Self {
      render_texts: Vec::new(),
    }
  }

  pub fn reset(&mut self) {
    self.render_texts.clear();
  }

  pub fn push_render_text(&mut self, text: &'a RenderText) {
    self.render_texts.push(text);
  }
}
