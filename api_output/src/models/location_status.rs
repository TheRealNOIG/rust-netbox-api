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
pub struct LocationStatus {
    /// * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `retired` - Retired
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl LocationStatus {
    pub fn new() -> LocationStatus {
        LocationStatus {
            value: None,
            label: None,
        }
    }
}

/// * `planned` - Planned * `staging` - Staging * `active` - Active * `decommissioning` - Decommissioning * `retired` - Retired
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "decommissioning")]
    Decommissioning,
    #[serde(rename = "retired")]
    Retired,
}

impl Default for Value {
    fn default() -> Value {
        Self::Planned
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "Staging")]
    Staging,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Decommissioning")]
    Decommissioning,
    #[serde(rename = "Retired")]
    Retired,
}

impl Default for Label {
    fn default() -> Label {
        Self::Planned
    }
}

