use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookRequest {
    pub event: String,
    pub timestamp: String,
    pub model: String,
    pub username: String,
    pub request_id: String,
    pub data: Data,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Data {
    pub id: String,
    pub url: String,
    pub address: String,
    pub assigned_object: DeviceObject,
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
