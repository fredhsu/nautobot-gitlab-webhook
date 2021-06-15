use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookRequest {
    pub event: String,
    pub timestamp: String,
    pub model: String,
    pub username: String,
    pub request_id: String,
    pub data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "model")]
pub enum Data {
    #[serde(rename = "ipaddress")]
    Ipaddress { id: String },
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceObject {
    pub id: String,
    pub url: String,
    pub device: Device,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Device {
    pub id: String,
    pub url: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query {
    pub query: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Nautobot {
    pub hostname: String,
    pub url: String,
    pub token: String,
    pub allow_insecure: bool,
}

impl Nautobot {
    pub fn query(
        &self,
        q: Query,
    ) -> std::result::Result<reqwest::blocking::Response, Box<dyn Error>> {
        let j = serde_json::to_string(&q).unwrap(); // setup match and return err
        println!("query json: {}", j);
        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(self.allow_insecure)
            .build()
            .unwrap();
        let res = client
            .post(&self.url)
            .header(header::AUTHORIZATION, "Token ".to_owned() + &self.token)
            .json(&q)
            .send()?;
        Ok(res)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IPAddress {
    pub id: String,
    pub url: String,
    pub family: Family,
    pub address: String,
    pub vrf: String,
    pub tenant: String,
    pub status: String,
    pub assigned_object: DeviceObject,
    pub tags: Vec<NestedTag>,
    pub hostname: String,
    pub token: String,
    pub allow_insecure: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NestedTag {
    id: String,
    url: String,
    name: String,
    slug: String,
    color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Family {
    label: String,
    value: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_query() {
        let nb = Nautobot {
            hostname: "nautobot".to_string(),
            token: "f6df868dfa674ff1d5fdfaac169eda87a55d2d93".to_string(),
            url: "https://nautobot/api/graphql/".to_string(),
            allow_insecure: true,
        };
        let query = Query {
            query: r"query { ip_addresses { address dns_name}}".to_string(),
        };
        let result = nb.query(query);
        match result {
            Ok(r) => assert_eq!(r.status(), 200),
            Err(e) => assert!(false, "Failed query: {}", e),
        }
    }
}
