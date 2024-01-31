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
pub struct ContactAssignmentPriority {
    /// * `primary` - Primary * `secondary` - Secondary * `tertiary` - Tertiary * `inactive` - Inactive
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl ContactAssignmentPriority {
    pub fn new() -> ContactAssignmentPriority {
        ContactAssignmentPriority {
            value: None,
            label: None,
        }
    }
}

/// * `primary` - Primary * `secondary` - Secondary * `tertiary` - Tertiary * `inactive` - Inactive
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "tertiary")]
    Tertiary,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Primary
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Primary")]
    Primary,
    #[serde(rename = "Secondary")]
    Secondary,
    #[serde(rename = "Tertiary")]
    Tertiary,
    #[serde(rename = "Inactive")]
    Inactive,
}

impl Default for Label {
    fn default() -> Label {
        Self::Primary
    }
}

