use serde::{Deserialize, Serialize};

pub struct ListItem {
    pub list: String,
    pub item: String,
    pub checked: bool,
}

impl ListItem {
    pub fn from(list: &str, item: &str, checked: bool) -> Self {
        Self {
            list: list.to_string(),
            item: item.to_string(),
            checked,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    checked: bool,
}

impl Item {
    pub fn from(name: String, checked: bool) -> Self {
        Self { name, checked }
    }
}
