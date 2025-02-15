// ui.rs
use egui::{Slider, Checkbox, Ui};

#[derive(Clone)]
pub struct SimulationParams {
    pub fuel_amount: f32,
    pub oxygen: f32,
    pub wind: f32,
    pub cooling_rate: f32,
    pub spark_intensity: f32,
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
                fuel_amount: 50.0,
                oxygen: 21.0,
                wind: 0.0,
                cooling_rate: 5.0,
                spark_intensity: 1.0,
                enable_sparks: true,
                enable_smoke: true,
            },
            thermometer: 0.0,
            grid_overlay: false,
        }
    }
    
    pub fn build_ui(&mut self, ui: &mut Ui) {
        ui.label("Fuel Amount");
        ui.add(Slider::new(&mut self.params.fuel_amount, 0.0..=100.0));
        ui.label("Oxygen Level");
        ui.add(Slider::new(&mut self.params.oxygen, 0.0..=30.0));
        ui.label("Wind");
        ui.add(Slider::new(&mut self.params.wind, -50.0..=50.0));
        ui.label("Cooling Rate");
        ui.add(Slider::new(&mut self.params.cooling_rate, 0.0..=20.0));
        ui.label("Spark Intensity");
        ui.add(Slider::new(&mut self.params.spark_intensity, 0.0..=5.0));
        ui.checkbox(&mut self.params.enable_sparks, "Enable Sparks");
        ui.checkbox(&mut self.params.enable_smoke, "Enable Smoke");
        ui.checkbox(&mut self.grid_overlay, "Grid Overlay");
        ui.label(format!("Temperature: {:.1}°C", self.thermometer));
    }
}
