// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn build_menu() -> Menu {
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "Single key shortcuts",
            Menu::new()
                .add_item(CustomMenuItem::new("albums", "Go to Albums").accelerator("A"))
                .add_item(CustomMenuItem::new("library", "Go to Library").accelerator("L"))
                .add_item(CustomMenuItem::new("queue", "Toggle Queue").accelerator("Q"))
                .add_item(
                    CustomMenuItem::new("lyrics", "Toggle Lyrics")
                        .accelerator("CommandOrControl+L"),
                ),
        ));

    return menu;
}

fn main() {
    tauri::Builder::default()
        .menu(build_menu())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
