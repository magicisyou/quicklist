use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    checked: bool,
}

impl Item {
    fn from(name: String, checked: bool) -> Self {
        Self { name, checked }
    }
}

pub struct List {
    items: HashMap<String, bool>,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    // Update list values
    pub fn update(&mut self, items: Vec<Item>) {
        let mut map = HashMap::new();
        for item in &items {
            map.insert(item.name.clone(), item.checked);
        }
        self.items = map;
    }

    // Insert new item to list
    pub fn insert(&mut self, item_name: String) {
        self.items.entry(item_name).or_insert(false);
    }

    // Toggle the checked value of an item in list
    pub fn toggle(&mut self, item_name: &str) {
        if let Some(checked) = self.items.get_mut(item_name) {
            *checked = !*checked;
        }
    }

    // Return the list as vector
    pub fn get_vec(&self) -> Vec<Item> {
        self.items
            .iter()
            .map(|(key, value)| Item::from(key.to_string(), *value))
            .collect::<Vec<Item>>()
    }

    // Remove an item from list
    pub fn remove(&mut self, item_name: &str) {
        self.items.remove(item_name);
    }
}
