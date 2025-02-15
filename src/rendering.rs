// rendering.rs
use crate::sim::{FireSim, ParticleType};
use crate::ui::UIState;
use crate::assets::ImageAsset;
use crate::constants;
use egui::{Painter, Rect, Color32, Pos2};

pub fn draw_simulation(painter: &Painter, rect: Rect, sim: &FireSim, _background: &ImageAsset, _border: &ImageAsset, ui: &UIState) {
    // Fill the background.
    painter.rect_filled(rect, 0.0, Color32::BLACK);

    // Draw particles as squares.
    for p in &sim.particles {
        let pos = Pos2::new(p.x, p.y);
        let size = constants::PARTICLE_SIZE as f32;
        let color = match p.kind {
            ParticleType::Fuel => Color32::from_rgb(100, 50, 0),
            ParticleType::Heat => Color32::from_rgb(255, 100, 0),
            ParticleType::Smoke => Color32::from_rgba_unmultiplied(100, 100, 100, 150),
            ParticleType::Ember => Color32::from_rgb(255, 200, 50),
        };
        painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(size, size)), 0.0, color);
    }

    // Draw grid overlay if enabled.
    if ui.grid_overlay {
        let grid_size = 20.0;
        let mut x = rect.min.x;
        while x < rect.max.x {
            painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], (1.0, Color32::DARK_GRAY));
            x += grid_size;
        }
        let mut y = rect.min.y;
        while y < rect.max.y {
            painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], (1.0, Color32::DARK_GRAY));
            y += grid_size;
        }
    }

    // Draw border overlay.
    painter.rect_stroke(rect, 0.0, (2.0, Color32::WHITE));
}
