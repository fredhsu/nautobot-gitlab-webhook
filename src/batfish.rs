use serde::{Deserialize, Serialize};
#[path = "avd.rs"]
mod avd;
use avd::StandardACL;
#[path = "nautobot.rs"]
mod nautobot;
use nautobot::IpAddressType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BatfishPolicy {
    permit: Vec<BatfishEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatfishEntry {
    name: String,
    dst_ips: String,
    ip_protocols: Vec<String>,
    dst_ports: Vec<String>,
}

pub fn permit_from_ips(ips: &Vec<crate::nautobot::IpAddressType>) -> BatfishPolicy {
    let mut bfes = Vec::new();
    for ip in ips {
        let bfe = BatfishEntry {
            name: "".to_owned(),
            dst_ips: ip.address.clone(),
            ip_protocols: vec!["tcp".to_owned()],
            dst_ports: Vec::new(),
        };
        bfes.push(bfe);
    }
    BatfishPolicy { permit: bfes }
}
impl BatfishPolicy {
    // pub fn to_avd() -> StandardACL {}
}
/*
pub fn from_avd(sacls: StandardACLs) -> BatfishPolicy {
    let mut bfEntries = Vec::new();
    for (key, val) in sacls.standard_access_lists.iter() {
        for (seq, entry) in val.sequence_numbers.iter() {

        }
    }
    let bfPolicy = BatfishPolicy { permit: bfEntries };
}
*/
