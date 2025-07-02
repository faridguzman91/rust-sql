use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Table {
    columns: Vec<String>,
    rows: HashMap<usize, Row>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct Row {
    data: HashMap<String, String>,
}
