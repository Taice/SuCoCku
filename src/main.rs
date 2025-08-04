mod settings;
mod sudoku;

use std::{io::ErrorKind, process::exit, time::Instant};

use directories::ProjectDirs;
use macroquad::prelude::*;
use settings::{Settings, config::Config};
use sudoku::Sudoku;

#[macro_export]
macro_rules! time {
    ($s:literal, $instant:expr) => {
        println!("{} took {}ms to load", $s, $instant.elapsed().as_millis());
        $instant = Instant::now();
    };
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sudoku".to_string(),
        window_width: 500,
        window_height: 500,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let mut timer = Instant::now();
    let config = if let Ok(val) = load_config() {
        Some(val)
    } else {
        None
    };
    time!("config", timer);
    let mut settings = Settings::from_config(&config);
    time!("settings", timer);

    let mut sudoku = Sudoku::default();
    time!("settings", timer);
    let _ = timer;
    loop {
        clear_background(GRAY);
        sudoku.draw(&settings);
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
            return Err(e);
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
