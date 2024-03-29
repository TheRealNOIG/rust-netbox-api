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
pub struct CustomFieldUiVisible {
    /// * `always` - Always * `if-set` - If set * `hidden` - Hidden
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl CustomFieldUiVisible {
    pub fn new() -> CustomFieldUiVisible {
        CustomFieldUiVisible {
            value: None,
            label: None,
        }
    }
}

/// * `always` - Always * `if-set` - If set * `hidden` - Hidden
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "if-set")]
    IfSet,
    #[serde(rename = "hidden")]
    Hidden,
}

impl Default for Value {
    fn default() -> Value {
        Self::Always
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "If set")]
    IfSet,
    #[serde(rename = "Hidden")]
    Hidden,
}

impl Default for Label {
    fn default() -> Label {
        Self::Always
    }
}

