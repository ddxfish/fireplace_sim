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
    pub size: f32, // Visual size.
}

pub struct FireSim {
    pub particles: Vec<Particle>,
    pub width: f32,
    pub height: f32,
}

impl FireSim {
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
        let dt_eff = dt * params.simulation_speed;
        let initial_heat_size = 18.0;
        let mut new_particles = Vec::new();
        
        for p in self.particles.iter_mut() {
            // Add a random factor between 0.8 and 1.2 to lifetime reduction.
            let random_factor = 0.8 + rand::random::<f32>() * 0.4;
            match p.kind {
                ParticleType::Fuel => {
                    p.y = self.height - 10.0;
                    p.lifetime -= dt_eff * 10.0 * random_factor;
                },
                ParticleType::Heat => {
                    p.x += p.vx * dt_eff;
                    p.y += p.vy * dt_eff;
                    p.vx += params.wind * dt_eff;
                    p.temp -= params.simulation_speed * dt_eff;
                    p.lifetime -= dt_eff * 10.0 * random_factor;
                    let initial_life = params.oxygen;
                    p.size = (p.lifetime / initial_life) * initial_heat_size;
                    // When heat particles shrink too small, they spawn smoke.
                    if p.size < 4.0 {
                        if params.enable_smoke {
                            new_particles.push(Particle {
                                x: p.x,
                                y: p.y,
                                vx: (rand::random::<f32>() - 0.5) * 10.0,
                                vy: -rand::random::<f32>() * 30.0 - 20.0,
                                temp: 50.0,
                                lifetime: 64.0, // Smoke lasts longer.
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
                    // Ember lifetime reduction increases rapidly once they leave the bottom 50 pixels.
                    let death_factor = if p.y > self.height - 50.0 {
                        5.0
                    } else {
                        5.0 + 10.0 * (((self.height - p.y) / 50.0).powf(2.0))
                    };
                    p.lifetime -= dt_eff * death_factor * random_factor;
                },
                ParticleType::Smoke => {
                    p.x += p.vx * dt_eff;
                    p.y += p.vy * dt_eff;
                    p.lifetime -= dt_eff * 10.0 * random_factor;
                    // Smoke particles expand over time.
                    p.size += 2.0 * dt_eff;
                },
            }
        }
        self.particles.retain(|p| p.lifetime > 0.0);
        self.particles.append(&mut new_particles);
        
        // Spawn new heat particles based on Particle Count.
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
                size: initial_heat_size,
            });
        }
        if params.enable_smoke {
            let smoke_spawn_count = ((params.smoke_amount / 100.0) * 3.0).max(1.0) as usize;
            for _ in 0..smoke_spawn_count {
                self.particles.push(Particle {
                    x: rand::random::<f32>() * self.width,
                    y: self.height - 20.0,
                    vx: (rand::random::<f32>() - 0.5) * 10.0,
                    vy: -rand::random::<f32>() * 30.0 - 20.0,
                    temp: 50.0,
                    lifetime: 64.0,
                    kind: ParticleType::Smoke,
                    size: 4.0,
                });
            }
        }
        if params.enable_sparks {
            let base_ember_count = ((params.fuel_amount / 100.0) * (params.oxygen / 50.0)).max(1.0);
            let ember_count = ((base_ember_count * (params.ember_amount / 100.0)) / 12.0).max(1.0) as usize;
            for _ in 0..ember_count {
                self.particles.push(Particle {
                    x: rand::random::<f32>() * self.width,
                    y: self.height - 15.0,
                    vx: (rand::random::<f32>() - 0.5) * 30.0,
                    vy: -rand::random::<f32>() * 70.0 - 40.0,
                    temp: 120.0,
                    lifetime: params.oxygen / 2.0 + 10.0,
                    kind: ParticleType::Ember,
                    size: 3.0,
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
