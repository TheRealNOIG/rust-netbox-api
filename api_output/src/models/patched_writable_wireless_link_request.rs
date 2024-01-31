/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableWirelessLinkRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableWirelessLinkRequest {
    #[serde(rename = "interface_a", skip_serializing_if = "Option::is_none")]
    pub interface_a: Option<i32>,
    #[serde(rename = "interface_b", skip_serializing_if = "Option::is_none")]
    pub interface_b: Option<i32>,
    #[serde(rename = "ssid", skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// * `connected` - Connected * `planned` - Planned * `decommissioning` - Decommissioning
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    /// * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    /// * `auto` - Auto * `tkip` - TKIP * `aes` - AES
    #[serde(rename = "auth_cipher", skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<AuthCipher>,
    #[serde(rename = "auth_psk", skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableWirelessLinkRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableWirelessLinkRequest {
        PatchedWritableWirelessLinkRequest {
            interface_a: None,
            interface_b: None,
            ssid: None,
            status: None,
            tenant: None,
            auth_type: None,
            auth_cipher: None,
            auth_psk: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `connected` - Connected * `planned` - Planned * `decommissioning` - Decommissioning
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Connected
    }
}
/// * `open` - Open * `wep` - WEP * `wpa-personal` - WPA Personal (PSK) * `wpa-enterprise` - WPA Enterprise
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
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

impl Default for AuthType {
    fn default() -> AuthType {
        Self::Open
    }
}
/// * `auto` - Auto * `tkip` - TKIP * `aes` - AES
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthCipher {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "tkip")]
    Tkip,
    #[serde(rename = "aes")]
    Aes,
    #[serde(rename = "")]
    Empty,
}

impl Default for AuthCipher {
    fn default() -> AuthCipher {
        Self::Auto
    }
}

