/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableCircuitTerminationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableCircuitTerminationRequest {
    #[serde(rename = "circuit")]
    pub circuit: i32,
    /// * `A` - A * `Z` - Z
    #[serde(rename = "term_side")]
    pub term_side: TermSide,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<i32>>,
    #[serde(rename = "provider_network", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_network: Option<Option<i32>>,
    /// Physical circuit speed
    #[serde(rename = "port_speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<Option<i32>>,
    /// Upstream speed, if different from port speed
    #[serde(rename = "upstream_speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<Option<i32>>,
    /// ID of the local cross-connect
    #[serde(rename = "xconnect_id", skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
    /// Patch panel ID and port number(s)
    #[serde(rename = "pp_info", skip_serializing_if = "Option::is_none")]
    pub pp_info: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableCircuitTerminationRequest {
    /// Adds support for custom fields and tags.
    pub fn new(circuit: i32, term_side: TermSide) -> WritableCircuitTerminationRequest {
        WritableCircuitTerminationRequest {
            circuit,
            term_side,
            site: None,
            provider_network: None,
            port_speed: None,
            upstream_speed: None,
            xconnect_id: None,
            pp_info: None,
            description: None,
            mark_connected: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `A` - A * `Z` - Z
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TermSide {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "Z")]
    Z,
}

impl Default for TermSide {
    fn default() -> TermSide {
        Self::A
    }
}

