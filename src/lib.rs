pub mod geometry;
pub mod keyboard;
pub mod renderer;
mod util;

use geometry::Geometry;
use renderer::render_text::*;
use renderer::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub trait Game {
    fn initialize(
        &mut self,
        geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        window_size: (f32, f32),
    );
    fn update(&mut self, geometry: &mut Geometry, text_renderer: &mut TextRenderer);
    fn process_keyboard(&mut self, input: keyboard::KeyboardInput);
    fn is_quitting(&self) -> bool;
}

pub fn start(title: &str, mut game: Box<dyn Game>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    use futures::executor::block_on;

    let mut renderer = block_on(Renderer::new(&window));
    let mut geometry = Geometry::new();
    let mut text_renderer = TextRenderer::new();

    game.initialize(
        &mut geometry,
        &mut text_renderer,
        (renderer.width(), renderer.height()),
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if game.is_quitting() == true {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };

        match event {
            Event::RedrawRequested(_) => {
                game.update(&mut geometry, &mut text_renderer);
                renderer.render(&geometry, &text_renderer);
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
                    let keyboard_input = keyboard::KeyboardInput::new(key, state);
                    game.process_keyboard(keyboard_input);
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
        }
    });
}
