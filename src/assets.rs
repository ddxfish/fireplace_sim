// assets.rs
use image::io::Reader as ImageReader;
use image::GenericImageView;
use crate::constants;

pub struct ImageAsset {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // RGBA8 data.
}

pub fn load_background_image(path: &str) -> ImageAsset {
    if let Some(img) = ImageReader::open(path).ok().and_then(|r| r.decode().ok()) {
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();
        ImageAsset { width: w as usize, height: h as usize, data: img.into_raw() }
    } else {
        ImageAsset { width: constants::WINDOW_WIDTH as usize, height: constants::WINDOW_HEIGHT as usize, data: vec![] }
    }
}

pub fn load_border_image(path: &str) -> ImageAsset {
    if let Some(img) = ImageReader::open(path).ok().and_then(|r| r.decode().ok()) {
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();
        ImageAsset { width: w as usize, height: h as usize, data: img.into_raw() }
    } else {
        ImageAsset { width: constants::WINDOW_WIDTH as usize, height: constants::WINDOW_HEIGHT as usize, data: vec![] }
    }
}
