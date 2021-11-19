use serde::{Deserialize, Serialize};

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

pub fn permit_from_ips(ips: &[crate::nautobot::IpAddressType]) -> BatfishPolicy {
    let mut bfes = Vec::new();
    for ip in ips {
        let bfe = BatfishEntry {
            name: String::from(""),
            dst_ips: ip.address.clone(),
            ip_protocols: vec![String::from("tcp")],
            dst_ports: Vec::new(),
        };
        bfes.push(bfe);
    }
    BatfishPolicy { permit: bfes }
}
