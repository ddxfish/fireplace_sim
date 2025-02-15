// config.rs
use std::fs;

pub struct AppConfig {
    pub background_path: String,
    pub border_path: String,
}

pub fn load_config() -> AppConfig {
    if let Ok(contents) = fs::read_to_string("config.ini") {
        let mut background_path = "background.png".to_string();
        let mut border_path = "border.png".to_string();
        for line in contents.lines() {
            if line.starts_with("background=") {
                background_path = line["background=".len()..].trim().to_string();
            } else if line.starts_with("border=") {
                border_path = line["border=".len()..].trim().to_string();
            }
        }
        AppConfig { background_path, border_path }
    } else {
        AppConfig {
            background_path: "background.png".to_string(),
            border_path: "border.png".to_string(),
        }
    }
}

pub fn save_config(config: &AppConfig) {
    let contents = format!("background={}\nborder={}\n", config.background_path, config.border_path);
    let _ = fs::write("config.ini", contents);
}
