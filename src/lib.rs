pub mod geometry;
pub mod keyboard;
pub mod renderer;
mod util;

use geometry::quad::Quad;

use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

pub trait InputHandler {
  fn process_keyboard(&self, input: keyboard::KeyboardInput);
}

pub trait GameState {
  fn initialize(&self);
  fn update(&self);
  fn quads(&self) -> Vec<&Quad>;
}

pub fn start(
  title: &str,
  game_state: Box<dyn GameState>,
  input_handlers: Vec<Box<dyn InputHandler>>,
) {
  env_logger::init();
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title(title)
    .build(&event_loop)
    .unwrap();

  use futures::executor::block_on;

  let mut renderer = block_on(renderer::Renderer::new(&window));

  game_state.initialize();

  event_loop.run(move |event, _, control_flow| match event {
    Event::RedrawRequested(_) => {
      game_state.update();
      renderer.render(&game_state);
    }
    Event::MainEventsCleared => {
      window.request_redraw();
    }
    Event::WindowEvent {
      ref event,
      window_id,
    } if window_id == window.id() => match event {
      WindowEvent::KeyboardInput {
        input:
          winit::event::KeyboardInput {
            virtual_keycode: Some(key),
            state,
            ..
          },
        ..
      } => {
        for input_handler in &input_handlers {
          let keyboard_input = keyboard::KeyboardInput::new(key, state);
          input_handler.process_keyboard(keyboard_input);
        }
      }
      WindowEvent::MouseWheel { delta: _, .. } => {
        // self.camera_controller.process_scroll(delta);
      }
      WindowEvent::MouseInput {
        button: MouseButton::Left,
        state: _,
        ..
      } => {
        // self.mouse_pressed = *state == ElementState::Pressed;
      }
      WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
      WindowEvent::Resized(physical_size) => {
        renderer.resize(*physical_size);
      }
      WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        renderer.resize(**new_inner_size);
      }
      _ => {}
    },
    _ => {}
  });
}
