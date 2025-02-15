// rendering.rs
use crate::sim::{FireSim, ParticleType};
use crate::ui::UIState;
use crate::assets::ImageAsset;
use egui::{Painter, Rect, Color32, Pos2};

pub fn draw_simulation(painter: &Painter, rect: Rect, sim: &FireSim, _background: &ImageAsset, _border: &ImageAsset, ui: &UIState) {
    // If grid overlay is enabled, render in chunky 8-bit style.
    if ui.grid_overlay {
        let grid_size = 20.0;
        let cells_x = ((rect.width()) / grid_size).ceil() as usize;
        let cells_y = ((rect.height()) / grid_size).ceil() as usize;
        let mut cell_data = vec![(0u32, 0u32, 0u32, 0u32, 0u32); cells_x * cells_y];
        
        // Accumulate colors for each cell.
        for p in &sim.particles {
            let pos = Pos2::new(p.x, p.y);
            if rect.contains(pos) {
                let cell_x = ((p.x - rect.min.x) / grid_size).floor() as usize;
                let cell_y = ((p.y - rect.min.y) / grid_size).floor() as usize;
                let idx = cell_y * cells_x + cell_x;
                let color = match p.kind {
                    ParticleType::Fuel => (100, 50, 0, 255),
                    ParticleType::Heat => {
                        let ratio = (p.size / 18.0).clamp(0.0, 1.0);
                        let r = 200 + (55.0 * ratio) as u32;
                        let g = 70 + (95.0 * ratio) as u32;
                        (r, g, 0, 255)
                    },
                    ParticleType::Smoke => {
                        let alpha = (150.0 - (p.size - 4.0) * 10.0).clamp(30.0, 150.0) as u32;
                        (100, 100, 100, alpha)
                    },
                    ParticleType::Ember => (255, 200, 50, 255),
                };
                let (r, g, b, a, count) = cell_data[idx];
                cell_data[idx] = (r + color.0, g + color.1, b + color.2, a + color.3, count + 1);
            }
        }
        // Draw each grid cell with the averaged color.
        for cell_y in 0..cells_y {
            for cell_x in 0..cells_x {
                let idx = cell_y * cells_x + cell_x;
                let (r, g, b, a, count) = cell_data[idx];
                let cell_rect = Rect::from_min_max(
                    Pos2::new(rect.min.x + cell_x as f32 * grid_size, rect.min.y + cell_y as f32 * grid_size),
                    Pos2::new(rect.min.x + (cell_x as f32 + 1.0) * grid_size, rect.min.y + (cell_y as f32 + 1.0) * grid_size)
                );
                let avg_color = if count > 0 {
                    Color32::from_rgba_unmultiplied((r / count) as u8, (g / count) as u8, (b / count) as u8, (a / count) as u8)
                } else {
                    Color32::BLACK
                };
                painter.rect_filled(cell_rect, 0.0, avg_color);
            }
        }
    } else {
        // Normal rendering: draw each particle individually.
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
                    let alpha = (150.0 - (p.size - 4.0) * 10.0).clamp(30.0, 150.0) as u8;
                    Color32::from_rgba_unmultiplied(100, 100, 100, alpha)
                },
                ParticleType::Ember => Color32::from_rgb(255, 200, 50),
            };
            painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(size, size)), 0.0, color);
        }
    }
    // Draw grid lines in both modes.
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
    painter.rect_stroke(rect, 0.0, (2.0, Color32::WHITE));
}
