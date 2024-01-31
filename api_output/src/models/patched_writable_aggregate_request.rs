/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableAggregateRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableAggregateRequest {
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Regional Internet Registry responsible for this IP space
    #[serde(rename = "rir", skip_serializing_if = "Option::is_none")]
    pub rir: Option<i32>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "date_added", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<Option<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableAggregateRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableAggregateRequest {
        PatchedWritableAggregateRequest {
            prefix: None,
            rir: None,
            tenant: None,
            date_added: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}


