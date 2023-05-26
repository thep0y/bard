// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod logger;

#[macro_use]
extern crate log;
extern crate simplelog;

use crate::config::read_config;
use config::APP_CONFIG_DIR;
use logger::{log_level, logger_config};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::fs as file_system;

fn main() {
    let log_level = log_level();
    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
            logger_config(true),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            log_level,
            logger_config(true),
            file_system::File::create(APP_CONFIG_DIR.join("bard.log")).unwrap(),
        ),
    ])
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
