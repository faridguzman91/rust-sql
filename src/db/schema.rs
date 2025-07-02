use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: HashMap<usize, Row>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Row {
    data: HashMap<String, String>,
}
