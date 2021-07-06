use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::error::Error;
use std::str;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookRequest {
    pub event: String,
    pub timestamp: String,
    pub model: String,
    pub username: String,
    pub request_id: String,
    // pub data: IPAddress,
    pub data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(tag = "model")]
#[serde(untagged)]
pub enum Data {
    #[serde(rename = "ipaddress")]
    Ipaddress(IPAddress), // Ipaddress { id: String, address: String },
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GqlData {
    pub data: IpAddresses,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IpAddresses {
    pub ip_addresses: Vec<IpAddressType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IpAddressType {
    pub address: String,
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
        let mut res = client
            .post(&self.url)
            .header(header::AUTHORIZATION, "Token ".to_owned() + &self.token)
            .json(&q)
            .send()?;
        // let mut buf: Vec<u8> = vec![];
        // res.copy_to(&mut buf)?;
        // println!("response: {:?}", str::from_utf8(&buf)?);
        Ok(res)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IPAddress {
    pub id: String,
    pub url: String,
    pub family: Family,
    pub address: String,
    // pub vrf: String,
    // pub tenant: String,
    // pub status: String,
    pub assigned_object: Option<DeviceObject>,
    pub tags: Vec<NestedTag>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NestedTag {
    pub id: String,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Family {
    pub label: String,
    pub value: u8,
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
            query: r#"query { ip_addresses(tag:"critical") { address }}"#.to_string(),
        };
        let result = nb.query(query);
        match result {
            Ok(r) => {
                assert_eq!(r.status(), 200);
                println!("{}", &r.text().unwrap());
            }
            Err(e) => assert!(false, "Failed query: {}", e),
        }
    }
}
