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
pub struct WirelessLanAuthCipher {
    /// * `auto` - Auto * `tkip` - TKIP * `aes` - AES
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl WirelessLanAuthCipher {
    pub fn new() -> WirelessLanAuthCipher {
        WirelessLanAuthCipher {
            value: None,
            label: None,
        }
    }
}

/// * `auto` - Auto * `tkip` - TKIP * `aes` - AES
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "tkip")]
    Tkip,
    #[serde(rename = "aes")]
    Aes,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Auto
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "TKIP")]
    Tkip,
    #[serde(rename = "AES")]
    Aes,
}

impl Default for Label {
    fn default() -> Label {
        Self::Auto
    }
}

