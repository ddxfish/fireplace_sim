// rendering.rs
use crate::sim::{FireSim, ParticleType};
use crate::ui::UIState;
use crate::assets::ImageAsset;
use crate::constants;
use egui::{Painter, Rect, Color32, Pos2};

pub fn draw_simulation(painter: &Painter, rect: Rect, sim: &FireSim, _background: &ImageAsset, _border: &ImageAsset, ui: &UIState) {
    painter.rect_filled(rect, 0.0, Color32::BLACK);
    
    for p in &sim.particles {
        let pos = Pos2::new(p.x, p.y);
        let size = p.size;
        let color = match p.kind {
            ParticleType::Fuel => Color32::from_rgb(100, 50, 0),
            ParticleType::Heat => {
                let ratio = (p.size / 18.0).clamp(0.0, 1.0);
                let r = 200 + (55.0 * ratio) as u8;
                let g = 70 + (95.0 * ratio) as u8;
                Color32::from_rgb(r, g, 0)
            },
            ParticleType::Smoke => {
                let ratio = (4.0 / p.size).clamp(0.0, 1.0);
                let alpha = (150.0 * ratio).min(150.0).max(30.0) as u8;
                Color32::from_rgba_unmultiplied(100, 100, 100, alpha)
            },
            ParticleType::Ember => Color32::from_rgb(255, 200, 50),
        };
        painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(size, size)), 0.0, color);
    }
    
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
    
    painter.rect_stroke(rect, 0.0, (2.0, Color32::WHITE));
}
