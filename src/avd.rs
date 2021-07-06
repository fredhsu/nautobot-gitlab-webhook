use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct StandardACLs {
    pub standard_access_lists: HashMap<String, StandardACL>,
}

#[derive(Deserialize, Serialize)]
pub struct StandardACL {
    pub sequence_numbers: HashMap<i32, AccessListEntry>,
}

#[derive(Deserialize, Serialize)]
pub struct AccessListEntry {
    pub action: String,
}
