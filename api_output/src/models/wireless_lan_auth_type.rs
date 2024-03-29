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
pub struct WirelessLanAuthType {
    /// * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl WirelessLanAuthType {
    pub fn new() -> WirelessLanAuthType {
        WirelessLanAuthType {
            value: None,
            label: None,
        }
    }
}

/// * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "wep")]
    Wep,
    #[serde(rename = "wpa-personal")]
    WpaPersonal,
    #[serde(rename = "wpa-enterprise")]
    WpaEnterprise,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Open
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "WEP")]
    Wep,
    #[serde(rename = "WPA Personal (PSK)")]
    WpaPersonalLeftParenthesisPskRightParenthesis,
    #[serde(rename = "WPA Enterprise")]
    WpaEnterprise,
}

impl Default for Label {
    fn default() -> Label {
        Self::Open
    }
}

