/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProviderAccountRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderAccountRequest {
    #[serde(rename = "provider")]
    pub provider: Box<crate::models::NestedProviderRequest>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ProviderAccountRequest {
    /// Adds support for custom fields and tags.
    pub fn new(provider: crate::models::NestedProviderRequest, account: String) -> ProviderAccountRequest {
        ProviderAccountRequest {
            provider: Box::new(provider),
            name: None,
            account,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}


