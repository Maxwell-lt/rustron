use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IntegrationReport {
    #[serde(rename = "LIPIdList")]
    id_list: IdList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdList {
    #[serde(rename = "Devices")]
    devices: Vec<Device>,
    #[serde(rename = "Zones")]
    zones: Vec<Zone>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ID")]
    id: u16, // Pretty sure IDs can only go up to 100 with the Smart Bridge, but other hubs might go higher
    #[serde(rename = "Area")]
    area: Option<DeviceName>, // Excluded on the hub device
    #[serde(rename = "Buttons")]
    buttons: Vec<Button>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Zone {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ID")]
    id: u16, // Pretty sure IDs can only go up to 100 with the Smart Bridge, but other hubs might go higher
    #[serde(rename = "Area")]
    area: DeviceName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceName {
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    #[serde(rename = "Number")]
    id: u8, // Really doubt that there are any devices with more than 100 buttons, 0-127 should be fine here
    #[serde(rename = "Name")]
    name: Option<String>, // Used for the Scenes represented by virtual buttons on the hub itself
}
