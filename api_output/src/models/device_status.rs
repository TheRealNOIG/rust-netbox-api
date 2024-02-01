/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceStatus {
    /// * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `inventory` - Inventory * `decommissioning` - Decommissioning
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl DeviceStatus {
    pub fn new() -> DeviceStatus {
        DeviceStatus {
            value: None,
            label: None,
        }
    }
}

/// * `offline` - Offline * `active` - Active * `planned` - Planned * `staged` - Staged * `failed` - Failed * `inventory` - Inventory * `decommissioning` - Decommissioning
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staged")]
    Staged,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Value {
    fn default() -> Value {
        Self::Offline
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Offline")]
    Offline,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "Staged")]
    Staged,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Inventory")]
    Inventory,
    #[serde(rename = "Decommissioning")]
    Decommissioning,
}

impl Default for Label {
    fn default() -> Label {
        Self::Offline
    }
}
