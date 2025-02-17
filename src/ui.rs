// ui.rs
use egui::{Slider, Checkbox, Ui};

#[derive(Clone)]
pub struct SimulationParams {
    pub fuel_amount: f32,         // Particle Count
    pub oxygen: f32,              // Particle Lifespan
    pub wind: f32,
    pub simulation_speed: f32,    // Simulation Speed
    pub enable_sparks: bool,
    pub enable_smoke: bool,
    pub ember_amount: f32,        // Ember Amount (0-100)
    pub smoke_amount: f32,        // Smoke Amount (0-100)
    pub flicker: bool,            // Flicker effect
}

pub struct UIState {
    pub params: SimulationParams,
    pub eight_bit_mode: bool, // true for 8-bit style, false for normal
}

impl UIState {
    pub fn new() -> Self {
        Self {
            params: SimulationParams {
                fuel_amount: 50.0,
                oxygen: 10.0,
                wind: 0.0,
                simulation_speed: 1.0,
                enable_sparks: true,
                enable_smoke: true,
                ember_amount: 50.0,
                smoke_amount: 50.0,
                flicker: false,
            },
            eight_bit_mode: false,
        }
    }
    
    pub fn build_ui(&mut self, ui: &mut Ui) {
        ui.label("Particle Count");
        ui.add(Slider::new(&mut self.params.fuel_amount, 0.0..=1000.0));
        ui.label("Particle Lifespan");
        ui.add(Slider::new(&mut self.params.oxygen, 50.0..=300.0));
        ui.label("Wind");
        ui.add(Slider::new(&mut self.params.wind, -100.0..=100.0));
        ui.label("Simulation Speed");
        ui.add(Slider::new(&mut self.params.simulation_speed, 0.0..=15.0));
        ui.label("Ember Amount");
        ui.add(Slider::new(&mut self.params.ember_amount, 0.0..=100.0));
        ui.label("Smoke Amount");
        ui.add(Slider::new(&mut self.params.smoke_amount, 0.0..=100.0));
        ui.checkbox(&mut self.params.enable_sparks, "Enable Sparks");
        ui.checkbox(&mut self.params.enable_smoke, "Enable Smoke");
        ui.checkbox(&mut self.params.flicker, "Flicker Effect");
        ui.checkbox(&mut self.eight_bit_mode, "8-bit Mode");
    }
}
