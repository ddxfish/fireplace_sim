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
    pub size: f32, // Visual size of the particle.
}

pub struct FireSim {
    pub particles: Vec<Particle>,
    pub width: f32,
    pub height: f32,
}

impl FireSim {
    // Create simulation with given dimensions.
    pub fn with_size(width: f32, height: f32) -> Self {
        let mut particles = Vec::new();
        // Spawn fuel particles along the bottom.
        for i in 0..constants::INITIAL_FUEL_PARTICLES {
            particles.push(Particle {
                x: (i as f32 / constants::INITIAL_FUEL_PARTICLES as f32) * width,
                y: height - 10.0,
                vx: 0.0,
                vy: 0.0,
                temp: 20.0,
                lifetime: 300.0,
                kind: ParticleType::Fuel,
                size: 8.0,
            });
        }
        FireSim { particles, width, height }
    }
    
    pub fn new() -> Self {
        Self::with_size(constants::WINDOW_WIDTH as f32, constants::WINDOW_HEIGHT as f32)
    }
    
    pub fn update(&mut self, dt: f32, params: &SimulationParams) {
        // Effective dt scaled by simulation speed.
        let dt_eff = dt * params.simulation_speed;
        let mut new_particles = Vec::new();
        for p in self.particles.iter_mut() {
            match p.kind {
                ParticleType::Fuel => {
                    // Fuel remains at the bottom.
                    p.y = self.height - 10.0;
                    p.lifetime -= dt_eff * 10.0;
                },
                ParticleType::Heat => {
                    p.x += p.vx * dt_eff;
                    p.y += p.vy * dt_eff;
                    p.vx += params.wind * dt_eff;
                    p.temp -= params.simulation_speed * dt_eff;
                    p.lifetime -= dt_eff * 10.0;
                    let initial_life = params.oxygen;
                    p.size = (p.lifetime / initial_life) * params.spark_intensity;
                    if p.size < 4.0 {
                        if params.enable_smoke {
                            new_particles.push(Particle {
                                x: p.x,
                                y: p.y,
                                vx: (rand::random::<f32>() - 0.5) * 10.0,
                                vy: -rand::random::<f32>() * 30.0 - 20.0,
                                temp: 50.0,
                                lifetime: 8.0,
                                kind: ParticleType::Smoke,
                                size: 4.0,
                            });
                        }
                        p.lifetime = 0.0;
                    }
                },
                ParticleType::Ember => {
                    p.x += p.vx * dt_eff;
                    p.y += p.vy * dt_eff;
                    p.vx += params.wind * dt_eff;
                    p.temp -= params.simulation_speed * dt_eff;
                    p.lifetime -= dt_eff * 10.0;
                },
                ParticleType::Smoke => {
                    p.x += p.vx * dt_eff;
                    p.y += p.vy * dt_eff;
                    p.lifetime -= dt_eff * 10.0;
                },
            }
        }
        self.particles.retain(|p| p.lifetime > 0.0);
        self.particles.append(&mut new_particles);
        // Spawn new particles based on Particle Count.
        let spawn_count = (params.fuel_amount / 10.0).max(1.0) as usize;
        for _ in 0..spawn_count {
            self.particles.push(Particle {
                x: rand::random::<f32>() * self.width,
                y: self.height - 15.0,
                vx: (rand::random::<f32>() - 0.5) * 20.0,
                vy: -rand::random::<f32>() * 30.0 - 10.0,
                temp: 100.0 * params.simulation_speed,
                lifetime: params.oxygen,
                kind: ParticleType::Heat,
                size: params.spark_intensity,
            });
        }
        if params.enable_smoke {
            for _ in 0..1 {
                self.particles.push(Particle {
                    x: rand::random::<f32>() * self.width,
                    y: self.height - 20.0,
                    vx: (rand::random::<f32>() - 0.5) * 10.0,
                    vy: -rand::random::<f32>() * 30.0 - 20.0,
                    temp: 50.0,
                    lifetime: 8.0,
                    kind: ParticleType::Smoke,
                    size: 4.0,
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
                    lifetime: 10.0, // Extended ember lifetime.
                    kind: ParticleType::Ember,
                    size: (params.spark_intensity * 0.5).max(2.0),
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
