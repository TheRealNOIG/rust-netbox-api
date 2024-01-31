/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.7.1 (3.7)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContactGroup : Extends PrimaryModelSerializer to include MPTT support.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactGroup {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<Box<crate::models::NestedContactGroup>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "contact_count")]
    pub contact_count: i32,
    #[serde(rename = "_depth")]
    pub _depth: i32,
}

impl ContactGroup {
    /// Extends PrimaryModelSerializer to include MPTT support.
    pub fn new(id: i32, url: String, display: String, name: String, slug: String, created: Option<String>, last_updated: Option<String>, contact_count: i32, _depth: i32) -> ContactGroup {
        ContactGroup {
            id,
            url,
            display,
            name,
            slug,
            parent: None,
            description: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            contact_count,
            _depth,
        }
    }
}

