use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// #[path = "nautobot.rs"]
// mod nautobot;
// use nautobot::IpAddressType;

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

pub fn permit_from_ips(ips: &Vec<crate::nautobot::IpAddressType>) -> StandardACLs {
    let mut sacls = HashMap::new();
    let mut seqn = HashMap::new();
    for (i, ip) in ips.iter().enumerate() {
        let action = format!("permit ip any {}", ip.address);
        let ale = AccessListEntry { action: action };
        seqn.insert((i as i32 + 1) * 10, ale);
    }
    let sacl = StandardACL {
        sequence_numbers: seqn,
    };
    sacls.insert("critical".to_owned(), sacl);
    StandardACLs {
        standard_access_lists: sacls,
    }
}
