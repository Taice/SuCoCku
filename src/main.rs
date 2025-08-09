mod frame;
mod settings;
mod sudoku;
mod unwrap_or_else;

use std::{process::exit, rc::Rc};

use directories::ProjectDirs;
use macroquad::prelude::*;
use settings::{Settings, config::Config};

use crate::frame::Frame;

fn window_conf() -> Conf {
    Conf {
        window_title: "Sudoku".to_string(),
        window_width: 500,
        window_height: 500,
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let config = if let Ok(val) = load_config() {
        Some(val)
    } else {
        None
    };
    let settings = Rc::new(Settings::from_config(&config));
    let mut frame = Frame::new(settings);

    loop {
        frame.draw();
        frame.update();
        next_frame().await;
    }
}

fn load_config() -> std::io::Result<Config> {
    let prj_dirs = &ProjectDirs::from("com", "Taice", "Sucocku").unwrap();
    let config_dir = directories::ProjectDirs::config_dir(&prj_dirs);
    let file = config_dir.join("config.toml");
    let content = match std::fs::read_to_string(&file) {
        Ok(x) => x,
        Err(e) => {
            let _ = std::fs::write(file, include_str!("../assets/config.toml"));
            println!("{e}");
            exit(1)
        }
    };
    let config: Config = match toml::from_str(&content) {
        Ok(x) => x,
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };
    Ok(config)
}

pub fn draw_rect_outlines(r: Rect, t: f32, color: Color) {
    draw_rectangle(r.x, r.y, t, r.h, color);
    draw_rectangle(r.x, r.y, r.w, t, color);
    draw_rectangle(r.x + r.w - t, r.y, t, r.h, color);
    draw_rectangle(r.x, r.y + r.h - t, r.w, t, color);
}
