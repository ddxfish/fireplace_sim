// main.rs
mod sim;
mod ui;
mod rendering;
mod config;
mod assets;
mod constants;

use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use pixels::{Pixels, SurfaceTexture};
use egui::{Context, FontDefinitions};
use egui_winit::State as EguiWinitState;

fn main() {
    // Create window and event loop.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fire Simulation")
        .with_inner_size(LogicalSize::new(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    // Create Pixels.
    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT, surface_texture).unwrap()
    };

    // Load configuration and assets.
    let config = config::load_config();
    let background = assets::load_background_image(&config.background_path);
    let border = assets::load_border_image(&config.border_path);

    // Initialize simulation and UI state.
    let mut simulation = sim::FireSim::new();
    let mut ui_state = ui::UIState::new();

    // Set up egui (for input processing only; UI overlay not rendered).
    let mut egui_state = EguiWinitState::new(&window);
    let mut egui_ctx = Context::default();
    egui_ctx.set_fonts(FontDefinitions::default());

    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Handle window events.
        match &event {
            Event::WindowEvent { event, .. } => {
                // Process egui events (input state update).
                let _ = egui_state.on_event(&egui_ctx, event);

                match event {
                    WindowEvent::Resized(new_size) => {
                        pixels.resize_surface(new_size.width, new_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        pixels.resize_surface(new_inner_size.width, new_inner_size.height);
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Return),
                            ..
                        } = input
                        {
                            if input.modifiers.alt() {
                                if window.fullscreen().is_some() {
                                    window.set_fullscreen(None);
                                } else {
                                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                                }
                            }
                        }
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
            _ => {}
        }

        // Update and render.
        match event {
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt = now.duration_since(last_frame).as_secs_f32();
                last_frame = now;

                // Run egui frame to update UI state (UI not drawn).
                let raw_input = egui_state.take_egui_input(&window);
                let _ = egui_ctx.run(raw_input, |ctx| {
                    ui_state.build_ui(ctx);
                });
                // (The egui UI is not rendered to the screen in this version.)

                // Update simulation based on (possibly changed) UI parameters.
                simulation.update(dt, &ui_state.params);
                ui_state.thermometer = simulation.average_temperature();

                // Render simulation.
                let frame = pixels.frame_mut();
                rendering::draw_frame(frame, &simulation, &background, &border, &ui_state);

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}
