
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, Submenu};
use std::fs::File;
use std::io::{Read, Write};
use log::{info, warn};
use tauri::Manager;

fn main() {
    // Initialize logger
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let window = handle.get_window("main").unwrap();
                window.emit("event-name", "Welcome to RustDeskApp!").unwrap();
            });
            Ok(())
        })
        .menu(create_menu())
        .invoke_handler(tauri::generate_handler![read_file, write_file, call_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Create menu for the app
fn create_menu() -> Menu {
    let file_menu = Submenu::new("File", Menu::new().add_item(CustomMenuItem::new("quit", "Quit")));
    let help_menu = Submenu::new("Help", Menu::new().add_item(CustomMenuItem::new("about", "About")));
    Menu::new().add_submenu(file_menu).add_submenu(help_menu)
}

// Read a file
#[tauri::command]
fn read_file(file_path: &str) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|_| "File not found".to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| "Unable to read file".to_string())?;
    Ok(contents)
}

// Write to a file
#[tauri::command]
fn write_file(file_path: &str, content: &str) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|_| "Unable to create file".to_string())?;
    file.write_all(content.as_bytes()).map_err(|_| "Unable to write to file".to_string())?;
    Ok(())
}

// Example API call
#[tauri::command]
fn call_api() -> Result<String, String> {
    info!("API call initiated");
    Ok("This is a mock API response.".to_string())
}
