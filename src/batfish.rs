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
    ip_protocols: String,
    dst_ports: Vec<String>,
}
