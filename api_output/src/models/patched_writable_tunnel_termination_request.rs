/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableTunnelTerminationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableTunnelTerminationRequest {
    #[serde(rename = "tunnel", skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<i32>,
    /// * `peer` - Peer * `hub` - Hub * `spoke` - Spoke
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "termination_type", skip_serializing_if = "Option::is_none")]
    pub termination_type: Option<String>,
    #[serde(rename = "termination_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub termination_id: Option<Option<i64>>,
    #[serde(rename = "outside_ip", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outside_ip: Option<Option<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableTunnelTerminationRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableTunnelTerminationRequest {
        PatchedWritableTunnelTerminationRequest {
            tunnel: None,
            role: None,
            termination_type: None,
            termination_id: None,
            outside_ip: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `peer` - Peer * `hub` - Hub * `spoke` - Spoke
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "peer")]
    Peer,
    #[serde(rename = "hub")]
    Hub,
    #[serde(rename = "spoke")]
    Spoke,
}

impl Default for Role {
    fn default() -> Role {
        Self::Peer
    }
}

