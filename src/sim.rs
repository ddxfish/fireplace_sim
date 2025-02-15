// sim.rs
use crate::constants;
use crate::ui::SimulationParams;

#[derive(Copy, Clone)]
pub enum ParticleType {
    Fuel,
    Heat,
    Smoke,
    Ember,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub temp: f32,
    pub lifetime: f32,
    pub kind: ParticleType,
}

pub struct FireSim {
    pub particles: Vec<Particle>,
}

impl FireSim {
    pub fn new() -> Self {
        let mut particles = Vec::new();
        for i in 0..constants::INITIAL_FUEL_PARTICLES {
            particles.push(Particle {
                x: (i as f32 / constants::INITIAL_FUEL_PARTICLES as f32) * constants::WINDOW_WIDTH as f32,
                y: constants::WINDOW_HEIGHT as f32 - 10.0,
                vx: 0.0,
                vy: 0.0,
                temp: 20.0,
                lifetime: 100.0,
                kind: ParticleType::Fuel,
            });
        }
        FireSim { particles }
    }
    
    pub fn update(&mut self, dt: f32, params: &SimulationParams) {
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx += params.wind * dt;
            if let ParticleType::Heat | ParticleType::Ember = p.kind {
                p.temp -= params.cooling_rate * dt;
            }
            p.lifetime -= dt * 10.0;
        }
        self.particles.retain(|p| p.lifetime > 0.0);
        if params.fuel_amount > 0.0 {
            self.spawn_particles(params);
        }
    }
    
    pub fn spawn_particles(&mut self, params: &SimulationParams) {
        self.particles.push(Particle {
            x: rand::random::<f32>() * constants::WINDOW_WIDTH as f32,
            y: constants::WINDOW_HEIGHT as f32 - 15.0,
            vx: (rand::random::<f32>() - 0.5) * 20.0,
            vy: -rand::random::<f32>() * 50.0 - 30.0,
            temp: 100.0,
            lifetime: 2.0,
            kind: ParticleType::Heat,
        });
        if params.enable_smoke {
            self.particles.push(Particle {
                x: rand::random::<f32>() * constants::WINDOW_WIDTH as f32,
                y: constants::WINDOW_HEIGHT as f32 - 20.0,
                vx: (rand::random::<f32>() - 0.5) * 10.0,
                vy: -rand::random::<f32>() * 30.0 - 20.0,
                temp: 50.0,
                lifetime: 3.0,
                kind: ParticleType::Smoke,
            });
        }
        if params.enable_sparks {
            self.particles.push(Particle {
                x: rand::random::<f32>() * constants::WINDOW_WIDTH as f32,
                y: constants::WINDOW_HEIGHT as f32 - 15.0,
                vx: (rand::random::<f32>() - 0.5) * 30.0,
                vy: -rand::random::<f32>() * 70.0 - 40.0,
                temp: 120.0,
                lifetime: 1.5,
                kind: ParticleType::Ember,
            });
        }
    }
    
    pub fn average_temperature(&self) -> f32 {
        if self.particles.is_empty() { 0.0 } else { self.particles.iter().map(|p| p.temp).sum::<f32>() / self.particles.len() as f32 }
    }
}
