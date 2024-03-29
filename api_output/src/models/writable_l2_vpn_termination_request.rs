/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableL2VpnTerminationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableL2VpnTerminationRequest {
    #[serde(rename = "l2vpn")]
    pub l2vpn: i32,
    #[serde(rename = "assigned_object_type")]
    pub assigned_object_type: String,
    #[serde(rename = "assigned_object_id")]
    pub assigned_object_id: i64,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableL2VpnTerminationRequest {
    /// Adds support for custom fields and tags.
    pub fn new(l2vpn: i32, assigned_object_type: String, assigned_object_id: i64) -> WritableL2VpnTerminationRequest {
        WritableL2VpnTerminationRequest {
            l2vpn,
            assigned_object_type,
            assigned_object_id,
            tags: None,
            custom_fields: None,
        }
    }
}


