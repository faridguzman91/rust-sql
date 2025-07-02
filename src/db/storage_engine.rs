use crate::db::schema::{Row, Table};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]

pub struct StorageEngine {
    tables: HashMap<String, Table>,
}

impl StorageEngine {
    pub fn new() -> Self {
        StorageEngine {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) {
        self.tables.insert(
            name.to_string(),
            Table {
                columns,
                rows: HashMap::new(),
            },
        );
    }

    pub fn insert_row() {}

    pub fn serialize() {}

    pub fn deserialize() {}
}
