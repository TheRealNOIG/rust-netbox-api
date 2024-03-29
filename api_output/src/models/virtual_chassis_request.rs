/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VirtualChassisRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualChassisRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "master", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub master: Option<Option<Box<crate::models::NestedDeviceRequest>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl VirtualChassisRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String) -> VirtualChassisRequest {
        VirtualChassisRequest {
            name,
            domain: None,
            master: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}


