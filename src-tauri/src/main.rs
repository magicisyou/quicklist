// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod list;

use list::{Item, List};

use std::sync::Mutex;
use tauri::State;

struct AppState {
    list: Mutex<List>,
}

#[tauri::command]
fn add_item_to_list(state: State<AppState>, item: String) -> Vec<Item> {
    state.list.lock().unwrap().insert(item);
    state.list.lock().unwrap().get_vec()
}

#[tauri::command]
fn toggle_checked_value(state: State<AppState>, item_name: String) -> Vec<Item> {
    state.list.lock().unwrap().toggle(&item_name);
    state.list.lock().unwrap().get_vec()
}

#[tauri::command]
fn build_list_from_memory(state: State<AppState>, items: Vec<Item>) -> Vec<Item> {
    state.list.lock().unwrap().update(items);
    state.list.lock().unwrap().get_vec()
}

#[tauri::command]
fn get_list_state(state: State<AppState>) -> Vec<Item> {
    state.list.lock().unwrap().get_vec()
}

#[tauri::command]
fn remove_item_from_list(state: State<AppState>, item_name: String) -> Vec<Item> {
    state.list.lock().unwrap().remove(&item_name);
    state.list.lock().unwrap().get_vec()
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            list: Mutex::new(List::new()),
        })
        .invoke_handler(tauri::generate_handler![
            add_item_to_list,
            toggle_checked_value,
            build_list_from_memory,
            get_list_state,
            remove_item_from_list,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
