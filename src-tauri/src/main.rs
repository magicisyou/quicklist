// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod list;
mod state;

use list::{Item, ListItem};
use state::{AppState, ServiceAccess};

use tauri::{AppHandle, Manager, State};

#[tauri::command]
fn add_item_to_list(app_handle: AppHandle, list: String, item: String) -> Option<Vec<Item>> {
    if let Ok(list) =
        app_handle.db(|db| database::add_item(db, &ListItem::from(&list, &item, false)))
    {
        return Some(list);
    }
    None
}

#[tauri::command]
fn toggle_checked_value(
    app_handle: AppHandle,
    list_name: String,
    item_name: String,
) -> Option<Vec<Item>> {
    if let Ok(list) = app_handle.db(|db| database::toggle_checked(db, &list_name, &item_name)) {
        return Some(list);
    }
    None
}

#[tauri::command]
fn get_list(app_handle: AppHandle, list_name: String) -> Option<Vec<Item>> {
    if let Ok(list) = app_handle.db(|db| database::get_list(db, &list_name)) {
        return Some(list);
    }
    None
}

#[tauri::command]
fn remove_item_from_list(
    app_handle: AppHandle,
    list_name: String,
    item_name: String,
) -> Option<Vec<Item>> {
    if let Ok(list) = app_handle.db(|db| database::delete_item(db, &list_name, &item_name)) {
        return Some(list);
    }
    None
}

#[tauri::command]
fn delete_list(app_handle: AppHandle, list_name: String) -> Option<Vec<String>> {
    if let Ok(list_names) = app_handle.db(|db| database::delete_list(db, &list_name)) {
        return Some(list_names);
    }
    None
}

#[tauri::command]
fn get_list_names(app_handle: AppHandle) -> Option<Vec<String>> {
    if let Ok(list_names) = app_handle.db(database::get_list_names) {
        return Some(list_names);
    }
    None
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .setup(|app| {
            let handle = app.handle();
            let app_state: State<AppState> = handle.state();
            let db =
                database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_item_to_list,
            toggle_checked_value,
            get_list,
            remove_item_from_list,
            get_list_names,
            delete_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
