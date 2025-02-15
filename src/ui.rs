// ui.rs
use egui::{Slider, Checkbox, Ui};

#[derive(Clone)]
pub struct SimulationParams {
    pub fuel_amount: f32,         // Particle Count
    pub oxygen: f32,              // Particle Lifespan
    pub wind: f32,
    pub simulation_speed: f32,    // Simulation Speed
    pub spark_intensity: f32,     // Initial Particle Size
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
                fuel_amount: 50.0,         // Default Particle Count
                oxygen: 10.0,              // Default Particle Lifespan
                wind: 0.0,
                simulation_speed: 1.0,     // Default Simulation Speed
                spark_intensity: 12.0,     // Default Initial Particle Size
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
        ui.add(Slider::new(&mut self.params.simulation_speed, 5.0..=50.0));
        ui.label("Initial Particle Size");
        ui.add(Slider::new(&mut self.params.spark_intensity, 50.0..=200.0));
        ui.checkbox(&mut self.params.enable_sparks, "Enable Sparks");
        ui.checkbox(&mut self.params.enable_smoke, "Enable Smoke");
        ui.checkbox(&mut self.grid_overlay, "Grid Overlay");
        ui.label(format!("Temperature: {:.1}°C", self.thermometer));
    }
}
