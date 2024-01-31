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
pub struct InterfaceTemplateRfRole {
    /// * `ap` - Access point * `station` - Station
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl InterfaceTemplateRfRole {
    pub fn new() -> InterfaceTemplateRfRole {
        InterfaceTemplateRfRole {
            value: None,
            label: None,
        }
    }
}

/// * `ap` - Access point * `station` - Station
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "ap")]
    Ap,
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Ap
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Access point")]
    AccessPoint,
    #[serde(rename = "Station")]
    Station,
}

impl Default for Label {
    fn default() -> Label {
        Self::AccessPoint
    }
}

