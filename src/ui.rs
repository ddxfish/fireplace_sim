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
}

pub struct UIState {
    pub params: SimulationParams,
    pub thermometer: f32,
    pub grid_overlay: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            params: SimulationParams {
                fuel_amount: 50.0,       // Particle Count (0–1000)
                oxygen: 10.0,            // Particle Lifespan (50–300)
                wind: 0.0,               // Wind (-100 to 100)
                simulation_speed: 1.0,   // Simulation Speed (0–50)
                enable_sparks: true,
                enable_smoke: true,
            },
            thermometer: 0.0,
            grid_overlay: false,
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
        ui.add(Slider::new(&mut self.params.simulation_speed, 0.0..=50.0));
        ui.checkbox(&mut self.params.enable_sparks, "Enable Sparks");
        ui.checkbox(&mut self.params.enable_smoke, "Enable Smoke");
        ui.checkbox(&mut self.grid_overlay, "Grid Overlay");
        ui.label(format!("Temperature: {:.1}°C", self.thermometer));
    }
}
