/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableL2VpnTerminationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableL2VpnTerminationRequest {
    #[serde(rename = "l2vpn", skip_serializing_if = "Option::is_none")]
    pub l2vpn: Option<i32>,
    #[serde(rename = "assigned_object_type", skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<String>,
    #[serde(rename = "assigned_object_id", skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<i64>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableL2VpnTerminationRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableL2VpnTerminationRequest {
        PatchedWritableL2VpnTerminationRequest {
            l2vpn: None,
            assigned_object_type: None,
            assigned_object_id: None,
            tags: None,
            custom_fields: None,
        }
    }
}


