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
}

impl Default for FireSimApp {
    fn default() -> Self {
        let config = config::load_config();
        let background = assets::load_background_image(&config.background_path);
        let border = assets::load_border_image(&config.border_path);
        Self {
            simulation: sim::FireSim::new(),
            ui_state: ui::UIState::new(),
            background,
            border,
            config,
        }
    }
}

impl App for FireSimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Show control window with sliders/checkboxes.
        egui::Window::new("Fire Controls").show(ctx, |ui| {
            self.ui_state.build_ui(ui);
        });

        // Use a fixed dt (1/60 sec) for simplicity.
        let dt = 1.0 / 60.0;

        // Update simulation.
        self.simulation.update(dt, &self.ui_state.params);
        self.ui_state.thermometer = self.simulation.average_temperature();

        // Draw simulation in the central panel.
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
        initial_window_size: Some(Vec2::new(constants::WINDOW_WIDTH as f32, constants::WINDOW_HEIGHT as f32)),
        resizable: true,
        ..Default::default()
    };
    run_native("Fire Simulation", native_options, Box::new(|_cc| Box::new(app)));
}
