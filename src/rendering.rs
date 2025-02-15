// rendering.rs
use crate::sim::{FireSim, ParticleType};
use crate::ui::UIState;
use crate::assets::ImageAsset;
use egui::{Painter, Rect, Color32, Pos2};

pub fn draw_simulation(painter: &Painter, rect: Rect, sim: &FireSim, _background: &ImageAsset, _border: &ImageAsset, ui: &UIState) {
    if ui.eight_bit_mode {
        // 8-bit style: partition into 10x10 cells.
        let grid_size = 10.0;
        let cells_x = ((rect.width()) / grid_size).ceil() as usize;
        let cells_y = ((rect.height()) / grid_size).ceil() as usize;
        let mut cell_data = vec![(0u32, 0u32, 0u32, 0u32, 0u32); cells_x * cells_y];
        
        for p in &sim.particles {
            let pos = Pos2::new(p.x, p.y);
            if rect.contains(pos) {
                let cell_x = (((p.x - rect.min.x) / grid_size).floor() as usize).min(cells_x - 1);
                let cell_y = (((p.y - rect.min.y) / grid_size).floor() as usize).min(cells_y - 1);
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
        for cell_y in 0..cells_y {
            for cell_x in 0..cells_x {
                let idx = cell_y * cells_x + cell_x;
                let (r, g, b, a, count) = cell_data[idx];
                let cell_rect = Rect::from_min_max(
                    Pos2::new(rect.min.x + cell_x as f32 * grid_size, rect.min.y + cell_y as f32 * grid_size),
                    Pos2::new(rect.min.x + (cell_x as f32 + 1.0) * grid_size, rect.min.y + (cell_y as f32 + 1.0) * grid_size)
                );
                let mut avg_color = if count > 0 {
                    Color32::from_rgba_unmultiplied((r / count) as u8, (g / count) as u8, (b / count) as u8, (a / count) as u8)
                } else {
                    Color32::BLACK
                };
                if ui.params.flicker {
                    let factor = 0.9 + rand::random::<f32>() * 0.2;
                    let (fr, fg, fb, fa) = (avg_color.r() as f32 * factor, avg_color.g() as f32 * factor, avg_color.b() as f32 * factor, avg_color.a() as f32);
                    avg_color = Color32::from_rgba_unmultiplied(fr.min(255.0) as u8, fg.min(255.0) as u8, fb.min(255.0) as u8, fa.min(255.0) as u8);
                }
                painter.rect_filled(cell_rect, 0.0, avg_color);
            }
        }
    } else {
        // Normal mode: render each particle.
        painter.rect_filled(rect, 0.0, Color32::BLACK);
        for p in &sim.particles {
            let pos = Pos2::new(p.x, p.y);
            let mut color = match p.kind {
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
            if ui.params.flicker {
                let factor = 0.9 + rand::random::<f32>() * 0.2;
                let (r, g, b, a) = (color.r() as f32 * factor, color.g() as f32 * factor, color.b() as f32 * factor, color.a() as f32);
                color = Color32::from_rgba_unmultiplied(r.min(255.0) as u8, g.min(255.0) as u8, b.min(255.0) as u8, a.min(255.0) as u8);
            }
            painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(p.size, p.size)), 0.0, color);
        }
    }
    painter.rect_stroke(rect, 0.0, (2.0, Color32::WHITE));
}
