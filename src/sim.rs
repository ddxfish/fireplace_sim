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
    pub width: f32,
    pub height: f32,
}

impl FireSim {
    pub fn with_size(width: f32, height: f32) -> Self {
        let mut particles = Vec::new();
        // Create fuel particles along the bottom.
        for i in 0..constants::INITIAL_FUEL_PARTICLES {
            particles.push(Particle {
                x: (i as f32 / constants::INITIAL_FUEL_PARTICLES as f32) * width,
                y: height - 10.0,
                vx: 0.0,
                vy: 0.0,
                temp: 20.0,
                lifetime: 300.0,
                kind: ParticleType::Fuel,
            });
        }
        FireSim { particles, width, height }
    }
    
    pub fn new() -> Self {
        Self::with_size(constants::WINDOW_WIDTH as f32, constants::WINDOW_HEIGHT as f32)
    }
    
    pub fn update(&mut self, dt: f32, params: &SimulationParams) {
        for p in self.particles.iter_mut() {
            match p.kind {
                ParticleType::Fuel => {
                    // Keep fuel particles fixed at the bottom.
                    p.y = self.height - 10.0;
                    p.lifetime -= dt * 10.0;
                }
                _ => {
                    p.x += p.vx * dt;
                    p.y += p.vy * dt;
                    p.vx += params.wind * dt;
                    if let ParticleType::Heat | ParticleType::Ember = p.kind {
                        p.temp -= params.cooling_rate * dt;
                    }
                    p.lifetime -= dt * 10.0;
                }
            }
        }
        self.particles.retain(|p| p.lifetime > 0.0);
        if params.fuel_amount > 0.0 {
            self.spawn_particles(params);
        }
    }
    
    pub fn spawn_particles(&mut self, params: &SimulationParams) {
        // Spawn two heat particles.
        for _ in 0..2 {
            self.particles.push(Particle {
                x: rand::random::<f32>() * self.width,
                y: self.height - 15.0,
                vx: (rand::random::<f32>() - 0.5) * 20.0,
                vy: -rand::random::<f32>() * 50.0 - 30.0,
                temp: 100.0,
                lifetime: 6.0,
                kind: ParticleType::Heat,
            });
        }
        if params.enable_smoke {
            for _ in 0..2 {
                self.particles.push(Particle {
                    x: rand::random::<f32>() * self.width,
                    y: self.height - 20.0,
                    vx: (rand::random::<f32>() - 0.5) * 10.0,
                    vy: -rand::random::<f32>() * 30.0 - 20.0,
                    temp: 50.0,
                    lifetime: 8.0,
                    kind: ParticleType::Smoke,
                });
            }
        }
        if params.enable_sparks {
            for _ in 0..2 {
                self.particles.push(Particle {
                    x: rand::random::<f32>() * self.width,
                    y: self.height - 15.0,
                    vx: (rand::random::<f32>() - 0.5) * 30.0,
                    vy: -rand::random::<f32>() * 70.0 - 40.0,
                    temp: 120.0,
                    lifetime: 4.0,
                    kind: ParticleType::Ember,
                });
            }
        }
    }
    
    pub fn average_temperature(&self) -> f32 {
        if self.particles.is_empty() {
            0.0
        } else {
            self.particles.iter().map(|p| p.temp).sum::<f32>() / self.particles.len() as f32
        }
    }
}
