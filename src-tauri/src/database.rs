use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

use crate::list::{Item, ListItem};

const CURRENT_DB_VERSION: u32 = 1;
const TABLE_NAME: &str = "lists";

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created");
    let sqlite_path = app_dir.join("list.sqlite");
    let mut db = Connection::open(sqlite_path)?;
    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| row.get(0))?;
    drop(user_pragma);
    upgrade_database_if_needed(&mut db, existing_user_version)?;
    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;
        let tx = db.transaction()?;
        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;
        let query = format!(
            "
      CREATE TABLE {TABLE_NAME} (
        list TEXT NOT NULL,
        item TEXT NOT NULL,
        checked BOOL NOT NULL,
        PRIMARY KEY (list, item)
      );"
        );

        tx.execute_batch(&query)?;
        tx.commit()?;
    }
    Ok(())
}

pub fn add_item(db: &Connection, list_item: &ListItem) -> Result<Vec<Item>, rusqlite::Error> {
    let checked = match list_item.checked {
        true => 1,
        false => 0,
    };
    let query =
        format!("INSERT INTO {TABLE_NAME} (list, item, checked) VALUES (@list, @item, @checked)");
    let mut statement = db.prepare(&query)?;
    statement.execute(
        named_params! {"@list": list_item.list, "@item": list_item.item, "@checked": checked },
    )?;
    get_list(db, &list_item.list)
}

pub fn get_list(db: &Connection, list_name: &str) -> Result<Vec<Item>, rusqlite::Error> {
    let query = format!("SELECT item, checked FROM {TABLE_NAME} WHERE list = '{list_name}'");
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query([])?;
    let mut list = Vec::new();
    while let Some(row) = rows.next()? {
        let item: String = row.get("item")?;
        let checked = row.get("checked")?;
        let checked: bool = matches!(checked, 1);
        list.push(Item::from(item, checked));
    }
    Ok(list)
}

pub fn delete_item(
    db: &Connection,
    list_name: &str,
    item_name: &str,
) -> Result<Vec<Item>, rusqlite::Error> {
    let query = format!("DELETE FROM {TABLE_NAME} WHERE item = @item AND list = @list");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@item": item_name, "@list": list_name })?;
    get_list(db, list_name)
}

fn get_checked_value(
    db: &Connection,
    list_name: &str,
    item_name: &str,
) -> Result<bool, rusqlite::Error> {
    let query = format!(
        "SELECT checked FROM {TABLE_NAME} WHERE item = '{item_name}' AND list = '{list_name}'"
    );
    let mut statement = db.prepare(&query)?;
    let mut rows = statement.query([])?;
    if let Some(row) = rows.next()? {
        let checked = row.get("checked")?;
        return Ok(matches!(checked, 0));
    }
    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn toggle_checked(
    db: &Connection,
    list_name: &str,
    item_name: &str,
) -> Result<Vec<Item>, rusqlite::Error> {
    let checked = get_checked_value(db, list_name, item_name)?;
    let query =
        format!("UPDATE {TABLE_NAME} SET checked = {checked} WHERE item = @item AND list = @list");
    let mut statement = db.prepare(&query)?;
    statement.execute(named_params! { "@item": item_name, "@list": list_name })?;
    get_list(db, list_name)
}
