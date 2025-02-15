// rendering.rs
use crate::sim::{FireSim, ParticleType};
use crate::ui::UIState;
use crate::assets::ImageAsset;
use crate::constants;

pub fn draw_frame(frame: &mut [u8], sim: &FireSim, background: &ImageAsset, border: &ImageAsset, ui: &UIState) {
    let width = constants::WINDOW_WIDTH as usize;
    let height = constants::WINDOW_HEIGHT as usize;
    // Clear frame.
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
        pixel[3] = 255;
    }
    
    // Draw background.
    if !background.data.is_empty() {
        for y in 0..height.min(background.height) {
            for x in 0..width.min(background.width) {
                let idx_frame = (y * width + x) * 4;
                let idx_bg = (y * background.width + x) * 4;
                frame[idx_frame..idx_frame+4].copy_from_slice(&background.data[idx_bg..idx_bg+4]);
            }
        }
    }
    
    // Draw particles.
    for p in &sim.particles {
        let ix = p.x as i32;
        let iy = p.y as i32;
        if ix < 0 || iy < 0 || ix >= width as i32 || iy >= height as i32 {
            continue;
        }
        let idx = (iy as usize * width + ix as usize) * 4;
        match p.kind {
            ParticleType::Fuel => { frame[idx] = 100; frame[idx+1] = 50; frame[idx+2] = 0; frame[idx+3] = 255; },
            ParticleType::Heat => { frame[idx] = 255; frame[idx+1] = 100; frame[idx+2] = 0; frame[idx+3] = 255; },
            ParticleType::Smoke => { frame[idx] = 100; frame[idx+1] = 100; frame[idx+2] = 100; frame[idx+3] = 150; },
            ParticleType::Ember => { frame[idx] = 255; frame[idx+1] = 200; frame[idx+2] = 50; frame[idx+3] = 255; },
        }
    }
    
    // Draw grid overlay.
    if ui.grid_overlay {
        let grid_size = 20;
        for y in (0..height).step_by(grid_size) {
            for x in 0..width {
                let idx = (y * width + x) * 4;
                frame[idx] = 50; frame[idx+1] = 50; frame[idx+2] = 50; frame[idx+3] = 255;
            }
        }
        for x in (0..width).step_by(grid_size) {
            for y in 0..height {
                let idx = (y * width + x) * 4;
                frame[idx] = 50; frame[idx+1] = 50; frame[idx+2] = 50; frame[idx+3] = 255;
            }
        }
    }
    
    // Draw border.
    if !border.data.is_empty() {
        for y in 0..height.min(border.height) {
            for x in 0..width.min(border.width) {
                let idx_frame = (y * width + x) * 4;
                let idx_border = (y * border.width + x) * 4;
                if border.data[idx_border+3] > 0 {
                    frame[idx_frame..idx_frame+4].copy_from_slice(&border.data[idx_border..idx_border+4]);
                }
            }
        }
    }
}
