// main.rs
mod sim;
mod ui;
mod rendering;
mod config;
mod assets;
mod constants;

use eframe::{egui, NativeOptions, run_native, App, Frame};
use egui::Vec2;

struct FireSimApp {
    simulation: sim::FireSim,
    ui_state: ui::UIState,
    background: assets::ImageAsset,
    border: assets::ImageAsset,
    config: config::AppConfig,
    sim_area: egui::Vec2, // Current simulation drawing area.
    is_fullscreen: bool,  // Fullscreen state.
}

impl Default for FireSimApp {
    fn default() -> Self {
        let config = config::load_config();
        let background = assets::load_background_image(&config.background_path);
        let border = assets::load_border_image(&config.border_path);
        let default_size = Vec2::new(constants::WINDOW_WIDTH as f32, constants::WINDOW_HEIGHT as f32);
        Self {
            simulation: sim::FireSim::new(),
            ui_state: ui::UIState::new(),
            background,
            border,
            config,
            sim_area: default_size,
            is_fullscreen: false,
        }
    }
}

impl App for FireSimApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // Toggle fullscreen with Alt+Enter.
        if ctx.input(|i| i.key_pressed(egui::Key::Enter))
            && ctx.input(|i| i.modifiers.alt)
        {
            self.is_fullscreen = !self.is_fullscreen;
            frame.set_fullscreen(self.is_fullscreen);
        }

        // Show control UI.
        egui::Window::new("Fire Controls").show(ctx, |ui| {
            self.ui_state.build_ui(ui);
        });

        // Use the full screen rect from egui input to detect size changes.
        let new_size = ctx.input(|i| i.screen_rect().size());
        if new_size != self.sim_area {
            // Reinitialize simulation with new dimensions.
            self.simulation = sim::FireSim::with_size(new_size.x, new_size.y);
            self.sim_area = new_size;
        }

        // Use a fixed dt.
        let dt = 1.0 / 60.0;
        self.simulation.update(dt, &self.ui_state.params);
        self.ui_state.thermometer = self.simulation.average_temperature();

        // Draw simulation.
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let (_response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());
            let rect = ui.max_rect();
            rendering::draw_simulation(&painter, rect, &self.simulation, &self.background, &self.border, &self.ui_state);
        });

        ctx.request_repaint();
    }
}

fn main() {
    let app = FireSimApp::default();
    let native_options = NativeOptions {
        initial_window_size: Some(Vec2::new(
            constants::WINDOW_WIDTH as f32,
            constants::WINDOW_HEIGHT as f32,
        )),
        resizable: true,
        ..Default::default()
    };
    run_native("Fire Simulation", native_options, Box::new(|_cc| Box::new(app)));
}
