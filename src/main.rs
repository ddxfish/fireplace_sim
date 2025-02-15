// main.rs
mod sim;
mod ui;
mod rendering;
mod config;
mod assets;
mod constants;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use egui_winit::State as EguiWinitState;
use egui::{Context, FontDefinitions};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fire Simulation")
        .with_inner_size(winit::dpi::LogicalSize::new(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT, surface_texture).unwrap()
    };

    let config = config::load_config();
    let background = assets::load_background_image(&config.background_path);
    let border = assets::load_border_image(&config.border_path);

    let mut simulation = sim::FireSim::new();
    let mut ui_state = ui::UIState::new();
    
    let mut egui_state = EguiWinitState::new(&window);
    let mut egui_ctx = Context::default();
    egui_ctx.set_fonts(FontDefinitions::default());
    
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let _ = egui_state.on_event(&egui_ctx, &event);
        
        match event {
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt = now.duration_since(last_frame).as_secs_f32();
                last_frame = now;
                
                simulation.update(dt, &ui_state.params);
                ui_state.thermometer = simulation.average_temperature();
                
                let raw_input = egui_state.take_egui_input(&window);
                let full_output = egui_ctx.run(raw_input, |ctx| {
                    ui_state.build_ui(ctx);
                });
                egui_state.handle_platform_output(&window, &egui_ctx, full_output.platform_output);
                
                let frame = pixels.get_frame_mut();
                rendering::draw_frame(frame, &simulation, &background, &border, &ui_state);
                
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
